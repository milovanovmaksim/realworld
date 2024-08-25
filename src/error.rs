use serde_json::Value as JsonValue;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // 422
    #[error("Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),
}
