use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Database Connection Error")]
    DatabaseError,
    #[error("Redis Connection Error")]
    RedisError,
    #[error("Serde Error")]
    SerdeError,
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        crate::errlog!(self);

        let status = match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let mut response = axum::Json(self.to_string()).into_response();
        *response.status_mut() = status;
        response
    }
}
