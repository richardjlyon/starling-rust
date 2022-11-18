//! TODO:
//! - serde
//! - itertools https://crates.io/crates/itertools
//! - iterator API https://doc.rust-lang.org/stable/std/iter/
//! - what is the difference between iter and into_iter
//! - thiserror
//! - tokio process https://docs.rs/tokio/latest/tokio/process/index.html
//!
//! BONUS:
//! - rayon https://crates.io/crates/rayon
//! - tokio tasks https://tokio.rs/tokio/tutorial/spawning
//! - tokio channels https://tokio.rs/tokio/tutorial/channels
//! - tokio streams https://tokio.rs/tokio/tutorial/streams

mod cli;

use anyhow::Context;
use budget::bean::directives::open::open as bean_open;
use budget::bean::BeanTransaction;
use budget::starling::client::Client as StarlingClient;
use itertools::Itertools;
use rust_decimal::Decimal;

// fn main() {
//     let cli = Cli::parse();

//     // You can check the value provided by positional arguments, or option arguments
//     if let Some(name) = cli.name.as_deref() {
//         println!("Value for name: {}", name);
//     }
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let personal = StarlingClient::new("personal");
    let business = StarlingClient::new("business");
    let now = chrono::Utc::now();

    // let file = std::fs::File::create("starling.bean")?;

    // let stream = futures::stream::iter(&[personal, business]);
    // expand each client
    // expand each account
    // expanc each transaction
    // sort transactions
    // collect into a vector

    // all transactions for personal and business
    let mut transaction_data: Vec<BeanTransaction> = Vec::new();
    let mut transaction_total: Decimal;

    // get all transactions for the specified period
    for client in &[personal, business] {
        let accounts = client.accounts().await.context("failed to list accounts")?;

        for account in accounts {
            // tracing::info!("Account: {:#?}", account);
            tracing::info!("fetching transactions for {}/{}", client.name, account.name);
            let starling_transactions = client
                .transactions(&account.account_uid, now - chrono::Duration::days(365), now)
                .await
                .context("when fetching transactions")?;

            transaction_total = Decimal::ZERO;

            for tx in starling_transactions {
                // tracing::info!("Transaction: {:#?}", transaction);
                let bean_tx = BeanTransaction {
                    date: tx.settlement_time,
                    status: tx.status.clone(),
                    account_name: account.name.clone(),
                    counter_party_name: tx.counter_party_name.clone(),
                    reference: tx.reference.as_deref().unwrap_or_default().to_string(),
                    note: tx.user_note.as_deref().unwrap_or_default().to_string(),
                    spending_category: tx.spending_category.clone(),
                    balance_sheet_account: String::new(),
                    income_statement_account: String::new(),
                    amount: tx.to_decimal(),
                    direction: tx.direction.clone(),
                    currency: tx.amount.currency.clone(),
                };
                transaction_data.push(bean_tx);
                transaction_total += tx.to_decimal();
            }

            tracing::info!(
                "Total for account `{}` = {}",
                account.name,
                transaction_total
            );

            let open_entry = bean_open(&now, &account, &String::from("GBP"));
            tracing::info!("Open statement: {}", open_entry);
        }
    }

    // sort by date and print
    transaction_data
        .iter()
        .sorted_by_key(|tx| tx.date)
        .for_each(|tx| {
            println!("{}\n", tx);
        });

    Ok(())
}
