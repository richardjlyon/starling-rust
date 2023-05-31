//! Handles querying the Starling API and converting results into `Transaction` objects.

use super::{
    account::{Account, Accounts},
    transaction::{StarlingTransaction, StarlingTransactions},
};
use chrono::{DateTime, Utc};
use serde::Serialize;

enum AccountType {
    Personal(String),
    Business(String),
}

#[async_trait::async_trait]
pub trait StarlingClient {
    async fn accounts(&self) -> Vec<Account>;
    async fn transactions_since(
        &self,
        account_uid: &str,
        category: &str,
        since: chrono::Duration,
    ) -> Vec<StarlingTransaction>;
    async fn default_category(&self);
}

// API client /////////////////////////////////////////////////////////////////////////////////////////////////

/// Represents a single Starling account
pub struct StarlingApiClient {
    key: String,
    base_url: String,
}

impl StarlingApiClient {
    pub fn new(api_key: &String) -> Self {
        Self {
            key: api_key.to_owned(),
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
    ) -> Vec<StarlingTransaction> {
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

        resp.body_json::<StarlingTransactions>()
            .await
            .unwrap()
            .feed_items
    }

    async fn default_category(&self) {}
}

// Mock Client for testing /////////////////////////////////////////////////////////////////////////////////////

/// A structure for testing
struct StarlingMockClient;

#[async_trait::async_trait]
impl StarlingClient for StarlingMockClient {
    async fn accounts(&self) -> Vec<Account> {
        vec![]
    }
    async fn transactions_since(
        &self,
        _account_uid: &str,
        _category: &str,
        _since: chrono::Duration,
    ) -> Vec<StarlingTransaction> {
        vec![]
    }
    async fn default_category(&self) {}
}

/// Represents a query to the API
#[derive(Serialize)]
struct Query {
    #[serde(rename = "changesSince")]
    changes_since: DateTime<Utc>,
}
