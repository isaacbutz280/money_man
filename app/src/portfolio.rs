use crate::{misc::{self, Dollar}, vope, Transaction};
use serde::{Deserialize, Serialize};
use std::{error::Error, io::ErrorKind};

/**
 * A portfolio is a collection of Vopes
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Portfolio {
    default: (String, misc::Dollar), // Default vopes (name, budget)

    pub accounts: Vec<vope::Vope>,     // Our vopes
    pub salary: vope::Vope,
    pub ignored: vope::Vope,
    pub paycheck: misc::Dollar,        // Amount in one paycheck
    pub budgeted: misc::Dollar,        // Amount of paycheck budgeted
    pub holdings: misc::Dollar,        // Total money in account
}

impl Default for Portfolio {
    fn default() -> Self {
        Self::new()
    }
}

impl Portfolio {
    pub fn new() -> Portfolio {
        Self {
            default: ("Safety".to_owned(), misc::Dollar::from(100.0)),
            accounts: vec![vope::Vope::new("Safety".to_owned(), misc::Dollar::from(100.0))],
            salary: vope::Vope::new("Salary".to_owned(), misc::Dollar::from(0.0)),
            ignored: vope::Vope::new("Ignored".to_owned(), misc::Dollar::from(0.0)),
            paycheck: misc::Dollar::from(2100.0),
            budgeted: misc::Dollar::from(100.0),
            holdings: misc::Dollar::from(0.0),
        }
    }

    /*
     * Checks for if a vope is contained
     */
    pub fn contains(&self, name: &str) -> bool {
        self.accounts
            .iter()
            .any(|v| v.name.eq_ignore_ascii_case(name))
    }

    pub fn transaction_exists(&self, trans: & misc::Transaction) -> bool {
        let mut flag = false;

        for v in self.accounts.iter() {
            if v.transactions.contains(trans) {
                flag = true;
            }
        }

        if self.salary.transactions.contains(trans) {
            flag = true;
        }

        if self.ignored.transactions.contains(trans) {
            flag = false;
        }

        flag
    }

    pub fn get_accounts(&mut self) -> &mut Vec<vope::Vope> {
        &mut self.accounts
    }

    pub fn add(&mut self, name: &str, amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        if self.contains(&name) {
            // Duplicate name
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput)))
        // } else if self.paycheck < (self.budgeted + amount) {
        //     // Over budget
        //     Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
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

    /* 
     * Updates a named vope with the transaction
     */
    pub fn update(&mut self, name: &str, trans: & misc::Transaction) -> Result<(), Box<dyn Error>> {
        let op_vope = self
            .accounts
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(name));

        if op_vope.is_some() {
            let v_up = op_vope.unwrap();
            v_up.actual_amount += trans.charge;
            v_up.transactions.push(trans.clone());
        } else if name == "Salary" {
           self.cash_paycheck(trans.charge)?;
           self.salary.transactions.push(trans.clone())
        } else if name == "Ignore" {
           self.ignored.transactions.push(trans.clone())   
        }else {
           return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        };

        self.calc_holdings();
        Ok(())
    }

    fn cash_paycheck(&mut self, amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        // Keep a running total of how much we add to even it out in the end
        let mut sum_added = Dollar::from(0.0);

        // A salary is divided by each
        for v in self.accounts.iter_mut() {
            if v.name == self.default.0 {
                // Ignore default - handle later
            } else {
                let percent_of_budget = v.budget / self.budgeted;
                let dollar_amount = amount * percent_of_budget;
                v.actual_amount += dollar_amount;
                sum_added += dollar_amount;

                // println!("name: {} | percent: {} | dollar_amount: {}", v.name, percent_of_budget, dollar_amount)
            }
        }

        // Update default
        let op_default = self
            .accounts
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(&self.default.0))
            .unwrap();

        let default_percent = op_default.budget / self.budgeted;
        // Make sure to incorperate the diff
        let default_added = amount * default_percent;
        sum_added += default_added;

        op_default.actual_amount += default_added + (amount - sum_added);

        // println!("Total amount added: {} | default percent {}", sum_added, default_percent);

        self.calc_holdings();

        Ok(())
    }

    fn calc_holdings(&mut self) {
        self.holdings = misc::Dollar::from(0.0);

        for v in self.accounts.iter_mut() {
            self.holdings += v.actual_amount;
        }
    }

    pub fn re_calc(&mut self) {
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
