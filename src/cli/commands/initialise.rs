/// Generate a new .beancount file from all transactions from `start_date`
///
use budget::{
    bean::{Bean, BeanBalance, BeanTransaction},
    starling::client::Client as StarlingClient,
};
use chrono::{DateTime, Utc};

pub async fn initialise(start_date: &Option<String>) -> anyhow::Result<()> {
    let start_date = parse_date(start_date);
    let account_names = ["personal", "business"];
    let mut bean = Bean::new();

    for account_name in account_names {
        // fetch latest data from Starling
        let client = StarlingClient::new(account_name);
        let account = client.account_for_currency("GBP").await?;
        let balance = client.balance(account.account_uid).await?;
        let transactions = client
            .transactions_from(account.account_uid, start_date)
            .await?;

        bean.add_account(account.clone());

        bean.add_balance(BeanBalance {
            account_name: account.name.clone(),
            transactions: transactions.clone(),
            balance,
        });

        let bean_transactions = transactions
            .into_iter()
            .map(|tx| BeanTransaction {
                account_name: account.name.clone(),
                transaction: tx,
            })
            .collect();
        bean.add_transactions(bean_transactions);
    }

    // write transactions
    bean.write();

    Ok(())
}

fn parse_date(date: &Option<String>) -> DateTime<Utc> {
    const DEFAULT_DAYS: i64 = 7;
    match date {
        Some(date) => DateTime::parse_from_str(date, "%Y-%m-%d")
            .unwrap()
            .with_timezone(&Utc),
        None => chrono::Utc::now() - chrono::Duration::days(DEFAULT_DAYS),
    }
}
