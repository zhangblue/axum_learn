// region:  --- user Types

use chrono::{NaiveDateTime};
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// 用于从数据库查询后返回的
#[derive(Clone, Debug, Serialize)]
pub struct UserVo {
    pub user_id: String,
    pub account: String,
    pub password: String,
    pub nick_name: String,
    pub create_time: String,
    pub role_id: String,
    pub role_name: Option<String>,
}

// 用于用户传入
#[derive(Deserialize)]
pub struct UserForCreate {
    pub account: String,
    pub password: String,
    pub nickname: String,
    pub role_id: String,
}

#[skip_serializing_none]
#[derive(FromQueryResult, Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub user_id: String,
    pub account: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub create_time: NaiveDateTime,
    pub role_id: Option<String>,
    pub role_name: Option<String>,
}

// endregion:  --- user Types