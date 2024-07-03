use sea_orm::{ActiveModelTrait, EntityTrait};
use sea_orm::ActiveValue::Set;
use database_common::entity;
use database_common::entity::prelude;
use crate::error;
use crate::error::Error;
use crate::web::common::{ApplicationState};
use crate::web::role::model::{RoleForCreate, RoleVo};

// 创建角色
pub async fn create_role(application_stat: &mut ApplicationState, role_fc: RoleForCreate) -> error::Result<RoleVo> {
    let create_at = chrono::Utc::now().naive_utc();
    let id = application_stat.snowflake_id_generator.lock().unwrap().real_time_generate().to_string();
    let role = entity::roles::ActiveModel {
        id: Set(id),
        role_name: Set(role_fc.role_name),
        create_time: Set(create_at),
    };

    let insert_result = role.insert(application_stat.db_conn.as_ref()).await;

    return match insert_result {
        Ok(data) => {
            Ok(model_to_role_vo(data))
        }
        Err(error) => {
            Err(Error::DatabaseOperationError { msg: error.to_string() })
        }
    };
}

pub async fn list_role(application_stat: &ApplicationState) -> error::Result<Vec<RoleVo>> {
    let vec_data = prelude::Roles::find()
        .all(application_stat.db_conn.as_ref()).await;

    if let Err(err) = vec_data {
        return Err(Error::DatabaseOperationError { msg: err.to_string() });
    }

    let role_vo_vec: Vec<RoleVo> = vec_data.unwrap().into_iter().map(|model| model_to_role_vo(model)).collect();

    Ok(role_vo_vec)
}

pub async fn delete_role(application_stat: &ApplicationState, id: &str) -> error::Result<u64> {
    let delete_result = prelude::Roles::delete_by_id(id).exec(application_stat.db_conn.as_ref()).await;

    return match delete_result {
        Ok(data) => {
            Ok(data.rows_affected)
        }
        Err(err) => {
            Err(Error::DatabaseOperationError { msg: err.to_string() })
        }
    };
}

async fn _get_role_one(application_state: &ApplicationState, id: &str) -> error::Result<Option<RoleVo>> {
    let result = prelude::Roles::find_by_id(id).one(application_state.db_conn.as_ref()).await;
    if let Err(err) = result {
        return Err(Error::DatabaseOperationError { msg: err.to_string() });
    }

    return match result.unwrap() {
        None => {
            Ok(None)
        }
        Some(data) => {
            Ok(Some(model_to_role_vo(data)))
        }
    };
}

// 将数据库的格式转成 RoleVo
fn model_to_role_vo(role_model: entity::roles::Model) -> RoleVo {
    RoleVo {
        id: role_model.id,
        role_name: role_model.role_name,
        create_time: base_common::date::date_time_to_date_format_default(role_model.create_time),
    }
}

