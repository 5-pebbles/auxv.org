use std::path::PathBuf;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum ServerError {
    IoError(tokio::io::Error),
    BadRequest(String),
    NotFound(PathBuf),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::IoError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message).into_response(),
            Self::NotFound(path) => (
                StatusCode::NOT_FOUND,
                format!("File not found: {:#?}", path),
            )
                .into_response(),
        }
    }
}

impl From<tokio::io::Error> for ServerError {
    fn from(error: tokio::io::Error) -> Self {
        Self::IoError(error)
    }
}
