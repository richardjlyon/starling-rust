use crate::schemas::accounts::AccountV2;

pub mod schemas;
// use serde::Deserialize;

use config::Config;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let personal_auth_token = get_key("personal");
    let client = starling::Client::new(&personal_auth_token);

    client.accounts().await;

    // match response.status() {
    //     reqwest::StatusCode::OK => (),
    //     reqwest::StatusCode::FORBIDDEN => {
    //         println!("Authorisation failed");
    //         println!("Exiting");
    //         std::process::exit(0);
    //     }
    //     _ => {
    //         panic!("Something unexpected happened");
    //     }
    // }

    // println!("{:?}", response)

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

    let mut keys = config.try_deserialize::<HashMap<String, String>>().unwrap();

    match keys.remove(account_name) {
        Some(key) => key,
        None => {
            println!("No API key found for account'{}'", account_name);
            std::process::exit(0);
        }
    }
}
