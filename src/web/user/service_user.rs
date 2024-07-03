use sea_orm::{ActiveModelTrait, DbBackend, EntityTrait, JoinType, QuerySelect, QueryTrait, RelationTrait};
use sea_orm::ActiveValue::Set;
use database_common::entity;
use database_common::entity::prelude;
use crate::error;
use crate::web::common::{ApplicationState};
use crate::web::user::model::{UserDto, UserForCreate, UserVo};

use crate::error::Error;

pub async fn create_user(app_state: &mut ApplicationState, user_fc: UserForCreate) -> error::Result<UserVo> {
    let create_at = chrono::Utc::now().naive_utc();

    // 模拟id递增，计算id
    let id = app_state.snowflake_id_generator.lock().unwrap().real_time_generate().to_string();

    let user = entity::users::ActiveModel {
        id: Set(id),
        account: Set(user_fc.account),
        password: Set(user_fc.password),
        nickname: Set(user_fc.nickname),
        role_id: Set(user_fc.role_id),
        create_time: Set(create_at),
    };

    let insert_result = user.insert(app_state.db_conn.as_ref()).await;

    return match insert_result {
        Ok(data) => {
            Ok(model_to_user_vo(data))
        }
        Err(error) => {
            Err(Error::DatabaseOperationError { msg: error.to_string() })
        }
    };
}

pub async fn list_user(app_state: &ApplicationState) -> error::Result<Vec<UserVo>> {
    let select = prelude::Users::find()
        .select_only()
        .column_as(entity::users::Column::Id, "user_id")
        .column(entity::users::Column::Account)
        .column(entity::users::Column::Password)
        .column(entity::users::Column::Nickname)
        .column(entity::users::Column::CreateTime)
        .column_as(entity::roles::Column::Id, "role_id")
        .column(entity::roles::Column::RoleName)
        .join(
            JoinType::LeftJoin,
            entity::users::Relation::Roles.def(),
        );

    println!("list user sql = {}", select.build(DbBackend::Postgres).to_string());

    let vec_data = select.into_model::<UserDto>().all(app_state.db_conn.as_ref()).await;

    if let Err(err) = vec_data {
        return Err(Error::DatabaseOperationError { msg: err.to_string() });
    }

    let user_vo_vec: Vec<UserVo> = vec_data.unwrap().into_iter().map(|user| dto_to_user_vo(user)).collect();

    Ok(user_vo_vec)
}

pub async fn delete_user(app_state: &ApplicationState, id: &str) -> error::Result<UserVo> {
    let delete_model = get_user_one(app_state, id).await?;

    return match delete_model {
        None => {
            Err(Error::DeleteFailIdNotFound { id: id.to_string() })
        }
        Some(data) => {
            let _ = prelude::Users::delete_by_id(id).exec(app_state.db_conn.as_ref()).await;
            Ok(data)
        }
    };
}

pub async fn get_user_one(app_stat: &ApplicationState, id: &str) -> error::Result<Option<UserVo>> {
    let res = prelude::Users::find_by_id(id)
        .column_as(entity::users::Column::Id, "user_id")
        .column(entity::roles::Column::RoleName)
        .column_as(entity::roles::Column::Id, "role_id")
        .join(
            JoinType::LeftJoin,
            entity::users::Relation::Roles.def(),
        ).into_model::<UserDto>()
        .one(app_stat.db_conn.as_ref()).await;

    if let Err(err) = res {
        return Err(Error::DatabaseOperationError { msg: err.to_string() });
    }

    return match res.unwrap() {
        None => {
            Ok(None)
        }
        Some(data) => {
            Ok(Some(dto_to_user_vo(data)))
        }
    };
}

// 将model转为dto
fn model_to_user_vo(user_model: entity::users::Model) -> UserVo {
    UserVo {
        user_id: user_model.id,
        account: user_model.account,
        password: user_model.password,
        nick_name: user_model.nickname,
        create_time: base_common::date::date_time_to_date_format_default(user_model.create_time),
        role_id: user_model.role_id,
        role_name: None,
    }
}

// 将dto转为vo
fn dto_to_user_vo(user_dto: UserDto) -> UserVo {
    UserVo {
        user_id: user_dto.user_id,
        account: user_dto.account.unwrap_or_default(),
        password: user_dto.password.unwrap_or_default(),
        nick_name: user_dto.nickname.unwrap_or_default(),
        create_time: base_common::date::date_time_to_date_format_default(user_dto.create_time),
        role_id: user_dto.role_id.unwrap_or_default(),
        role_name: user_dto.role_name,
    }
}