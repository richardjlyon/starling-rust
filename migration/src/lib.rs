pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_transaction_table;
mod m20220101_000002_create_counterparty_table;
mod m20220101_000003_create_account_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_transaction_table::Migration),
            Box::new(m20220101_000002_create_counterparty_table::Migration),
            Box::new(m20220101_000003_create_account_table::Migration),
        ]
    }
}
