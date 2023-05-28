// m20220101_000003_create_transaction_table.rs

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create the Transaction table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transaction::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transaction::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transaction::FeedUid).string().not_null())
                    .col(ColumnDef::new(Transaction::TransactionTime).timestamp().not_null())
                    .col(ColumnDef::new(Transaction::CounterpartyID).integer().not_null())
                    .col(ColumnDef::new(Transaction::Amount).float().not_null())
                    .col(ColumnDef::new(Transaction::Currency).string().not_null())
                    .col(ColumnDef::new(Transaction::SpendingCategory).string().not_null())
                    .col(ColumnDef::new(Transaction::Reference).string().not_null())
                    .col(ColumnDef::new(Transaction::UserNote).string().not_null())
                    .col(ColumnDef::new(Transaction::Status).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Drop the FeedItem table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transaction::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Transaction {
    Table,
    Id,
    FeedUid,
    TransactionTime,
    CounterpartyID,
    Amount,
    Currency,
    SpendingCategory,
    Reference,
    UserNote,
    Status,
}
