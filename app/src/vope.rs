use serde::{Serialize, Deserialize};
use crate::misc;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vope {
    pub name: String,
    pub budget: misc::Dollar,
    pub actual_amount: misc::Dollar,
    pub transactions: Vec<misc::Transaction>,
}

impl Vope {
    pub fn new(name: String, budget: misc::Dollar) -> Vope {
        Vope {
            name,
            budget,
            actual_amount: misc::Dollar::from(0.0),
            transactions: vec![],
        }
    }
}



