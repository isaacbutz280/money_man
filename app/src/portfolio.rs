use crate::{misc, vope};
use serde::{Deserialize, Serialize};
use std::{error::Error, io::ErrorKind, collections::HashMap};

/**
 * A portfolio is a collection of envelopes
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    accounts:     Vec<vope::Vope>,     // The list of envelopes
    budgeted:     misc::Dollar,        // Amount in monthly budget
    holdings:     misc::Dollar,        // Total money in account
}

impl Default for Portfolio {
    fn default() -> Self {
        Self::new()
    }
}

impl Portfolio {

    /* Creates a new Portfolio
     * 
     */
    pub fn new() -> Portfolio {
        Self {
            default: ("Safety".to_owned(), misc::Dollar::from(100.0)),
            accounts: vec![vope::Vope::new("Safety".to_owned(), misc::Dollar::from(100.0))],
            budgeted: misc::Dollar::from(0.0),
            holdings: misc::Dollar::from(0.0),
        }
    }

    pub fn add(&mut self, name: &str, amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        if self.contains(&name) {
            // Duplicate name
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput)))
        } else {
            // Allowed
            self.accounts.push(vope::Vope::new(name.to_string(), amount));
            self.budgeted += amount;

            Ok(())
        }
    }

    pub fn remove(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        // Assume default exists. Probably have bigger problems otherwise
        let pos = self
            .accounts
            .iter()
            .position(|v| v.name.eq_ignore_ascii_case(name));

        match pos {
            Some(ind) => {
                let removed = self.accounts.remove(ind);
                self.budgeted -= removed.budget;
                self.accounts
                    .iter_mut()
                    .find(|default| default.name.eq_ignore_ascii_case(&self.default.0))
                    .unwrap()
                    .actual_amount += removed.actual_amount;
                Ok(())
            }
            None => Err(Box::new(std::io::Error::from(ErrorKind::InvalidData))),
        }
    }

    pub fn transfer(
        &mut self,
        from_name: &str,
        dest_name: &str,
        amount: misc::Dollar,
    ) -> Result<(), Box<dyn Error>> {
        let from_vope = self
            .accounts
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(from_name));

        if let Some(from) = from_vope {
            from.actual_amount -= amount;
        } else {
            return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)));
        }

        let dest_vope = self
            .accounts
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(dest_name));
        if let Some(dest) = dest_vope {
            dest.actual_amount += amount;
        } else {
            return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)));
        }

        Ok(())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.accounts
            .iter()
            .any(|v| v.name.eq_ignore_ascii_case(name))
    }

    pub fn get_accounts(&mut self) -> &mut Vec<vope::Vope> {
        &mut self.accounts
    }

    pub fn stringy(&self, verbose: bool) -> String {
        let mut s = String::new();

        if verbose {
            s.push_str("Some real verbose output");
        }

        s.push_str(&format!(
            "Paycheck: {}\n\
                                    Budgeted: {}\n\
                                    Holdings: {}\n\
                                    Default Vope: {}\n",
            self.paycheck, self.budgeted, self.holdings, self.default.0
        ));

        for v in &self.accounts {
            let temp = format!("  {} | {} | {}\n", v.name, v.actual_amount, v.budget);
            s.push_str(&temp);
        }

        s
    }

    pub fn update(&mut self, name: &str, amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        let op_vope = self
            .accounts
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(name));

        if let Some(vope) = op_vope {
            vope.actual_amount += amount;
            self.calc_holdings();
            Ok(())
        } else {
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        }
    }

    pub fn cash_paycheck(&mut self, mut amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        // To be salary, must be greater than all vopes - default
        if amount > (self.paycheck - self.default.1) {
            for v in self.accounts.iter_mut() {
                if v.name == self.default.0 {
                    // Ignore - default
                } else {
                    v.actual_amount += v.budget;
                    amount -= v.budget;
                }
            }

            // Update default
            let op_default = self
                .accounts
                .iter_mut()
                .find(|v| v.name.eq_ignore_ascii_case(&self.default.0))
                .unwrap();
            op_default.actual_amount += amount;

            self.calc_holdings();

            Ok(())
        } else {
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput)))
        }
    }

    // 
    fn calc_holdings(&mut self) {
        self.holdings = misc::Dollar::from(0.0);

        for v in self.accounts.iter_mut() {
            self.holdings += v.actual_amount;
        }
    }

    fn re_calc(&mut self) {
        let mut budgeted = misc::Dollar::from(0.0);
        let mut holdings = misc::Dollar::from(0.0);
        
        for v in self.accounts.iter() {
            budgeted += v.budget;
            holdings += v.actual_amount;
        }

        self.budgeted = budgeted;
        self.holdings = holdings;
    }



}
