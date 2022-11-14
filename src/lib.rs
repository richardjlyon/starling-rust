const APIBASE: &str = "https://api.starlingbank.com/api/v2";
use reqwest;
use reqwest::header;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            header::HeaderValue::from_str(auth_key).unwrap(),
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

    // return accounts
    pub async fn accounts(&self) {
        let url = format!("{}/accounts", APIBASE);
        let response = self.get("accounts").await;

        println!("{:?}", response);
    }

    // get a url
    async fn get(&self, url: &str) -> &reqwest::Response {
        let url = format!("{}/{}", APIBASE, url);
        let response = &self
            .client
            .get(url)
            .send()
            .await
            .expect("Failed to get url");

        response.clone()
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
