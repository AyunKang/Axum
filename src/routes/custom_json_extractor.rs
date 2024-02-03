use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    http::{self, StatusCode},
    BoxError, Json,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "must be a valid email address"))]
    username: String,
    #[validate(length(min = 8, message = "must be at least 8 characters"))]
    password: String,
}

#[async_trait]
impl<B> FromRequest<B> for RequestUser
where
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(request: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(user) = request
            .extract::<Json<RequestUser>>()
            .await
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("{}", err)))?;

        if let Err(error) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", error)));
        }

        Ok(user)
    }
}

pub async fn custom_json_extractor(user: RequestUser) {
    dbg!(user);
}
