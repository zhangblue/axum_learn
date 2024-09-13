pub mod redis_opt; // redis操作

// 创建redis链接
pub async fn create_redis_connection(redis: &base_common::environment::RedisConf) -> Result<redis::Client, Box<dyn std::error::Error>> {
    let client = redis::Client::open(redis.url.as_str())?;
    Ok(client)
}