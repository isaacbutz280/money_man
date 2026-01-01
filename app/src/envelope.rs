use crate::misc::{dollar, transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub(crate) name: String,
    pub(crate) budget: dollar::Dollar,
    pub(crate) actual_amount: dollar::Dollar,
    pub(crate) transactions: Vec<transaction::Transaction>,
}

// Public
impl Envelope {
    pub fn get_history(&self) -> &Vec<transaction::Transaction> {
        &self.transactions
    }

    pub fn get_budget(&self) -> dollar::Dollar {
        self.budget
    }

    pub fn get_actual(&self) -> dollar::Dollar {
        self.actual_amount
    }
}

// Create public
impl Envelope {
    pub(crate) fn new(name: String, budget: dollar::Dollar) -> Self {
        Envelope {
            name,
            budget,
            actual_amount: dollar::Dollar::from(0.0),
            transactions: vec![],
        }
    }

    // When processing a transaction, the first thing we try is to use the
    // partial transaction. If that is None, then we just use the full transaction
    pub(crate) fn add_transaction(&mut self, t: transaction::Transaction) {
        match t.partial_charge {
            Some(p) => self.actual_amount += p,
            None => self.actual_amount += t.charge,
        }

        // Add the transaction to my list
        self.transactions.push(t);
    }
}
