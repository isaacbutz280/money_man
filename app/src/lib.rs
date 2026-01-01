// Library Imports
use std::{error, fs, path};

// Define and re-export crate modules
pub mod envelope;
pub mod misc;
pub mod portfolio;

/**
 * An account contains all information about the user.
 *
 * The highest level of what the end user can see.
 *
 * It encapsulates a Portfoloio and some metadata
 */
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Account {
    prt: Box<portfolio::Portfolio>,
    cfg: AccConfig,
}

impl Account {
    /**
     * The new function is the "constructor" for an Account.
     */
    pub fn new() -> Result<Account, Box<dyn error::Error>> {
        let mut acc = Account {
            prt: Box::new(portfolio::Portfolio::new()),
            cfg: AccConfig::new(),
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

    // Helper function for saving to my own path
    fn save(&mut self) -> Result<(), Box<dyn error::Error>> {
        self.save_as(&self.cfg.path.clone())
    }

    pub fn save_as(&mut self, acc_path: &path::Path) -> Result<(), Box<dyn error::Error>> {
        self.prt.calc_holdings();
        let js = serde_json::to_string(&self)?;

        let _wr = fs::write(acc_path, js)?;

        Ok(())
    }

    pub fn get_portfolio(&self) -> &portfolio::Portfolio {
        &self.prt
    }

    pub fn get_portfolio_mut(&mut self) -> &mut portfolio::Portfolio {
        &mut self.prt
    }

    pub fn get_config_mut(&mut self) -> &mut AccConfig {
        &mut self.cfg
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct AccConfig {
    name: String,
    date: String,
    path: path::PathBuf,
}

impl AccConfig {
    fn new() -> Self {
        // Default account location is ~/acc.js
        let path = path::Path::new("~").join("acc.json");
        log::info!("Creating new account at {:?}", path);

        AccConfig {
            name: "Unknown".to_string(),
            date: "Today".to_string(),
            path: path.to_path_buf(),
        }
    }

    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }
}
