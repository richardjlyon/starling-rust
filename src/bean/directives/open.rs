use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Utc,
};

use crate::starling::schemas::accounts::Account;

/// Beancount `open` directive
pub fn open(date: &DateTime<Utc>, account: &Account, currency: &String) -> String {
    format!(
        "{date} open {balance_sheet_account:<25} {currency}",
        date = fmt_date(&date),
        balance_sheet_account = fmt_balance_sheet_account(&account.name),
        currency = currency
    )
}

fn fmt_date(date: &DateTime<Utc>) -> DelayedFormat<StrftimeItems> {
    date.format("%Y-%m-%d")
}

// FIXME Generalise this to any account
fn fmt_balance_sheet_account(account_name: &String) -> String {
    format!("Assets:Starling:{}", account_name)
}

// fn fmt_currency(currency: &String) -> String {
//     currency
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_formats() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
