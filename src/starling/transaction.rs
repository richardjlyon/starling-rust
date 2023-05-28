use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Represents a single Transaction
#[derive(Deserialize, Debug)]
pub struct StarlingTransactions {
    #[serde(rename = "feedItems")]
    pub feed_items: Vec<StarlingTransaction>,
}

/// Represents a feed item returned from the API
#[derive(Deserialize, Debug)]
pub struct StarlingTransaction {
    pub amount: CurrencyValue,
    #[serde(rename = "counterPartyName")]
    pub counterparty_name: String,
    #[serde(rename = "counterPartyType")]
    pub counterparty_type: String,
    #[serde(rename = "counterPartyUid")]
    pub counterparty_uid: Option<String>,
    pub direction: Direction,
    pub reference: String,
    #[serde(rename = "spendingCategory")]
    pub spending_category: String,
    pub status: Status,
    #[serde(rename = "transactionTime")]
    pub transaction_time: DateTime<Utc>,
    #[serde(rename = "feedItemUid")]
    pub uid: String,
    #[serde(rename = "userNote")]
    pub user_note: Option<String>,
}

/// Represents available currency values
#[derive(Deserialize, Debug)]
pub struct CurrencyValue {
    #[serde(rename = "minorUnits")]
    pub pennies: u32,
    pub currency: Currency,
}

/// Represents available currencies
#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum Currency {
    GBP,
    USD,
}

/// Represents transaction credit or debit
#[derive(Deserialize, Debug, Eq, PartialEq)]
pub enum Direction {
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

/// Represents transaction status
#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum Status {
    #[serde(rename = "UPCOMING")]
    Upcoming,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SETTLED")]
    Settled,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "REVERSED")]
    Reversed,
}

/// Compute the transaction amount
impl StarlingTransaction {
    pub fn amount(&self) -> f32 {
        let direction = match self.direction {
            Direction::In => 1.0,
            Direction::Out => -1.0,
        };
        direction * self.amount.pennies as f32 / 100.0
    }

    pub fn currency(&self) -> String {
        self.amount.currency.to_string()
    }
}

impl ToString for StarlingTransaction {
    fn to_string(&self) -> String {
        format!(
            "{} : £{}.{} {} {}",
            self.transaction_time.format("%Y-%m-%d"),
            self.amount.pennies / 100,
            self.amount.pennies % 100,
            match self.direction {
                Direction::In => "<-",
                Direction::Out => "->",
            },
            self.counterparty_name
        )
    }
}
