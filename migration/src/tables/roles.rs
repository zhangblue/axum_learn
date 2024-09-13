use sea_orm_migration::prelude::*;
use crate::sea_orm::EntityTrait;
use crate::sea_orm::ActiveValue::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let table_create_statement = Table::create().table(Roles::Table).if_not_exists()
            .col(ColumnDef::new(Roles::Id).string_len(19).primary_key().not_null())
            .col(ColumnDef::new(Roles::RoleName).string().not_null())
            .col(ColumnDef::new(Roles::CreateTime).timestamp().default(Expr::current_timestamp()))
            .to_owned();


        manager.create_table(table_create_statement).await?;

        seed_data(manager).await;
        log::info!("create table roles finish");
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

/// 添加预置数据
async fn seed_data(manager: &SchemaManager<'_>) {
    let role_active_mode = database_common::entity::roles::ActiveModel {
        id: Set(String::from(
            "super_admin_role000")),
        role_name: Set(String::from(
            "超级管理员角色")),
        create_time: Set(chrono::Utc::now().naive_utc()),
    };


    if let Err(e) = database_common::entity::roles::Entity::insert(role_active_mode)
        .on_conflict( // 发生冲突时什么都不做
                      sea_query::OnConflict::column(database_common::entity::roles::Column::Id)
                          .do_nothing()
                          .to_owned()
        ).exec(manager.get_connection()).await {
        log::info!("写入 [超级管理员角色] 发生错误. {}", e);
        return;
    }

    log::info!("初始化 [roles] 表数据完成");
}

#[derive(DeriveIden)]
pub enum Roles {
    Table,
    Id,
    RoleName,
    CreateTime,
}
