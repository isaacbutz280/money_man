use crate::{dollar, transaction, vope};
use serde;
use std::{error, io::ErrorKind};

/**
 * A portfolio is a collection of Vopes
 */
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Portfolio {
    envelopes: Vec<vope::Vope>, // Our vopes
    ignored: vope::Vope,
    budgeted: dollar::Dollar, // Amount of paycheck budgeted
    holdings: dollar::Dollar, // Total money in account
}

impl Portfolio {
    // This is public only so it can be called by the account
    pub(crate) fn new() -> Portfolio {
        Self {
            envelopes: vec![],
            ignored: vope::Vope::new("Ignored".to_owned(), dollar::Dollar::default()),
            budgeted: dollar::Dollar::default(),
            holdings: dollar::Dollar::default(),
        }
    }

    // Getters
    pub fn view_vopes(&self) -> &Vec<vope::Vope> {
        &self.envelopes
    }

    pub fn view_holdings(&self) -> dollar::Dollar {
        self.holdings
    }

    pub fn view_budgeted(&self) -> dollar::Dollar {
        self.budgeted
    }

    /// Adds a new vope to the Portfolio
    ///
    /// Returns `Ok(())` on success.
    ///
    /// Returns `Err(InvalidInput)` if the vope name is a duplicate
    pub fn add_vope(
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
                .push(vope::Vope::new(name.to_string(), budget));
            self.budgeted += budget;

            Ok(())
        }
    }

    /// Removes a Vope from the Portfolio.
    ///
    /// Returns `Ok(())` on success.
    ///
    /// Returns `Err(InvalidInput)` if the vope name is a duplicate
    /// Returns `Err(InvalidData)` if the vope balance is not $0
    pub fn remove_vope(&mut self, name: &str) -> Result<(), Box<dyn error::Error>> {
        let pos = self.get_vope_pos(name);
        match pos {
            Some(ind) => {
                // Unwrap and remove are okay - we just checked for existance
                if self.envelopes.get(ind).unwrap().actual_amount == dollar::Dollar::from(0.0) {
                    self.envelopes.remove(ind);
                    Ok(())
                } else {
                    Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
                }
            }
            None => Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput))),
        }
    }

    pub fn transfer_holdings(
        &mut self,
        from_name: &str,
        dest_name: &str,
        amount: dollar::Dollar,
    ) -> Result<(), Box<dyn error::Error>> {
        let from_vope = self
            .envelopes
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(from_name));

        if let Some(from) = from_vope {
            from.actual_amount -= amount;
        } else {
            return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)));
        }

        let dest_vope = self
            .envelopes
            .iter_mut()
            .find(|v| v.name.eq_ignore_ascii_case(dest_name));
        if let Some(dest) = dest_vope {
            dest.actual_amount += amount;
        } else {
            return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)));
        }

        Ok(())
    }

    /// Given a set of Transactions, returns a list removing all duplicates
    pub fn clean_transaction_list(&self, list: &mut Vec<transaction::Transaction>) {
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

    /// Given a transaction, and a list of names/weights, distributes the
    pub fn assign_transaction(
        &mut self,
        names: &[(&str, f32)],
        trans: &transaction::Transaction,
        even_weight: bool,
    ) -> Result<(), Box<dyn error::Error>> {
        let mut total_weight = 0.0;

        // Before we start - verify all names, and sum the total weight
        for (n, w) in names {
            if self.contains(n) {
                total_weight += w;
            } else {
                return Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)));
            }
        }

        if even_weight {
            // Get percentage
            let weight = 1.0 / (names.len() as f32);
            let deposit = trans.charge * weight;
            let delta = deposit * (names.len() as f32) - trans.charge;

            // For now - if I have a rounding error, log it
            log::debug!("Delta missing! {}", delta);

            for (n, _w) in names {
                let v = if n.eq_ignore_ascii_case("ignore") {
                    &mut self.ignored
                } else {
                    //This unwrap should be safe, checked above
                    self.envelopes
                        .iter_mut()
                        .find(|v| v.name.eq_ignore_ascii_case(n))
                        .unwrap()
                };

                v.actual_amount += deposit;
                v.transactions.push(trans.clone());
            }
        } else {
            let mut sum = dollar::Dollar::default();

            for (name, w) in names {
                // Get individuals weight
                let weight = w / total_weight;
                let deposit = trans.charge * weight;
                sum += deposit;

                // Can unwrap cause I checked above
                let v = if name.eq_ignore_ascii_case("ignore") {
                    &mut self.ignored
                } else {
                    self.envelopes
                        .iter_mut()
                        .find(|x| &x.name.as_str() == name)
                        .unwrap()
                };

                v.actual_amount += deposit;
                v.transactions.push(trans.clone());
            }

            let delta = sum - trans.charge;

            // For now - if I have a rounding error, log it
            log::debug!("Delta missing! {}", delta);
        }

        // For now, ignore the float part, just assume equal sharing. Hey dummy!

        self.calc_holdings();
        Ok(())
    }

    ///
    ///
    pub fn get_vope_history(
        &self,
        name: &str,
    ) -> Result<Vec<transaction::Transaction>, Box<dyn error::Error>> {
        let op = self.envelopes.iter().find(|v| v.name == name);
        match op {
            Some(v) => Ok(v.transactions.clone()),
            None => Err(Box::new(std::io::Error::from(ErrorKind::InvalidInput))),
        }
    }

    pub fn sort_vope(&mut self) {
        todo!()
    }

    pub fn move_vope(&mut self) {
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

    // fn get_vope(&self, name: &str) -> Option<vope::Vope> {
    //     let pos = self
    //         .envelopes
    //         .iter()
    //         .position(|v| v.name.eq_ignore_ascii_case(name));

    //     self.envelopes.into_iter().find(|x| x.name == name)
    // }

    fn get_vope_pos(&self, name: &str) -> Option<usize> {
        todo!()
    }

    pub(crate) fn calc_holdings(&mut self) {
        self.holdings = dollar::Dollar::default();

        for v in self.envelopes.iter_mut() {
            self.holdings += v.actual_amount;
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

// fn re_calc(&mut self) {
//     let mut budgeted = misc::Dollar::from(0.0);
//     let mut holdings = misc::Dollar::from(0.0);

//     for v in self.accounts.iter() {
//         budgeted += v.budget;
//         holdings += v.actual_amount;
//     }

//     self.budgeted = budgeted;
//     self.holdings = holdings;
// }

// /// Given a path to a [properly formatted csv file](https://github.com/isaacbutz280/money_man#readme),
// /// parses the file to get all uncatorgorized transactions.
// ///
// fn get_uncatorgorized(&self, path: &Path) -> Vec<misc::Transaction> {
//     // Get transactions from file
//     let mut tr = match self.get_trans(path) {
//         Ok(t) => t,
//         Err(e) => vec![],
//     };

//     tr.into_iter()
//         .filter(|uncat_t| !self.port.transaction_exists(&uncat_t))
//         .collect()
// }

// // Given name of Vope, transaction, updates the appropraite vope
// fn update_vope(
//     &mut self,
//     name: &str,
//     transaction: &misc::Transaction,
// ) -> Result<(), Box<dyn Error>> {
//     self.port.update(name, transaction)?;

//     self.save()
// }

// /**
//  * Returns a list of Envelope history
//  */
// pub fn get_vope_history(&self, name: &str) -> Vec<misc::Transaction> {
//     let mut hist = vec![];

//     for v in self.port.accounts.iter() {
//         if name == v.name {
//             hist = v.transactions.clone();
//         }
//     }

//     hist
// }

// /**
//  * Account::add_vope() creates a new Vope for this account and adds it to the Account
//  *
//  * Returns io::Error::InavalidData if account already exists
//  */
// pub fn add_vope(&mut self, name: &str, budget: misc::Dollar) -> Result<(), Box<dyn Error>> {
//     // Attempt to add vope
//     self.port.add(name, budget)?;

//     // Attempt to save
//     self.save()
// }

// /**
//  * Account::rm_vope() will delete the requested vope. All remaining balance will go into the
//  * dafualt account.
//  *
//  * Will return an error if account does not exist, or if account is the default
//  * (note, to delete the current default account, first change default account)
//  */
// pub fn rm_vope(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
//     self.port.remove(name)?;

//     self.save()
// }

// pub fn transfer(
//     &mut self,
//     from: &str,
//     to: &str,
//     amount: misc::Dollar,
// ) -> Result<(), Box<dyn Error>> {
//     // Complete transfer
//     self.port.transfer(from, to, amount)?;

//     // Save account information
//     self.save()
// }

// // Helpers

// // Adds all the transactions in - for now, we are parsing the CSV's
// fn get_trans(&self, path: &Path) -> Result<Vec<misc::Transaction>, Box<dyn Error>> {
//     let mut trans = vec![];

//     let mut rdr = csv::Reader::from_path(path)?;

//     // For line in csv...
//     for result in rdr.records() {
//         // Get the record
//         let record = result?;

//         // csv should be of form:
//         // date(mm/dd/yyyy), description, amount
//         let date = record[0].to_owned();
//         let desc = record[1].to_owned();

//         let amount = match money_to_float(&record[2]) {
//             Ok(okay) => okay,
//             Err(_) => 0.0,
//         };

//         trans.push(misc::Transaction::new(
//             chrono::NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap(),
//             desc,
//             misc::Dollar::from(amount),
//         ));
//     }

//     Ok(trans)
// }

// fn cash_paycheck(&mut self, amount: dollar::Dollar) -> Result<(), Box<dyn error::Error>> {
//     // Keep a running total of how much we add to even it out in the end
//     let mut sum_added = dollar::Dollar::from(0.0);

//     // A salary is divided by each
//     for v in self.envelopes.iter_mut() {
//         if v.name == self.default.0 {
//             // Ignore default - handle later
//         } else {
//             let percent_of_budget = v.budget / self.budgeted;
//             let dollar_amount = amount * percent_of_budget;
//             v.actual_amount += dollar_amount;
//             sum_added += dollar_amount;

//             // println!("name: {} | percent: {} | dollar_amount: {}", v.name, percent_of_budget, dollar_amount)
//         }
//     }

//     // Update default
//     let op_default = self
//         .accounts
//         .iter_mut()
//         .find(|v| v.name.eq_ignore_ascii_case(&self.default.0))
//         .unwrap();

//     let default_percent = op_default.budget / self.budgeted;
//     // Make sure to incorperate the diff
//     let default_added = amount * default_percent;
//     sum_added += default_added;

//     op_default.actual_amount += default_added + (amount - sum_added);

//     // println!("Total amount added: {} | default percent {}", sum_added, default_percent);

//     self.calc_holdings();

//     Ok(())
// }
