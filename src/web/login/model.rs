use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginPlayLoad {
    pub username: String,
    pub pwd: String,
}