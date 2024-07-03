pub mod redis_opt; // redis操作

// 创建redis链接
pub async fn create_redis_connection() -> Result<redis::Client, Box<dyn std::error::Error>> {
    let client = redis::Client::open("redis://127.0.0.1")?;
    Ok(client)
}