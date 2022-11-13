use crate::schemas::accounts::AccountV2;

pub mod schemas;
// use serde::Deserialize;

use config::Config;
use std::collections::HashMap;
use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE, AUTHORIZATION};

const APIBASE: &str = "https://api.starlingbank.com/api/v2";

#[tokio::main]
async fn main() {
    let personal_auth_token = get_key("personal");
    let url = format!("{}/accounts", APIBASE);

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header(AUTHORIZATION, format!(" Bearer {}", personal_auth_token))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         println!("Success. {:?}");
    //     },
    //     reqwest::StatusCode::UNAUTHORIZED => {
    //         println!("Need an auth code");
    //     }
    //     _ => {
    //         panic!("Something unexpected happened");
    //     }
    // }

    println!("{:?}", response)

    // let account_v2 = AccountV2{
    //     description:String::from("test")
    // };
    // println!("I have account {:?}", account_v2);
}

// get the api key for the specified account name
// TODO Ask lex why not Optional?
fn get_key(account_name: &str) -> String {
    
    let config = Config::builder()
    .add_source(config::File::with_name("keys"))
    .build()
    .unwrap();

    let mut keys = config
    .try_deserialize::<HashMap<String, String>>()
    .unwrap();

    match keys.remove(account_name) {
        Some(key) => key,
        None => {
            println!("No API key found for account'{}'", account_name);
            std::process::exit(0);
        }
    }
}
