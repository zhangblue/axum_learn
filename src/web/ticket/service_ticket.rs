use sea_orm::{EntityTrait, JoinType, QuerySelect};
use sea_orm::ActiveValue::Set;
use uuid::Uuid;
use database_common::entity;
use crate::ctx::Ctx;
use crate::error;
use crate::web::common::ApplicationState;
use crate::web::ticket::model::{TicketForCreate, TicketVo};

pub async fn create_ticket(application_state: &ApplicationState, ctx: &Ctx, ticket_fc: TicketForCreate) -> error::Result<TicketVo> {
    // 模拟id递增，计算id
    let id = Uuid::new_v4().to_string();
    // 创建当前要入库的ticket
    let ticket = entity::tickets::ActiveModel {
        id: Set(id),
        user_id: Set(String::from(ctx.user_id())),
        title: Set(ticket_fc.title),
        create_time: Set(chrono::Utc::now().naive_utc()),
    };

    // 插入
    let insert_result = entity::tickets::Entity::insert(ticket).exec_with_returning(application_state.db_conn.as_ref()).await;

    return match insert_result {
        Ok(data) => {
            let ticket_vo = model_to_ticket_vo(data);
            Ok(ticket_vo)
        }
        Err(error) => {
            Err(error::Error::DatabaseOperationError { msg: error.to_string() })
        }
    };
}

pub async fn list_tickets(application_state: &ApplicationState, _ctx: &Ctx) -> error::Result<Vec<TicketVo>> {
    let vec_data = entity::prelude::Tickets::find()
        .select_only()
        .column(entity::tickets::Column::Id)
        .column(entity::tickets::Column::Title)
        .column_as(entity::users::Column::Id, "user_id")
        .column_as(entity::users::Column::Account, "user_account")
        .column_as(entity::users::Column::Nickname, "user_nickname")
        .join(
            JoinType::LeftJoin,
            entity::prelude::Tickets::belongs_to(entity::prelude::Users)
                .from(entity::tickets::Column::UserId)
                .to(entity::users::Column::Id)
                .into(),
        ).into_model::<TicketVo>()
        .all(application_state.db_conn.as_ref()).await;

    if let Err(err) = vec_data {
        return Err(error::Error::DatabaseOperationError { msg: err.to_string() });
    }

    Ok(vec_data.unwrap())
}

pub async fn delete_ticket(app_stat: &ApplicationState, ctx: &Ctx, id: &str) -> error::Result<TicketVo> {
    let delete_model = get_ticket_one(app_stat, ctx, id).await?;

    return match delete_model {
        None => {
            Err(error::Error::DeleteFailIdNotFound { id: id.to_string() })
        }
        Some(data) => {
            let _ = entity::prelude::Tickets::delete_by_id(id).exec(app_stat.db_conn.as_ref()).await;
            Ok(data)
        }
    };
}

pub async fn get_ticket_one(application_state: &ApplicationState, _ctx: &Ctx, id: &str) -> error::Result<Option<TicketVo>> {
    let res = entity::prelude::Tickets::find_by_id(id).one(application_state.db_conn.as_ref()).await;

    if let Err(err) = res {
        return Err(error::Error::DatabaseOperationError { msg: err.to_string() });
    }

    return match res.unwrap() {
        None => {
            Ok(None)
        }
        Some(data) => {
            let ticket_vo = model_to_ticket_vo(data);
            Ok(Some(ticket_vo))
        }
    };
}

fn model_to_ticket_vo(model_ticket: entity::tickets::Model) -> TicketVo {
    TicketVo {
        id: model_ticket.id,
        user_id: model_ticket.user_id,
        title: model_ticket.title,
        user_account: None,
        user_nickname: None,
    }
}

