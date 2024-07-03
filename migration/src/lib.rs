pub use sea_orm_migration::prelude::*;

mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(tables::roles::Migration),
            Box::new(tables::users::Migration), // 创建有关联的表有先后顺序，必须被关联的表存在才可创建成功
            Box::new(tables::tickets::Migration),
        ]
    }
}
