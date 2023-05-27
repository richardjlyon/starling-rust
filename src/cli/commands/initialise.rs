/// Generate a new .beancount file from all transactions from `start_date`
///
use budget::{
    bean::{Bean, BeanBalance, BeanTransaction},
    starling::client::Client as StarlingClient,
};
use chrono::{DateTime, Utc};

pub async fn initialise(start_date: &Option<String>) -> anyhow::Result<()> {
    let start_date = parse_date(start_date);
    let account_names = ["business", "personal"];
    let mut bean = Bean::new();

    for account_name in account_names {
        // fetch latest data from Starling
        let client = StarlingClient::new(account_name);
        let account = client.account_for_currency("GBP").await?;
        let balance = client.balance(account.account_uid).await?;
        let transactions = client.transactions_from(&account, start_date).await?;

        // println!("{:#?}", balance);
        // println!("{:#?}", transactions);

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
        Some(date) => {
            let mut date_tz = date.clone();
            date_tz.push_str(" 00:00:00 +00:00");
            DateTime::parse_from_str(&date_tz, "%Y-%m-%d %H:%M:%S %z")
                .unwrap()
                .with_timezone(&Utc)
        }
        None => chrono::Utc::now() - chrono::Duration::days(DEFAULT_DAYS),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_dates() {
        let expected = DateTime::parse_from_rfc3339("2022-11-29T00:00:00Z").unwrap();
        let date_string = Some(String::from("2022-11-29"));

        assert_eq!(expected, parse_date(&date_string));
    }
}
