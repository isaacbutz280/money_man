// Library Imports
use log;
use directories::ProjectDirs;
use serde;
use serde_json;
use std::{
    error,
    fs,
    path,
};

// Define and re-export crate modules
pub mod dollar;
pub mod misc;
pub mod portfolio;
pub mod transaction;
pub mod vope;
pub mod tests;

/**
 * An account contains all information about the user.
 *
 * The highest level of what the end user can see.
 *
 * It encapsulates a Portfoloio and some metadata
 */
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    port: portfolio::Portfolio,
    name: String,
    date: String,
    path: path::PathBuf,
}

impl Account {
    /**
     * The new function is the "constructor" for an Account.
     */
    pub fn new() -> Result<Account, Box<dyn error::Error>> {
        // Default account location is
        // %USERPROFILE%\AppData\Roaming\ButzIndustries\MoneyMan\data\acc.json
        let binding = ProjectDirs::from("io", "ButzIndustries", "MoneyMan").unwrap();
        let path = path::Path::new(binding.data_dir()).join("acc.json");
        fs::create_dir_all(path.parent().unwrap())?;

        log::info!("Creating new account at {:?}", path);

        let mut acc = Account {
            name: "Unknown".to_string(),
            date: "Today".to_string(),
            path: path.to_path_buf(),
            port: portfolio::Portfolio::new(),
        };

        acc.save()?;

        Ok(acc)
    }

    pub fn open(acc_path: path::PathBuf) -> Result<Account, Box<dyn error::Error>> {
        let file_read = fs::read_to_string(acc_path);

        match file_read {
            Ok(raw_json) => {
                let res_acc: Result<Account, serde_json::Error> = serde_json::from_str(&raw_json);

                match res_acc {
                    Ok(mut acc) => {
                        acc.save()?;
                        Ok(acc)
                    }
                    Err(e) => {
                        log::error!("Failed to deserialzie Account: {}", &e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to parse account file: {}", &e);
                Err(Box::new(e))
            }
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.save_as(&self.path.clone())
    }

    pub fn save_as(&mut self, acc_path: &path::Path) -> Result<(), Box<dyn error::Error>> {
        self.port.calc_holdings();
        let js = serde_json::to_string(&self)?;

        let _wr = fs::write(acc_path, js)?;

        Ok(())
    }

    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_portfolio(&self) -> & portfolio::Portfolio {
        & self.port
    }

    pub fn get_portfolio_mut(&mut self) -> &mut portfolio::Portfolio {
        &mut self.port
    }

}
