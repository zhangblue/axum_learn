use axum::{http::StatusCode, response::IntoResponse};

use ::serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

/// `#[serde(tag = "type", content = "data")]` 用于给枚举设置序列化与反序列化的字段
/// 例如: `TicketDeleteFailIdNotFound { id: 10 }` 序列化后为：`{"type":"TicketDeleteFailIdNotFound", "data":{"id":10}}`。同样也可以反序列化。
///
/// #[derive(strum_macros::AsRefStr)] 用于对枚举自动生成`as_ref`方法.该方法返回枚举变体的名称作为字符串切片(`&str`),特别是在需要将枚举变体转换为字符串时，比如用于日志记录、序列化或生成错误消息。
#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFail, // 登录失败

    // -- Auth errors
    AuthFailNoAuthTokenCookie,  // cookie中没有找到auth-token内容
    AuthFailTokenWrongFormat,   // auth-token的格式不对
    AuthFailCtxNotInRequestExt, // 从请求中没有找到Ctx内容

    // Model errors
    DeleteFailIdNotFound { id: String }, // 删除数据时，id没有找到错误
    DatabaseOperationError { msg: String }, // 数据库操作错误

}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        log::info!("->> {:<12} - {self:?}", "INTO_RES");

        // 创建一个response的占位符
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // 向response中插入Error
        log::info!("  ->> 向response的扩展空间中插入Error");
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    // 定义客户端响应码与ClientError类型的对应关系
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),
            // -- Auth
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequestExt => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),
            // -- Model
            Self::DeleteFailIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            } // -- Fallback 最后兜底的类型
            // _ => (
            //     StatusCode::INTERNAL_SERVER_ERROR,
            //     ClientError::SERVICE_ERROR,
            // ),
            // 服务异常
            _ => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR)
            }
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,     // 登录失败错误
    NO_AUTH,        // 没有认证错误
    INVALID_PARAMS, // 无效参数错误
    SERVICE_ERROR, // 其他的服务异常，这是最后的兜底类型
}
