use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use crate::web::hello_world::model::HelloParams;

// /hello?name=张三
pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    log::info!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}

// /hello2/张三
pub async fn handler_hello2(Path(params): Path<HelloParams>) -> impl IntoResponse {
    log::info!(
        "->> {:<12} - handler_hello2 - {}",
        "HANDLER",
        params.name.as_deref().unwrap_or("World")
    );

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!!</strong>"))
}