use crate::misc;
use serde;
use std::{fmt, ops};

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Copy)]
pub struct Dollar {
    amount: f32,
}

impl Dollar {
    pub fn as_f64(&self) -> f64 {
        self.amount as f64
    }

    pub fn as_f32(&self) -> f32 {
        self.amount
    }
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

impl ops::Div for Dollar {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.amount / rhs.amount
    }
}

impl ops::Mul<f32> for Dollar {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Dollar::from(misc::round(self.amount * rhs, 2))
    }
}

impl ops::Add<f32> for Dollar {
    type Output = Dollar;

    fn add(self, rhs: f32) -> Dollar {
        Dollar::from(misc::round(self.amount + rhs, 2))
    }
}

impl ops::Add<Dollar> for Dollar {
    type Output = Dollar;

    fn add(self, rhs: Dollar) -> Dollar {
        Dollar::from(misc::round(self.amount + rhs.amount, 2))
    }
}

impl ops::Sub<Dollar> for Dollar {
    type Output = Dollar;

    fn sub(self, rhs: Dollar) -> Self::Output {
        Dollar::from(misc::round(self.amount - rhs.amount, 2))
    }
}

impl ops::Sub<f32> for Dollar {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Dollar::from(misc::round(self.amount - rhs, 2))
    }
}

impl ops::AddAssign<f32> for Dollar {
    fn add_assign(&mut self, rhs: f32) {
        self.amount = misc::round(self.amount + rhs, 2);
    }
}

impl ops::AddAssign<Dollar> for Dollar {
    fn add_assign(&mut self, rhs: Dollar) {
        self.amount = misc::round(self.amount + rhs.amount, 2);
    }
}

impl ops::SubAssign<Dollar> for Dollar {
    fn sub_assign(&mut self, rhs: Dollar) {
        self.amount = misc::round(self.amount - rhs.amount, 2);
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

impl Ord for Dollar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.amount.total_cmp(&other.amount)
    }
}

impl fmt::Display for Dollar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.amount)
    }
}


