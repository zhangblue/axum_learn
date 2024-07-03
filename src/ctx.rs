//! 存储请求上下文的结构
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: String, // 用户id，从cookie解析出来的
}

// 构造器
impl Ctx {
    pub fn new(user_id: String) -> Self {
        Ctx { user_id }
    }
}

// 参数访问
impl Ctx {
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
}

// region: --- Ctx 提取器

// 自定义提取器，从 request 扩展空间中提取出Ctx。
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> crate::error::Result<Self> {
        println!("->> {:<12} - Ctx", "提取器", );
        println!("  ->> 从 parts 扩展中提取出 Ctx", );

        parts
            .extensions
            .get::<crate::error::Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInRequestExt)?
            .clone()
    }
}

// endregion: --- Ctx 提取器
