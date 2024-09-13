use axum::{
    http::{Method, Uri},
    middleware,
    response::{IntoResponse, Response},
    Json, Router,
};
use ctx::Ctx;
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;
use migration::MigratorTrait;

use crate::{error::Error, system_log::log_request};
use crate::web::common::{ApplicationState};
use crate::web::{role, ticket, user};
use crate::web::hello_world::routes_hello::{routes_hello, routes_static};

mod ctx;
mod error;
mod system_log;
mod web;

mod test;

#[tokio::main]
async fn main() -> error::Result<()> {

    // 初始化log
    init_log();
    // 读取配置文件
    let app_config = web::common::load_app_config();

    // 初始化 ModelController
    let mc = ApplicationState::new(&app_config).await?;
    // 数据库表创建
    migration::Migrator::up(mc.db_conn.as_ref(), None).await.expect("创建表失败");


    // 只给 /api下的所有url添加auth-token验证。其他的url不受这个中间件的影响
    let routes_apis =
        ticket::routes_tickets::routes(mc.clone()) // 添加ticket 接口
            .merge(user::routes_users::routes(mc.clone())) // 添加user接口
            .merge(role::routes_role::routes(mc.clone())) // 添加role接口
            .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));


    let routes_all = Router::new()
        .merge(routes_hello()) // 测试的路由
        .merge(web::login::routes_login::routes(mc.clone())) // 加入登录接口的路由
        .nest("/api", routes_apis)
        .layer(axum::middleware::map_response(main_response_mapper)) // 设置response的统一处理中间件
        .layer(
            // 首先这个中间件是为了从Cookie中提取数据并校验auth-token, 所以位置必须是在 CookieManagerLayer 中间件的后面执行。同时因为要将Ctx放入请求扩展中，方便后续中间件与handler通过自定义提取器 mw_require_auth 中提取Ctx，所以当前 mv_ctx_resolver 中间件必须是在
            axum::middleware::from_fn_with_state(mc, web::mw_auth::mv_ctx_resolver),
        )
        .layer(CookieManagerLayer::new()) // cookie管理器。用于在 HTTP 请求和响应之间进行 Cookie 的传递和管理
        .fallback_service(routes_static()); // 如果访问失败，设置静态资源

    // 启动服务 开始
    let listener = tokio::net::TcpListener::bind(&app_config.web.listening_address)
        .await
        .unwrap();

    log::info!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();

    // 启动服务 结束

    Ok(())
}

// 统一response
async fn main_response_mapper(
    ctx: Option<Ctx>, // 向req中放入的值为Err时，会自动转换成None。如果放入的是Ok，则自动转为Some
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    log::info!("->> {:12} - main_response_mapper", "统一 response 格式化");
    log::info!("  ->> 此处会触发调用Ctx的提取器. 当前ctx为：[{:?}]", ctx);

    let uuid = Uuid::new_v4();

    // 获取最后的response error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // 如果是client error, 创建一个新的response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = serde_json::json!({
                "error":{
                    "type":client_error.as_ref(),
                    "req_uuid":uuid.to_string()
                }
            });
            log::info!("  ->> client_error_body（返回给客户端的信息）: {client_error_body}");

            // 根据 client_error_body 创建一个新的 response
            (*status_code, Json(client_error_body)).into_response()
        });

    // 创建并记录一个服务器log
    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    log::info!("");

    error_response.unwrap_or(res)
}


fn init_log() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
}

