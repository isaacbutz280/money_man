pub mod misc;
pub mod portfolio;
pub mod vope;

use portfolio::Portfolio;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::PathBuf,
    str::FromStr,
    vec,
};

// Re-exports
pub use misc::Transaction;

/**
 * An account is the highest level. Contains a portfolio + metadata
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub date: String,
    pub path: PathBuf,
    pub port: Portfolio,
    // This is a collection of transactions. Only need a HashSet, but that isn't easily serde...
    transactions: HashMap<misc::Transaction, bool>,
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

impl Account {
    fn new() -> Account {
        Self {
            name: "Isaac".to_string(),
            date: "Yesterday".to_string(),
            path: PathBuf::from_str("C:\\Users\\Isaac\\Git_Repos\\money_man\\app\\acc\\isaac.json")
                .unwrap(),
            port: Portfolio::new(),
            transactions: HashMap::default(),
        }
    }

    /**
     * Builds a new account from a JSON file.
     */
    pub fn from(pb: PathBuf) -> Result<Account, Box<dyn Error>> {
        // Parse the JSON to a string
        let raw_json = fs::read_to_string(&pb).unwrap_or_default();

        // Parse the string to an account object
        let res_acc: Result<Account, serde_json::Error> = serde_json::from_str(&raw_json);

        // Return result, printing error in case of failure
        match res_acc {
            Ok(mut acc) => {
                acc.path = pb;
                acc.port.re_calc();
                acc.save()?;
                Ok(acc)
            }
            Err(e) => {
                println!("Failed to open Account: {}", &e);
                Err(Box::new(e))
            }
        }
    }

    // For assign

    pub fn get_uncatorgorized(&self) -> Vec<misc::Transaction> {
        // Get transactions from file
        let transactions = match self.get_trans() {
            Ok(t) => t,
            Err(_) => vec![],
        };

        // Return all transactions we haven't already catorgorized
        transactions
            .iter()
            .filter_map(|t| {
                if self.transactions.contains_key(t) {
                    None
                } else {
                    Some(t.clone())
                }
            })
            .collect()
    }

    // Given name of Vope, transaction, updates the appropraite vope
    pub fn update_vope(
        &mut self,
        name: &str,
        transaction: &misc::Transaction,
    ) -> Result<(), Box<dyn Error>> {
        self.transactions.insert(transaction.clone(), false);
        if name == "Salary" {
            self.update_salary(transaction)
        } else if name != "Ignore" {
            self.port.update(name, transaction.charge)?;
            self.save()
        }
        else {
           Ok(()) 
        }
    }

    // For vope management

    pub fn get_vope_history(&self) -> Vec<misc::Transaction> {
        vec![]
    }

    /**
     * Account::add_vope() creates a new Vope for this account and adds it to the Account
     *
     * Returns io::Error::InavalidData if account already exists
     */
    pub fn add_vope(&mut self, name: &str, budget: misc::Dollar) -> Result<(), Box<dyn Error>> {
        // Attempt to add vope
        self.port.add(name, budget)?;

        // Attempt to save
        self.save()
    }

    /**
     * Account::rm_vope() will delete the requested vope. All remaining balance will go into the
     * dafualt account.
     *
     * Will return an error if account does not exist, or if account is the default
     * (note, to delete the current default account, first change default account)
     */
    pub fn rm_vope(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.port.remove(name)?;

        self.save()
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: misc::Dollar) -> Result<(), Box<dyn Error>> {
        // Complete transfer
        self.port.transfer(from, to, amount)?;

        // Save account information
        self.save()
    }

    // Helper

    // Adds all the transactions in - for now, we are parsing the CSV's
    fn get_trans(&self) -> Result<Vec<misc::Transaction>, Box<dyn Error>> {
        let mut trans = vec![];

        let base = self.path.parent().unwrap();
        let csv = "/../csv/";
        let dir = PathBuf::from(format!("{}{}", base.to_str().unwrap(), csv));

        // For each csv in dir
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let name = entry.file_name().to_str().unwrap().to_string();

            // determine source
            let columns = if name.eq_ignore_ascii_case("discover.csv") {
                [0, 2, 3]
            } else if name.eq_ignore_ascii_case("chase.csv") {
                [0, 2, 5]
            } else {
                [1, 3, 4]
            };

            // Use csv reader to disect file
            let mut rdr = csv::Reader::from_path(entry.path())?;

            // For line in csv...
            for result in rdr.records() {
                let record = result?;
                let date = record[columns[0]].to_owned();
                let descrr = record[columns[1]].to_owned();

                let mut amount;

                match Account::money_to_float(&record[columns[2]]) {
                    Ok(okay) => amount = okay,
                    Err(_) => amount = 0.0,
                };

                if name.eq_ignore_ascii_case("discover.csv") {
                    amount *= -1.0
                } else if name.eq_ignore_ascii_case("apg.csv") {
                    if 0.0 == amount {
                        amount = Account::money_to_float(&record[columns[2] + 1]).unwrap();
                    } else {
                        amount *= -1.0;
                    }
                }

                trans.push(misc::Transaction::new(
                    chrono::NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap(),
                    descrr,
                    misc::Dollar::from(amount),
                ));
            }
        }

        Ok(trans)
    }

    fn update_salary(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        self.port.cash_paycheck(transaction.charge)
    }

    /**
     * Writes the account to disk
     */
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let js = serde_json::to_string(&self)?;
        fs::write(&self.path, js)?;

        Ok(())
    }

    fn money_to_float(s: &str) -> Result<f32, std::num::ParseFloatError> {
        let mut rv = String::new();
        for c in s.chars() {
            if c != '$' && c != ' ' && c != '+' && c != ',' {
                rv.push(c);
            }
        }
    
        println!("{rv}");
    
        rv.parse()
    }
}
