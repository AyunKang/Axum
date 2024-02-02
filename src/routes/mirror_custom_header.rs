use axum::http::HeaderMap;

pub async fn mirror_custom_header(header: HeaderMap) -> String {
    let message_value = header.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}
