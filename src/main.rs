pub(crate) mod schemas;

use std::time::Duration;

#[tokio::main]
async fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let client = starling::Client::new("personal");

    let accounts = client.accounts().await;
    tracing::info!("Account 1: {:#?}", accounts[0]);

    let now = chrono::Utc::now();

    for account in accounts {
        let balance = client.balance(&account.account_uid).await;
        println!("Balance: {:#?}", balance);

        let transactions = client
            .transactions(&account.account_uid, now - chrono::Duration::days(7), now)
            .await;

        tracing::info!("Transactions: {:#?}", transactions);
    }

    tracing::info!("{:?}", now);
}
