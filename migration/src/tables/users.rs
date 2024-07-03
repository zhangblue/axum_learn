use sea_orm_migration::prelude::*;
use crate::sea_orm::EntityTrait;
use crate::sea_orm::ActiveValue::Set;
use crate::tables::roles::Roles;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        println!("create table user begin");
        let table_create_statement = Table::create().table(Users::Table).if_not_exists()
            .col(ColumnDef::new(Users::Id).string_len(19).primary_key().not_null())
            .col(ColumnDef::new(Users::Account).string().not_null())
            .col(ColumnDef::new(Users::Password).string().not_null())
            .col(ColumnDef::new(Users::Nickname).string().not_null())
            .col(ColumnDef::new(Users::RoleId).string_len(19).not_null())
            .col(ColumnDef::new(Users::CreateTime).timestamp().default(Expr::current_timestamp()))
            .foreign_key(
                ForeignKey::create()
                    .name("user_to_role_id_fkey")
                    .from(Users::Table, Users::RoleId)
                    .to(Roles::Table, Roles::Id)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned();

        manager
            .create_table(table_create_statement).await?;

        seed_data(manager).await;
        println!("create table user finish");

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

/// 插入预置数据
async fn seed_data(manager: &SchemaManager<'_>) {
    let user_active_model = database_common::entity::users::ActiveModel {
        id: Set(String::from("0000000000000000000")),
        account: Set(String::from("admin")),
        password: Set(String::from("admin")),
        nickname: Set(String::from("超级管理员")),
        role_id: Set(String::from("super_admin_role000")),
        create_time: Set(chrono::Utc::now().naive_utc()),
    };

    if let Err(e) = database_common::entity::users::Entity::insert(user_active_model)
        .on_conflict( // id 发生冲突时什么也不做
                      OnConflict::column(database_common::entity::users::Column::Id)
                          .do_nothing()
                          .to_owned()
        ).exec(manager.get_connection()).await {
        println!("写入 [超级管理员] 发生错误. {}", e);
        return;
    }

    println!("初始化 [users] 表数据完成");
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Account,
    Password,
    Nickname,
    RoleId,
    CreateTime,
}
