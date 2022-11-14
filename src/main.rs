pub mod schemas;

#[tokio::main]
async fn main() {
    let client = starling::Client::new("personal");
    let accounts = client.accounts().await;

    println!("Account 1: {:#?}", accounts[0]);

    let balance = client.balance(&accounts[0].account_uid).await;
    println!("Balance: {:#?}", balance);
}
