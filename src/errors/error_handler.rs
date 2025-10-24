use axum::{
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::Serialize;
use tracing::error;
use thiserror::Error;

// Handler should return Result<T, AppError> and use ? to propagate failures --

#[derive(Error, Debug)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("forbidden: {0}")]
    Forbidden(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("validation error: {0:?}")]
    Validation(Vec<String>),

    #[error("internal server error: {0}")]
    Internal(#[from] anyhow::Error),
}

// JSON body returned to clients on error
#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    message: String
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(&self) -> Response {
        error!("handling errorL {:#?}", self);

        let status = self.status_code();
        let message = self.to_string();
        let error_text = status.canonical_reason().unwrap_or("error").to_string();

        let body = ErrorBody {
            error: error_text,
            message: message,
        };

        (status, Json(body)).into_response()
    }
}
