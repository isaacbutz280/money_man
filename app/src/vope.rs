use serde::{Serialize, Deserialize};
use crate::{transaction, dollar};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vope {
    pub name: String,
    pub budget: dollar::Dollar,
    pub actual_amount: dollar::Dollar,
    pub transactions: Vec<transaction::Transaction>,
}

impl Vope {
    pub fn new(name: String, budget: dollar::Dollar) -> Vope {
        Vope {
            name,
            budget,
            actual_amount: dollar::Dollar::from(0.0),
            transactions: vec![],
        }
    }
}
