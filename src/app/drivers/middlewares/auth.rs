use crate::{app::features::user::entities::User, error::AppError};
use actix_web::{HttpMessage, HttpRequest};
use serde_json::json;

pub fn get_current_user(req: &HttpRequest) -> Result<User, AppError> {
    req.extensions()
        .get::<User>()
        .map(|user| user.to_owned())
        .ok_or_else(|| {
            AppError::Unauthorized(json!({"error": "Unauthrized user. Need auth token on header."}))
        })
}
