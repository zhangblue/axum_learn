// region:  --- Tocket Types

use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

// 用于从数据库查询后返回的
#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct TicketVo {
    pub id: String,
    pub user_id: String, // 创建这条数据的user_id
    pub title: String,
    pub user_account: Option<String>,
    pub user_nickname: Option<String>,
}


// 用于用户传入
#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion:  --- Tocket Types