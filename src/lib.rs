use error::AppError;
use reqwest::header;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{self, Request};

mod error;
pub mod schemas;
use crate::schemas::accounts::{Account, AccountResponse};
use anyhow::Context;

const APIBASE: &str = "https://api.starlingbank.com/api/v2";

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth_key: &str) -> Self {
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

    // get accounts
    pub async fn accounts(&self) -> Vec<Account> {
        let url = format!("{}/accounts", APIBASE);
        let response = self.get("accounts").await.expect("Failed to get accounts");

        // grab the text
        // THIS IS A DICTIONARY, key is 'accounts'
        let json_text = response
            .text()
            .await
            .expect("Failed to get text from response");

        let data: AccountResponse = serde_json::from_str(&json_text).unwrap();

        data.accounts
    }

    // get a url
    async fn get(&self, url: &str) -> Result<reqwest::Response, AppError> {
        let url = format!("{}/{}", APIBASE, url);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .expect("Failed to get url");

        match response.status() {
            reqwest::StatusCode::OK => Ok(response),
            reqwest::StatusCode::FORBIDDEN => Err(AppError::Authorisation),
            _ => Err(AppError::Other),
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
