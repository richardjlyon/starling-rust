mod error;
pub mod schemas;

use config::Config;
use error::AppError;
use reqwest::header;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::schemas::accounts::{Account, AccountId, AccountResponse, Balance};
use crate::schemas::transactions::{Transaction, TransactionResponse};

const APIBASE: &str = "https://api.starlingbank.com/api/v2";

// A Starling API client
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    // make an authenticated client for account_name
    pub fn new(account_name: &str) -> Self {
        let auth_key = get_key(account_name);
        let auth_string = format!("Bearer {}", auth_key);
        let mut headers = header::HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(&auth_string).unwrap(),
        );
        headers.insert(
            CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(ACCEPT, header::HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self { client: client }
    }

    // endpoint /accounts
    pub async fn accounts(&self) -> Vec<Account> {
        let data: AccountResponse = self
            .get("accounts", &())
            .await
            .expect("Failed to get accounts");
        data.accounts
    }

    // endpoint /accounts/account_uid/balancd
    pub async fn balance(&self, account_uid: &AccountId) -> Balance {
        let url = format!("accounts/{}/balance", account_uid);
        let data: Balance = self.get(&url, &()).await.expect("Failed to get balance");

        data
    }

    // endpoint /feed/account/account_uid/settled-transactions-between
    pub async fn transactions(
        &self,
        account_uid: &AccountId,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
    ) -> Vec<Transaction> {
        #[derive(serde::Serialize)]
        struct Params {
            #[serde(rename = "minTransactionTimestamp")]
            min_transaction_timestamp: chrono::DateTime<chrono::Utc>,
            #[serde(rename = "maxTransactionTimestamp")]
            max_transaction_timestamp: chrono::DateTime<chrono::Utc>,
        }

        let url = format!("feed/account/{}/settled-transactions-between", account_uid);

        let data: TransactionResponse = self
            .get(
                &url,
                &Params {
                    min_transaction_timestamp: start_date,
                    max_transaction_timestamp: end_date,
                },
            )
            .await
            .expect("Failed to get transactions");

        tracing::info!("Data: {:#?}", data);

        data.feed_items
    }

    // get deserialised JSON for endpoint url
    async fn get<T: DeserializeOwned, Q: serde::Serialize>(
        &self,
        url: &str,
        query: &Q,
    ) -> Result<T, AppError> {
        let url = format!("{}/{}", APIBASE, url);

        // Result<a, b> + fn b -> c = Result<a, c>
        // by default, the question mark will _also_ attempt to convert whatever
        // error type into the error type in question
        let response = self
            .client
            .get(url)
            .query(query)
            .send()
            .await
            .map_err(|_| AppError::NetworkError)?;

        // status only borrows the request
        let status = response.status();

        let text = match status {
            StatusCode::OK => response.text().await.map_err(|_| AppError::ReadError),
            StatusCode::FORBIDDEN => Err(AppError::Authorisation),
            StatusCode::NOT_FOUND => Err(AppError::NotFound),
            _ => Err(AppError::Other),
        }?;

        serde_json::from_str(&text).map_err(|_| AppError::ParseError)
    }
}

// get the api key for the specified account name
fn get_key(account_name: &str) -> String {
    let config = Config::builder()
        .add_source(config::File::with_name("keys"))
        .build()
        .unwrap();

    let mut keys = config.try_deserialize::<HashMap<String, String>>().unwrap();

    match keys.remove(account_name) {
        Some(key) => key,
        None => {
            tracing::warn!("No API key found for account'{}'", account_name);
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("It panics!");
    }
}
