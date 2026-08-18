use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPayLoad {
    pub username: String,
    pub pwd: String,
}