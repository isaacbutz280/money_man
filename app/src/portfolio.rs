use crate::{
    envelope,
    misc::{dollar, transaction},
};
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::{error, io::ErrorKind};

/**
 * A portfolio is a collection of Vopes
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    budgeted: dollar::Dollar,           // Amount of paycheck budgeted
    holdings: dollar::Dollar,           // Total money in account
    envelopes: Vec<envelope::Envelope>, // Our vopes
    // there are two special envelopes: ignored and deferred. The ignored is for transactions that
    // I don't want to ever process - this prevents them from appearing again.
    // The deferred envelope is for transactions I don't want to process at this time.
    ignored: Box<envelope::Envelope>,
    deferred: Box<envelope::Envelope>,
}

impl Portfolio {
    // This is public only so it can be called by the account. Never expect user to create an
    // account
    pub(crate) fn new() -> Portfolio {
        let ignored = Box::new(envelope::Envelope::new(
            "Ignored".to_owned(),
            dollar::Dollar::default(),
        ));
        let deferred = Box::new(envelope::Envelope::new(
            "deferred".to_owned(),
            dollar::Dollar::default(),
        ));

        Self {
            envelopes: vec![],
            budgeted: dollar::Dollar::default(),
            holdings: dollar::Dollar::default(),
            ignored,
            deferred,
        }
    }

    // Getters
    pub fn get_envelopes(&self) -> &Vec<envelope::Envelope> {
        &self.envelopes
    }

    pub fn get_envelope_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut envelope::Envelope, Box<dyn error::Error>> {
        self.envelopes
            .iter_mut()
            .find(|e| e.name == name)
            .ok_or_else(|| {
                Box::new(std::io::Error::from(ErrorKind::NotFound)) as Box<dyn error::Error>
            })
    }

    pub fn get_holdings(&self) -> dollar::Dollar {
        self.holdings
    }

    pub fn view_budgeted(&self) -> dollar::Dollar {
        self.budgeted
    }

    pub fn add_envelope(
        &mut self,
        name: &str,
        budget: dollar::Dollar,
    ) -> Result<(), Box<dyn error::Error>> {
        if self.contains(&name) {
            // Duplicate name
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput)))
        } else {
            // Allowed
            self.envelopes
                .push(envelope::Envelope::new(name.to_string(), budget));
            self.budgeted += budget;

            Ok(())
        }
    }

    pub fn remove_envelope(
        &mut self,
        envelope: &envelope::Envelope,
    ) -> Result<(), Box<dyn error::Error>> {
        todo!()

        // Use Retain here
    }

    pub fn transfer(
        &mut self,
        src: &mut envelope::Envelope,
        dest: &mut envelope::Envelope,
        amount: dollar::Dollar,
    ) -> Result<(), Box<dyn error::Error>> {
        // Create two transactions. A negative and a positive.
        let today: NaiveDate = Local::now().date_naive();
        let desc = format!(
            "Transfer from {} to {}",
            src.name.as_str(),
            dest.name.as_str()
        );
        let neg_trans = transaction::Transaction::new(today, desc.clone(), amount * -1.0);
        let pos_trans = transaction::Transaction::new(today, desc, amount);

        src.add_transaction(neg_trans);
        dest.add_transaction(pos_trans);

        Ok(())
    }

    /// Given a set of Transactions, returns a list removing all duplicates
    pub fn get_cleaned_transaction_list(&self, list: &mut Vec<transaction::Transaction>) {
        // Remove any dupicates in the input
        list.sort();
        list.dedup();

        // Remove any duplicates from the list
        for v in self.envelopes.iter() {
            list.retain(|t| v.transactions.contains(&t) == false);
        }

        // Remove any ignored duplicates
        list.retain(|t| self.ignored.transactions.contains(&t) == false);
    }

    /// Given a transaction, and a list of envelopes, assigns the transactions
    ///
    ///
    pub fn assign_transaction_weighted(
        &mut self,
        envelopes: Vec<&mut envelope::Envelope>,
        trans: transaction::Transaction,
        note: Option<String>,
    ) -> Result<(), Box<dyn error::Error>> {
        // Get total of all budgeted. This enables fractional assignement
        let total_weight = envelopes
            .iter()
            .fold(0.0, |acc, &x| acc + x.budget.as_f64());
        let mut sum = 0.0;

        for &mut e in envelopes.iter_mut() {
            let weight = e.budget.as_f64() / total_weight;
            let partial = trans.charge.as_f64() * weight;
            sum += partial;

            // Craft new transaction with the partial
            let mut t = trans.clone();
            t.note = note.clone();
            t.partial_charge = Some(dollar::Dollar::from(partial));

            e.add_transaction(t);

            let delta = sum - trans.charge;

            // For now - if I have a rounding error, log it
            log::debug!("Delta missing! {}", delta);
        }

        // For now, ignore the float part, just assume equal sharing. Hey dummy!
        self.calc_holdings();
        Ok(())
    }

    pub fn assign_transactions_even(
        &mut self,
        envelopes: Vec<&mut envelope::Envelope>,
        trans: transaction::Transaction,
    ) -> Result<(), Box<dyn error::Error>> {
        todo!();

        // For now - if I have a rounding error, log it
        // log::debug!("Delta missing! {}", delta);

        // v.actual_amount += deposit;
        // v.transactions.push(trans.clone());
    }

    pub fn assign_transaction_custom(
        &mut self,
        envelopes: Vec<&mut envelope::Envelope>,
        trans: transaction::Transaction,
    ) -> Result<(), Box<dyn error::Error>> {
        todo!()
    }

    pub fn ignore_transaction(
        &mut self,
        trans: transaction::Transaction,
    ) -> Result<(), Box<dyn error::Error>> {
        todo!()
    }

    pub fn defer_transaction(
        &mut self,
        trans: transaction::Transaction,
    ) -> Result<(), Box<dyn error::Error>> {
        todo!()
    }

    // Helper functions

    /*
     * Checks for if the vope exists
     */
    fn contains(&self, name: &str) -> bool {
        name.eq_ignore_ascii_case(&self.ignored.name)
            || self
                .envelopes
                .iter()
                .any(|v| v.name.eq_ignore_ascii_case(name))
    }

    pub(crate) fn calc_holdings(&mut self) {
        self.holdings = dollar::Dollar::default();
        self.budgeted = dollar::Dollar::default();

        for v in self.envelopes.iter_mut() {
            self.holdings += v.actual_amount;
            self.budgeted += v.budget;
        }
    }
}

impl ToString for Portfolio {
    fn to_string(&self) -> String {
        let mut s = String::new();

        s.push_str(&format!(
            "Budgeted: {}\n\
                Holdings: {}\n",
            self.budgeted, self.holdings
        ));

        for v in &self.envelopes {
            let temp = format!("  {} | {} | {}\n", v.name, v.actual_amount, v.budget);
            s.push_str(&temp);
        }

        s
    }
}
