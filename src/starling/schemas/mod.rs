use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

pub mod account;
pub mod balance;
pub mod transaction;

// ALEX any way to reduce path lenth for imports like Python __init__ ?

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignedCurrencyAndAmount {
    pub currency: String,
    pub minor_units: i64,
}

impl SignedCurrencyAndAmount {
    pub fn as_decimal(&self) -> Decimal {
        Decimal::new(self.minor_units, 2)
    }
}

impl Sub for SignedCurrencyAndAmount {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            currency: self.currency,
            minor_units: self.minor_units - other.minor_units,
        }
    }
}

impl Add for SignedCurrencyAndAmount {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            currency: self.currency,
            minor_units: self.minor_units + other.minor_units,
        }
    }
}
