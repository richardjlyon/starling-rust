// m20220101_000001_create_counterparty_table.rs

use sea_orm_migration::prelude::*;
use super::m20220101_000001_create_transaction_table::Transaction;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_counterparty_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create the Counterparty table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Counterparty::Table)
                    .col(
                        ColumnDef::new(Counterparty::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Counterparty::Uid).string().not_null())
                    .col(ColumnDef::new(Counterparty::Type).string().not_null())
                    .col(ColumnDef::new(Counterparty::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Drop the Counterparty table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Counterparty::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Counterparty {
    Table,
    Id,
    Uid,
    Type,
    Name,
}