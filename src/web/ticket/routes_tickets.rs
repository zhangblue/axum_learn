use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::error::Result;
use crate::web::common::{ApplicationState, AppState};
use crate::web::ticket::model::{TicketForCreate, TicketVo};
use crate::web::ticket::service_ticket;

pub fn routes(mc: ApplicationState) -> Router {

    let app_state = AppState { mc };

    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets)) // 相同的接口，调用post方法执行 create_ticket，调用get方式执行 list_tickets
        .route("/tickets/:id", delete(delete_ticket)) // 删除接口。调用 delete_ticket
        .with_state(app_state) // 这里设置的状态为 AppState 类型。
}

// region: --- REST Handlers
async fn create_ticket(
    State(app_stat): State<ApplicationState>, // 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
    ctx: Ctx,                          // 调用提取器并得到的参数
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<TicketVo>> {
    log::info!("->> {:<12} - create_ticket", "处理程序");

    log::info!("从handler中获取到提取器中的参数 = {}", ctx.user_id());
    let ticket = service_ticket::create_ticket(&app_stat, &ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

// 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
async fn list_tickets(State(app_stat): State<ApplicationState>, ctx: Ctx) -> Result<Json<Vec<TicketVo>>> {
    log::info!("->> {:<12} - list_tickets", "处理程序");
    let ticket_vo_list = service_ticket::list_tickets(&app_stat, &ctx).await?;
    Ok(Json(ticket_vo_list))
}

async fn delete_ticket(
    State(app_stat): State<ApplicationState>, // 这里状态类型不必改为 AppState 类型，axum可以自动从 AppState 中得到 ModelController.
    Path(id): Path<String>,
    ctx: Ctx,
) -> Result<Json<TicketVo>> {
    log::info!("->> {:<12} - delete_ticket", "处理程序");
    let delete_ticket = service_ticket::delete_ticket(&app_stat, &ctx, &id).await;

    return match delete_ticket {
        Ok(data) => {
            Ok(Json(data))
        }
        Err(err) => {
            Err(err)
        }
    };
}

// endregion: --- REST Handlers
