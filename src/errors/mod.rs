use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    Tokio(TokioError),
    Mongo(mongodb::error::Error),
    Any,
}

#[derive(Debug)]
pub enum TokioError {
    Elapsed(tokio::time::error::Elapsed),
    JoinError(tokio::task::JoinError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "server error").into_response()
    }
}

impl From<tokio::time::error::Elapsed> for AppError {
    fn from(inner: tokio::time::error::Elapsed) -> Self {
        AppError::Tokio(TokioError::Elapsed(inner))
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(inner: tokio::task::JoinError) -> Self {
        AppError::Tokio(TokioError::JoinError(inner))
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(inner: mongodb::error::Error) -> Self {
        AppError::Mongo(inner)
    }
}
