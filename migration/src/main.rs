use sea_orm_migration::prelude::*;
use migration::Migrator;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = sea_orm::Database::connect("postgres://postgres:12345678@localhost/axum_example_db").await?;

    Migrator::up(&connection, None).await?;

    Ok(())
}
