use crate::misc::{self, dollar};
use serde::{Deserialize, Serialize};
use std::{cmp, error, fmt, hash, path};

// A transaction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub date: chrono::NaiveDate,
    pub desc: String,
    pub charge: dollar::Dollar,
    pub partial_charge: Option<dollar::Dollar>,
    pub note: Option<String>,
}

impl Eq for Transaction {}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date && self.desc == other.desc && self.charge == other.charge
        // We call two transactions equal if they have same date, desc, and total
        // Right now, I don't care if they have the same partial_charge or note
    }
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        "Test".to_string()
    }
}

impl hash::Hash for Transaction {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.desc.hash(state);
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        // Match by date
        match self.date.partial_cmp(&other.date) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Then by description
        match self.desc.partial_cmp(&other.desc) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        // Then by charge
        self.charge.partial_cmp(&other.charge)
    }
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Match by date
        match self.date.cmp(&other.date) {
            cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Then by description
        match self.desc.cmp(&other.desc) {
            cmp::Ordering::Equal => {}
            ord => return ord,
        }
        // Then by charge
        self.charge.cmp(&other.charge)
    }
}

impl Transaction {
    pub(crate) fn new(date: chrono::NaiveDate, desc: String, charge: dollar::Dollar) -> Self {
        Self {
            date,
            desc,
            charge,
            partial_charge: None,
            note: None,
        }
    }
}

// Adds all the transactions in - for now, we are parsing the CSV's
pub fn parse_transactions(path: &path::Path) -> Result<Vec<Transaction>, Box<dyn error::Error>> {
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

        let amount = match misc::money_to_float(&record[2]) {
            Ok(okay) => okay,
            Err(_) => 0.0,
        };

        trans.push(Transaction::new(
            chrono::NaiveDate::parse_from_str(&date, "%m/%d/%Y").unwrap(),
            desc,
            dollar::Dollar::from(amount),
        ));
    }

    Ok(trans)
}

// impl Serialize for Transaction {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(&format!("{} | {} | {} | {} | {}", self.date, self.desc, self.charge, self.partial_charge, self.note))
//     }
// }
//
// impl<'de> Deserialize<'de> for Transaction {
//     fn deserialize<D>(deserializer: D) -> Result<Transaction, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_string(TransactionVisitor)
//     }
// }
//
// struct TransactionVisitor;
//
// lazy_static! {
//     static ref MY_REGEX: Regex =
//         Regex::new(r"^(\d{4}-\d{2}-\d{2}) \| ([^\|]+) \| \$(-?[0-9]+\.[0-9]+)$").unwrap();
// }
//
// impl<'de> Visitor<'de> for TransactionVisitor {
//     type Value = Transaction;
//
//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("Of form date, desc, amount")
//     }
//
//     fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
//     where
//         E: de::Error,
//     {
//         if let Some(nums) = MY_REGEX.captures_iter(s).next() {
//             if let Ok(date) = NaiveDate::parse_from_str(&nums[1], "%Y-%m-%d") {
//                 // nums[0] is the whole match, so we must skip that
//                 let desc = &nums[2].to_string();
//                 if let Ok(float) = &nums[3].parse::<f32>() {
//                     Ok(Transaction::new(
//                         date,
//                         desc.to_string(),
//                         dollar::Dollar::from(*float),
//                     ))
//                 } else {
//                     Err(de::Error::invalid_value(Unexpected::Str(s), &self))
//                 }
//             } else {
//                 Err(de::Error::invalid_value(Unexpected::Str(s), &self))
//             }
//         } else {
//             Err(de::Error::invalid_value(Unexpected::Str(s), &self))
//         }
//     }
// }
//
