use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::{delete, post};
use crate::error;
use crate::error::Error;
use crate::web::common::{ApplicationState, AppState};
use crate::web::role::model::{RoleForCreate, RoleVo};
use crate::web::role::service_role;

pub fn routes(mc: ApplicationState) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/roles", post(create_role).get(list_roles))
        .route("/roles/:id", delete(delete_role))
        .with_state(app_state)
}

async fn create_role(
    State(mut application_stat): State<ApplicationState>,
    Json(role_fc): Json<RoleForCreate>,
) -> error::Result<Json<RoleVo>> {
    log::info!("->> {:<12} - create_role", "处理程序");
    let role = service_role::create_role(&mut application_stat, role_fc).await?;
    Ok(Json(role))
}

async fn list_roles(
    State(application_stat): State<ApplicationState>,
) -> error::Result<Json<Vec<RoleVo>>> {
    log::info!("->> {:<12} - list_roles", "处理程序");
    let role_vo_list = service_role::list_role(&application_stat).await?;
    Ok(Json(role_vo_list))
}

async fn delete_role(
    State(application_stat): State<ApplicationState>,
    Path(id): Path<String>,
) -> error::Result<String> {
    log::info!("->> {:<12} - delete_role", "处理程序");
    let delete_role = service_role::delete_role(&application_stat, &id).await?;
    return if delete_role > 0 {
        Ok(String::from("删除成功"))
    } else {
        Err(Error::DeleteFailIdNotFound { id })
    };
}