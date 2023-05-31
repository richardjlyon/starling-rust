// Script to dump the feed_item as a JSON file, for testing aginst

use money::starling::client::{StarlingApiClient, StarlingClient};
use std::env;

#[tokio::main]
async fn main() {
    // dotenvy::dotenv().ok();
    // let personal_token =
    //     env::var("PERSONAL_TOKEN").expect("PERSONAL_TOKEN is not set in .env file");
    // let client = StarlingApiClient::new(personal_token);

    // for account in client.accounts().await {
    //     println!("{:#?}", account);
    // }
}
