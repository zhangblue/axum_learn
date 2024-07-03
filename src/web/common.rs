use std::sync::{Arc, Mutex};
use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use snowflake::SnowflakeIdGenerator;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub mc: ApplicationState,
}

// region:  --- Model Controller

// 用于模拟数据库 会有多线程访问，所以需要使用arc，同时要加锁。
#[derive(Clone)]
pub struct ApplicationState {
    pub db_conn: Arc<DatabaseConnection>,
    pub redis_client: Arc<Mutex<redis::Client>>,
    pub snowflake_id_generator: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl ApplicationState {
    pub async fn new() -> crate::error::Result<Self> {
        // 数据库链接
        let db_connect = database_common::create_database_connection().await.expect("数据库链接失败!");
        // 雪花算法
        let snowflake_id_generator = SnowflakeIdGenerator::new(1, 1);
        // redis链接
        let redis_client = redis_common::create_redis_connection().await.expect("redis链接失败");
        Ok(Self {
            db_conn: Arc::new(db_connect),
            redis_client: Arc::new(Mutex::new(redis_client)),
            snowflake_id_generator: Arc::new(Mutex::new(snowflake_id_generator)),
        })
    }
}