use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct RoleVo {
    pub id: String,
    pub role_name: String,
    pub create_time: String,
}

#[derive(Deserialize)]
pub struct RoleForCreate {
    pub role_name: String,
}