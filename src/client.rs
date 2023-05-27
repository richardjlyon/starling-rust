//! Handles querying the Starling API and converting results into `Transaction` objects.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a Starling account
#[derive(Deserialize, Debug)]
pub struct Account {
    pub name: String,
    #[serde(rename = "accountUid")]
    pub account_uid: String,
    #[serde(rename = "defaultCategory")]
    pub default_category: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

/// Represents a list of Starling accounts
#[derive(Deserialize, Debug)]
pub struct Accounts {
    accounts: Vec<Account>,
}

/// Represents available currency values
#[derive(Deserialize, Debug)]
pub struct CurrencyValue {
    #[serde(rename = "minorUnits")]
    pennies: u32,
    currency: Currency,
}

/// Represents a feed item returned from the API
#[derive(Deserialize, Debug)]
pub struct StarlingFeedItem {
    #[serde(rename = "feedItemUid")]
    pub uid: String,
    #[serde(rename = "transactionTime")]
    pub transaction_time: DateTime<Utc>,
    #[serde(rename = "counterPartyType")]
    pub counterparty_type: String,
    #[serde(rename = "counterPartyUid")]
    pub counterparty_uid: String,
    #[serde(rename = "counterPartyName")]
    pub counterparty_name: String,
    pub direction: Direction,
    pub amount: CurrencyValue,
    pub reference: String,
    pub status: Status,
    #[serde(rename = "spendingCategory")]
    pub spending_category: String,
    #[serde(rename = "userNote")]
    pub user_note: Option<String>,
}

impl StarlingFeedItem {
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

impl ToString for StarlingFeedItem {
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

/// Represents a query to the API
#[derive(Serialize)]
struct Query {
    #[serde(rename = "changesSince")]
    changes_since: DateTime<Utc>,
}

/// Represents a single Transaction
#[derive(Deserialize, Debug)]
struct Transactions {
    #[serde(rename = "feedItems")]
    feed_items: Vec<StarlingFeedItem>,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub enum Direction {
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

#[derive(Deserialize, Debug, strum_macros::Display)]
pub enum Currency {
    GBP,
    USD,
}

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

/// Represents a single Starling account
pub struct StarlingApiClient {
    key: String,
    base_url: String,
}

#[async_trait::async_trait]
pub trait StarlingClient {
    async fn accounts(&self) -> Vec<Account>;
    async fn transactions_since(
        &self,
        account_uid: &str,
        category: &str,
        since: chrono::Duration,
    ) -> Vec<StarlingFeedItem>;
    async fn default_category(&self);
}

impl StarlingApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            key: api_key,
            base_url: "https://api.starlingbank.com/api/v2".to_string(),
        }
    }
}

#[async_trait::async_trait]
impl StarlingClient for StarlingApiClient {
    async fn accounts(&self) -> Vec<Account> {
        let mut resp = surf::get(format!("{}/accounts", &self.base_url))
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", &self.key))
            .await
            .unwrap();

        resp.body_json::<Accounts>().await.unwrap().accounts
    }

    /// Get the account holder's feed items which were created or updated since a given date
    /// /api/v2/feed/account/{accountUid}/category/{categoryUid}
    async fn transactions_since(
        &self,
        account_uid: &str,
        category: &str,
        since: chrono::Duration,
    ) -> Vec<StarlingFeedItem> {
        let start_date = Utc::now() - since;
        let mut resp = surf::get(format!(
            "{}/feed/account/{}/category/{}",
            &self.base_url, account_uid, category
        ))
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", &self.key))
        .query(&Query {
            changes_since: start_date,
        })
        .unwrap()
        .await
        .unwrap();

        resp.body_json::<Transactions>().await.unwrap().feed_items
    }

    async fn default_category(&self) {}
}
