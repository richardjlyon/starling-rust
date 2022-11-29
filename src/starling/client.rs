use std::collections::HashMap;

use crate::error::AppError;
use config::Config;
use reqwest::{
    header::{self, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
use serde::de::DeserializeOwned;

const APIBASE: &str = "https://api.starlingbank.com/api/v2";

pub struct Client {
    pub name: String,
    client: reqwest::Client,
}

impl Client {
    // make an authenticated client for account_name
    pub fn new(account_name: &str) -> Self {
        let auth_key = get_key(account_name);
        let auth_string = format!("Bearer {}", auth_key);
        let mut headers = header::HeaderMap::new();

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_string).unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            client,
            name: account_name.to_string(),
        }
    }

    // get deserialised JSON for endpoint url
    pub async fn get<T: DeserializeOwned, Q: serde::Serialize>(
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

        let json_text = match status {
            StatusCode::OK => response.text().await.map_err(|_| AppError::ReadError),
            StatusCode::FORBIDDEN => Err(AppError::Authorisation),
            StatusCode::NOT_FOUND => Err(AppError::NotFound),
            _ => Err(AppError::Other),
        }?;

        // deserialise with path to error - uncomment during development

        let jd = &mut serde_json::Deserializer::from_str(&json_text);

        serde_path_to_error::deserialize(jd)
            .map_err(|e| AppError::ParseError(e.path().to_owned(), e.into_inner()))
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
    use super::*;

    #[test]
    fn it_gets_a_key() {
        let auth_key = get_key("personal");
        assert_ne!(auth_key.len(), 0);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_key_missing() {
        let auth_key = get_key("bad_name");
    }

    #[test]
    fn it_gets_a_client() {
        let client = Client::new("personal");
        assert_eq!(client.name, "personal");
    }
}
