// m20220101_000003_create_account_table.rs

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create the Account table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::AccountUid).string().not_null())
                    .col(ColumnDef::new(Account::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Account::DefaultCategory).string().not_null())
                    .col(ColumnDef::new(Account::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Drop the Account table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Account {
    Table,
    Id,
    AccountUid,
    CreatedAt,
    DefaultCategory,
    Name,
}