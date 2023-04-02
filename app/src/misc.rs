use chrono::NaiveDate;
use core::{f32, fmt};
use lazy_static::lazy_static;
use regex::Regex;
use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::hash::Hash;
use std::{fmt::Display, ops};

// A transaction
#[derive(Clone, PartialEq, Eq)]
pub struct Transaction {
    pub date: chrono::NaiveDate,
    pub desc: String,
    pub charge: Dollar,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}, {}, {}", self.date, self.desc, self.charge,))
    }
}

impl<'de> Deserialize<'de> for Transaction {
    fn deserialize<D>(deserializer: D) -> Result<Transaction, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(TransactionVisitor)
    }
}

struct TransactionVisitor;

lazy_static! {
    static ref MY_REGEX: Regex =
        Regex::new(r"^(\d{4}-\d{2}-\d{2}), ([^,]+), \$(-?[0-9]+\.[0-9]+)$").unwrap();
}

// lazy_static! {
// static ref re: Regex::new.unwrap(); // PERF: move this into a lazy_static!
// }

impl<'de> Visitor<'de> for TransactionVisitor {
    type Value = Transaction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Of form date, desc, amount")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Some(nums) = MY_REGEX.captures_iter(s).next() {
            if let Ok(date) = NaiveDate::parse_from_str(&nums[1], "%Y-%m-%d") {
                // nums[0] is the whole match, so we must skip that
                let desc = &nums[2].to_string();
                if let Ok(float) = &nums[3].parse::<f32>() {
                    Ok(Transaction::new(
                        date,
                        desc.to_string(),
                        Dollar::from(*float),
                    ))
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
                }
            } else {
                Err(de::Error::invalid_value(Unexpected::Str(s), &self))
            }
        } else {
            Err(de::Error::invalid_value(Unexpected::Str(s), &self))
        }
    }
}

impl ToString for Transaction {
    fn to_string(&self) -> String {
        "Test".to_string()
    }
}

impl Hash for Transaction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.date.hash(state);
        self.desc.hash(state);
    }
}

impl Transaction {
    pub fn new(date: chrono::NaiveDate, desc: String, charge: Dollar) -> Self {
        Self { date, desc, charge }
    }
}

fn round(num: f32, decimals: u32) -> f32 {
    let precison = 10i32.pow(decimals) as f32;
    (num * precison).round() / precison
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub struct Dollar {
    pub amount: f32,
}

impl From<f32> for Dollar {
    fn from(value: f32) -> Self {
        Dollar { amount: value }
    }
}

impl From<&str> for Dollar {
    fn from(value: &str) -> Self {
        let mut rv = String::new();
        for c in value.chars() {
            if c != '$' && c != ' ' && c != '+' && c != ',' {
                rv.push(c);
            }
        }

        Dollar::from(rv.parse::<f32>().unwrap_or(0.0))
    }
}

impl Eq for Dollar {}

impl ops::Add<f32> for Dollar {
    type Output = Dollar;

    fn add(self, rhs: f32) -> Dollar {
        Dollar::from(round(self.amount + rhs, 2))
    }
}

impl ops::Add<Dollar> for Dollar {
    type Output = Dollar;

    fn add(self, rhs: Dollar) -> Dollar {
        Dollar::from(round(self.amount + rhs.amount, 2))
    }
}
impl ops::Sub<Dollar> for Dollar {
    type Output = Dollar;

    fn sub(self, rhs: Dollar) -> Self::Output {
        Dollar::from(round(self.amount - rhs.amount, 2))
    }
}

impl ops::AddAssign<f32> for Dollar {
    fn add_assign(&mut self, rhs: f32) {
        self.amount = round(self.amount + rhs, 2);
    }
}

impl ops::AddAssign<Dollar> for Dollar {
    fn add_assign(&mut self, rhs: Dollar) {
        self.amount = round(self.amount + rhs.amount, 2);
    }
}

impl ops::SubAssign<Dollar> for Dollar {
    fn sub_assign(&mut self, rhs: Dollar) {
        self.amount = round(self.amount - rhs.amount, 2);
    }
}

impl PartialEq for Dollar {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
    }
}

impl PartialOrd for Dollar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.amount.partial_cmp(&other.amount)
    }
}

impl Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.amount)
    }
}
