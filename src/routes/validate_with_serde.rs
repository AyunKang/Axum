use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    pub username: Option<String>,
    pub password: i32,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}
