use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data = Data {
        message: "Hi from Axum".to_owned(),
        count: 132,
        username: "Lucy Kang".to_owned(),
    };

    Json(data)
}
