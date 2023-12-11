use directories::ProjectDirs;
use portfolio::Portfolio;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    vec,
};

/// An Account is the abstraction for a user
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    init    : bool,       // True or false, if the account has been initalized
    pub name: String,     // Persons name
    pub date: String,     // Todays date
    pub port: Portfolio,  // The banking information
}

impl Account {
    pub fn new() -> Result<Account, Box<dyn Error>> {
        // Builds account from JSON, or creates new account if unavailable
        let binding = ProjectDirs::from("io", "ButzIndustries", "MoneyMan").unwrap();
        let path = Path::new(binding.data_dir()).join("acc.json");
        let file_read = fs::read_to_string(&path);

        match file_read {
            Ok(raw_json) => {
                print!("{}",raw_json);
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
                    init: false,
                    name: "Unknown".to_string(),
                    date: "Today".to_string(),
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
        let transactions = match self.get_trans(path) {
            Ok(t) => t,
            Err(_) => vec![],
        };

        // Return all transactions we haven't already catorgorized
        transactions
            .iter()
            .filter_map(|t| {
                if self.port.categorized(t) {
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
            self.update_salary(transaction)?;
            self.save()
        } else if name != "Ignore" {
            self.port.update(name, transaction.charge)?;
            self.save()
        } else {
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

    /// Transfers amount from one Vope to another.
    ///
    /// # Paramters
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

            let mut amount = match money_to_float(&record[2]) {
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

    fn update_salary(&mut self, transaction: &Transaction) -> Result<(), Box<dyn Error>> {
        self.port.cash_paycheck(transaction.charge)
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