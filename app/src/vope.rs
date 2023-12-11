use serde::{Serialize, Deserialize};
use crate::misc;

/// An envelope for categorizing.
/// 
/// # Examples
/// 
/// ```
/// let mut vope = Vope::new(String::from("Auto Loan"), misc::Dollar::from(150.0))
/// ```
/// 
/// 
/* An envelope holds infomation about the category, and a list of transactions that have been assigned to this envelop*/
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vope {
    name         : String,                  // Name of this envelope
    budget       : misc::Dollar,            // The budgeted amount of the envelope
    actual_amount: misc::Dollar,            // The actual holdings of the envelope
    transactions : Vec<misc::Transaction>,  // An ordered list of transactions
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
