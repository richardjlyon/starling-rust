use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Beancount `open` directive
pub fn open(
    date: &DateTime<Utc>,
    balance_sheet_account: &String,
    amount: &Decimal,
    currency: &String,
) -> String {
    let date = date.format("%Y-%m-%d").to_string();
    let income_statement_account = String::from("Equity:Opening-Balances");

    let line1 = format!("{} open {}", date, balance_sheet_account);
    let line2 = format!("{} open {}", date, income_statement_account);
    let line3 = format!("{} * \"Deposit\"", date);
    let line4 = format!(
        "  {:<40} {:>10} {}",
        balance_sheet_account,
        amount.to_string(),
        currency
    );
    let line5 = format!("  {}", income_statement_account);

    format!("{}\n{}\n\n{}\n{}\n{}", line1, line2, line3, line4, line5)
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
    use chrono::prelude::*;

    #[test]
    fn it_constructs_an_opening_entry() {
        let date = Utc.with_ymd_and_hms(2022, 11, 18, 9, 10, 11).unwrap();
        let balance_sheet_account = String::from("Assets:Starling:Business");
        let amount = Decimal::new(10000, 2);

        let open_stmnt = open(&date, &balance_sheet_account, &amount, &String::from("GBP"));

        let line1 = "2022-11-18 open Assets:Starling:Business";
        let line2 = "2022-11-18 open Equity:Opening-Balances";
        let line3 = "2022-11-18 * \"Deposit\"";
        let line4 = "  Assets:Starling:Business                     100.00 GBP";
        let line5 = "  Equity:Opening-Balances";

        let expected = format!("{}\n{}\n\n{}\n{}\n{}", line1, line2, line3, line4, line5);

        assert_eq!(expected, open_stmnt);
    }
}
