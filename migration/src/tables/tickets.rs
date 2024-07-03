use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let table_create_statement = Table::create().table(Tickets::Table).if_not_exists()
            .col(ColumnDef::new(Tickets::Id).string_len(36).primary_key().not_null())
            .col(ColumnDef::new(Tickets::UserId).string_len(19).not_null())
            .col(ColumnDef::new(Tickets::Title).string().not_null())
            .col(ColumnDef::new(Tickets::CreateTime).timestamp().default(Expr::current_timestamp()))
            .to_owned();

        manager.create_table(table_create_statement).await?;

        println!("create table tickets finish");

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Tickets {
    Table,
    Id,
    UserId,
    Title,
    CreateTime,
}
