use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub mod entity;


// 创建数据库链接
pub async fn create_database_connection() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let mut opt = ConnectOptions::new("postgres://postgres:12345678@localhost/axum_example_db");
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let connection = Database::connect(opt).await?;

    Ok(connection)
}