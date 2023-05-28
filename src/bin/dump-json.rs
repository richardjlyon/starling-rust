// Script to dump the feed_item as a JSON file, for testing aginst

use std::env;
use money::starling::client::StarlingApiClient;

fn main() {
    dotenvy::dotenv().ok();
    let personal_token =
        env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN is not set in .env file");
    let client = StarlingApiClient::new(personal_token);
}