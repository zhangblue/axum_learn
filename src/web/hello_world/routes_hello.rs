use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use crate::web::hello_world::service_hello::{handler_hello, handler_hello2};

// hello的路由
pub fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}


// 静态资源的路由
pub fn routes_static() -> Router {
    Router::new().nest_service("/static", ServeDir::new("./"))
}