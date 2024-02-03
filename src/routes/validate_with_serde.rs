use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>,
    password: i32,
}

pub async fn validate_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
}
