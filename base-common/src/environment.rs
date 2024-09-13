use serde::Deserialize;

/// 程序环境配置
#[derive(Deserialize, Debug)]
pub struct ApplicationEnvConfig {
    pub database: DatabaseConf,
    pub web: WebConf,
    pub redis: RedisConf,
}

/// 数据库配置
#[derive(Deserialize, Debug)]
pub struct DatabaseConf {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>,
    pub connect_timeout_seconds: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct RedisConf {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct WebConf {
    pub listening_address: String,
}