use sea_orm::{ColumnTrait, DatabaseBackend, EntityTrait, QueryFilter, QuerySelect, QueryTrait, RelationTrait};
use database_common::entity;
use migration::JoinType;
use crate::{error};
use crate::web::common::ApplicationState;
use crate::web::login::model::LoginPlayLoad;
use crate::web::user::model::UserDto;

// 执行登录逻辑
pub async fn user_login(application_state: &ApplicationState, login: LoginPlayLoad) -> error::Result<Option<UserDto>> {
    let select = entity::users::Entity::find()
        .join(
            JoinType::InnerJoin,
            entity::users::Relation::Roles.def(),
        )
        .column_as(entity::users::Column::Id, "user_id")
        // .column_as(entity::roles::Column::RoleName, "role_name")
        .filter(entity::users::Column::Account.eq(login.username))
        .filter(entity::users::Column::Password.eq(login.pwd));

    println!("login sql = [{}]", select.build(DatabaseBackend::Postgres).to_string());

    let login_user = select.into_model::<UserDto>()
        .one(application_state.db_conn.as_ref()).await;


    if let Err(err) = login_user {
        println!("登录失败");
        return Err(error::Error::DatabaseOperationError { msg: err.to_string() });
    }

    return match login_user.unwrap() {
        None => {
            println!("没查询到数据");
            Ok(None)
        }
        Some(data) => {
            Ok(Some(data))
        }
    };
}