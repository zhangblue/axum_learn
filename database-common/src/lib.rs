use std::time::Duration;
use sea_orm::{ConnectOptions, DatabaseConnection};
use base_common::environment;

pub mod entity;

// 创建数据库链接
pub async fn create_database_connection(database: &environment::DatabaseConf) -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut opt = ConnectOptions::new(&database.url);
    opt.max_connections(database.max_connections.unwrap_or(100))
        .min_connections(database.min_connections.unwrap_or(5))
        .connect_timeout(Duration::from_secs(database.connect_timeout_seconds.unwrap_or(8)))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let connection = sea_orm::Database::connect(opt).await?;

    Ok(connection)
}