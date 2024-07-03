use axum::{routing::post, Json, Router};
use axum::extract::State;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{error::{Error, Result}, web};
use crate::web::common::{ApplicationState, AppState};
use crate::web::login;
use crate::web::login::model::LoginPlayLoad;

pub fn routes(mc: ApplicationState) -> Router {
    let app_state = AppState { mc };
    Router::new().route("/api/login", post(api_login)).with_state(app_state)
}

async fn api_login(State(application_stat): State<ApplicationState>, cookies: Cookies, Json(playload): Json<LoginPlayLoad>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // 数据库权限登录验证
    let login_user = login::service_login::user_login(&application_stat, playload).await;

    match login_user {
        Ok(Some(data)) => {
            println!("login_user = [{:?}]", data);
            let token = format!("user-{}.exp.sign", data.user_id);
            {
                let mut lock_redis_client = application_stat.redis_client.as_ref().lock().unwrap();
                let login_user_msg = serde_json::to_string(&data).unwrap_or(String::from("none"));
                println!("login_user_info = [{}]", login_user_msg);

                redis_common::redis_opt::set_data(&mut lock_redis_client, &token, &login_user_msg);
                redis_common::redis_opt::expire(&mut lock_redis_client, &token, 1 * 60 * 60);
            }
            // 生成授权的token
            cookies.add(Cookie::new(web::AUTH_TOKEN, token));
        }
        _ => {
            return Err(Error::LoginFail);
        }
    }

    // 创建 success body
    let body = Json(json!({
        "result":{
            "success":true
        }
    }));

    Ok(body)
}




