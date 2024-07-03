use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{delete, post};
use crate::error::Result;
use crate::web::common::{ApplicationState, AppState};
use crate::web::user::model::{UserForCreate, UserVo};
use crate::web::user::service_user;

pub fn routes(mc: ApplicationState) -> Router {
    let app_state = AppState { mc };

    Router::new()
        .route("/users", post(create_user).get(list_user))
        .route("/users/:id", delete(delete_user))
        .with_state(app_state)
}

// 创建用户
async fn create_user(
    State(mut app_stat): State<ApplicationState>,
    Json(user_fc): Json<UserForCreate>,
) -> Result<Json<UserVo>> {
    println!("->> {:<12} - create_user", "处理程序");

    let create_result = service_user::create_user(&mut app_stat, user_fc).await?;
    Ok(Json(create_result))
}

// 列表所有用户
async fn list_user(State(app_state): State<ApplicationState>) -> Result<Json<Vec<UserVo>>> {
    println!("->> {:<12} - list_user", "处理程序");
    let user_vo_list = service_user::list_user(&app_state).await?;
    Ok(Json(user_vo_list))
}


// 删除用户
async fn delete_user(State(app_state): State<ApplicationState>, Path(id): Path<String>) -> Result<Json<UserVo>> {
    println!("->> {:<12} - delete_user", "处理程序");
    let delete_user = service_user::delete_user(&app_state, &id).await;

    return match delete_user {
        Ok(data) => {
            Ok(Json(data))
        }
        Err(err) => {
            Err(err)
        }
    };
}