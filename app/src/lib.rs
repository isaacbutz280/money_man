pub mod misc;
pub mod portfolio;
pub mod vope;

use directories::ProjectDirs;
use portfolio::Portfolio;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    vec,
};

// Re-exports
pub use misc::Transaction;

/**
 * An account contains all information about the user.
 *
 * The highest level of what the end user can see.
 */
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub date: String,
    pub path: PathBuf,
    pub port: Portfolio,
}

impl Account {
    pub fn new() -> Result<Account, Box<dyn Error>> {
        // Builds account from JSON, or creates new account if unavailable
        let binding = ProjectDirs::from("io", "ButzIndustries", "MoneyMan").unwrap();
        let path = Path::new(binding.data_dir()).join("acc.json");
        let file_read = fs::read_to_string(&path);

        match file_read {
            Ok(raw_json) => {
                let res_acc: Result<Account, serde_json::Error> = serde_json::from_str(&raw_json);

                match res_acc {
                    Ok(mut acc) => {
                        acc.port.re_calc();
                        acc.save()?;
                        println!("Acc reopened");
                        Ok(acc)
                    }
                    Err(e) => {
                        println!("Failed to open Account: {}", &e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(_) => {
                fs::create_dir_all(path.parent().unwrap())?;

                println!("Creating new account...");

                let acc = Account {
                    name: "Unknown".to_string(),
                    date: "Today".to_string(),
                    path: path.to_path_buf(),
                    port: Portfolio::new(),
                };

                println!("Acc created");

                acc.save()?;

                println!("Acc saved");

                Ok(acc)
            }
        }
    }

    /// Given a path to a [properly formatted csv file](https://github.com/isaacbutz280/money_man#readme),
    /// parses the file to get all uncatorgorized transactions.
    /// 
    pub fn get_uncatorgorized(&self, path: &Path) -> Vec<misc::Transaction> {
        // Get transactions from file
        let mut tr = match self.get_trans(path) {
            Ok(t) => t,
            Err(e) => vec![],
        };

        tr.into_iter().filter(|uncat_t| !self.port.transaction_exists(&uncat_t)).collect()
    }

    // Given name of Vope, transaction, updates the appropraite vope
    pub fn update_vope(
        &mut self,
        name: &str,
        transaction: &misc::Transaction,
    ) -> Result<(), Box<dyn Error>> {
        self.port.update(name, transaction)?;

        self.save()
    }

    // For vope management

    pub fn get_vope_history(&self, name: &str) -> Vec<misc::Transaction> {
        let mut hist = vec![];
        
        for v in self.port.accounts.iter() {
            if name == v.name {
                hist = v.transactions.clone();
            }
        }

        hist
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

    pub fn transfer(
        &mut self,
        from: &str,
        to: &str,
        amount: misc::Dollar,
    ) -> Result<(), Box<dyn Error>> {
        // Complete transfer
        self.port.transfer(from, to, amount)?;

        // Save account information
        self.save()
    }

    // Helper

    // Adds all the transactions in - for now, we are parsing the CSV's
    fn get_trans(&self, path: &Path) -> Result<Vec<misc::Transaction>, Box<dyn Error>> {
        let mut trans = vec![];

        let mut rdr = csv::Reader::from_path(path)?;

        // For line in csv...
        for result in rdr.records() {
            // Get the record
            let record = result?;


            // csv should be of form:
            // date(mm/dd/yyyy), description, amount
            let date = record[0].to_owned();
            let desc = record[1].to_owned();

            let amount = match money_to_float(&record[2]) {
                Ok(okay) => okay,
                Err(_) => 0.0,
            };

            trans.push(misc::Transaction::new(
                chrono::NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap(),
                desc,
                misc::Dollar::from(amount),
            ));
        }

        Ok(trans)
    }

    /**
     * Writes the account to disk
     */
    fn save(&self) -> Result<(), Box<dyn Error>> {
        let js = serde_json::to_string(&self)?;

        println!("path: {:?}", &self.path);

        fs::write(&self.path, js)?;

        Ok(())
    }
}

fn money_to_float(s: &str) -> Result<f32, std::num::ParseFloatError> {
    let mut rv = String::new();
    for c in s.chars() {
        if c != '$' && c != ' ' && c != '+' && c != ',' {
            rv.push(c);
        }
    }

    rv.parse()
}
