use deadpool_postgres::PoolError;
use thiserror::Error;
use tokio_postgres::Error as PostgresError;


pub type PgResult<T> = Result<T, PgError>;
pub type PgPool = deadpool_postgres::Pool;  // Just an alias for convenience


#[derive(Debug, Error)]
pub enum PgError {
    #[error("PostgreSQL pool error: {0}")]
    Pool(#[from] PoolError),

    #[error("PostgreSQL error: {0}")]
    Postgres(#[from] PostgresError),
}
