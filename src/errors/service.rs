use super::pgsql::PgError;
use redis::RedisError;
use thiserror::Error;


pub type ServiceResult<T> = Result<T, ServiceError>;


#[derive(Error, Debug)]
pub enum ServiceError {
    // -----------------------------
    // Infrastructure / library
    // -----------------------------

    #[error("JSON error: {0}")]
    SerDe(#[from] serde_json::Error),

    #[error("HTTP Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("PostgreSQL error: {0}")]
    Pgsql(#[from] PgError),

    // -----------------------------
    // Application-specific errors
    // -----------------------------

    #[error("Precondition failed: {0}")]
    PreconditionFailed(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unprocessable entity: {0}")]
    Unprocessable(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Gone: {0}")]
    Gone(String),
}
