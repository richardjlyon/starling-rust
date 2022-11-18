//! Initialise command
//!
//! Generate a new .beancount file for Starling transactions for the specified
//! time period, or the last 7 days

use anyhow::Context;
use budget::bean::directives::open::open as bean_open;
use budget::bean::BeanTransaction;
use budget::starling::client::Client as StarlingClient;
use chrono::{DateTime, Utc};
use itertools::Itertools;
use rust_decimal::Decimal;

const DEFAULT_DAYS: i64 = 7;

struct DateRange {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

pub async fn initialise(
    start_date: &Option<String>,
    end_date: &Option<String>,
) -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let date_range: DateRange = parse_dates(start_date, end_date);

    tracing::info!(
        "start date: {}, end date: {}",
        date_range.start,
        date_range.end
    );

    let personal = StarlingClient::new("personal");
    let business = StarlingClient::new("business");

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
                .transactions(&account.account_uid, date_range.start, date_range.end)
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

            let open_entry = bean_open(&date_range.start, &account, &String::from("GBP"));
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

fn parse_dates(start_date: &Option<String>, end_date: &Option<String>) -> DateRange {
    let end_date = match end_date {
        Some(date) => parse_date(date),
        None => chrono::Utc::now(),
    };

    let start_date = match start_date {
        Some(date) => parse_date(date),
        None => end_date - chrono::Duration::days(DEFAULT_DAYS),
    };

    DateRange {
        start: start_date,
        end: end_date,
    }
}

fn parse_date(date: &str) -> DateTime<Utc> {
    DateTime::parse_from_str(date, "%Y-%m-%d")
        .unwrap()
        .with_timezone(&Utc)
}
