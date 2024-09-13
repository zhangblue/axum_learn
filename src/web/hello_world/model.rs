use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    pub name: Option<String>,
}