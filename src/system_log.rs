use std::time::{SystemTime, UNIX_EPOCH};

use crate::ctx::Ctx;
use crate::error::ClientError;
use crate::{error::Error, error::Result};
use axum::http::Method;
use axum::http::Uri;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;
use serde_with::skip_serializing_none;
use uuid::Uuid;

// 记录日志功能
pub async fn log_request(
    uuid: Uuid,                        // 记录请求的UUID
    req_method: Method,                // 记录请求的方法
    uri: Uri,                          // 记录请求的日志
    ctx: Option<Ctx>,                  // 记录上下文数据
    service_error: Option<&Error>,     // 记录服务器异常
    client_error: Option<ClientError>, // 记录客户端异常
) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    let error_type = service_error.map(|se| se.as_ref().to_string());
    let error_data = serde_json::to_value(service_error)
        .ok()
        .and_then(|mut v| v.get_mut("data").map(|v| v.take()));

    // 创建 RequestLogLine

    let log_line = RequestLogLine {
        uuid: uuid.to_string(),
        timestamp: timestamp.to_string(),
        req_path: uri.to_string(),
        req_method: req_method.to_string(),
        user_id: ctx.map(|c| String::from(c.user_id())),
        client_error_type: client_error.map(|e| e.as_ref().to_string()),
        error_type,
        error_data,
    };

    log::info!("  ->> log_request（记录一个服务器log日志）: {}", json!(log_line));

    Ok(())
}

#[skip_serializing_none] // 跳过序列化None的内容
#[derive(Serialize)]
struct RequestLogLine {
    uuid: String, // uuid string formatted
    timestamp: String,
    // -- 用户和上下文属性
    user_id: Option<String>,

    // http 请求属性
    req_path: String,
    req_method: String,

    // -- Errors的属性
    client_error_type: Option<String>,
    error_type: Option<String>,
    error_data: Option<Value>,
}
