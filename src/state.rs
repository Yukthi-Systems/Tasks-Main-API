use crate::utils::initial::{ApiSettings, AppSettings, RmqSettings};
use crate::database::pool::{init_pg_pool, warm_pool};
use crate::tasks::start_background_jobs;
use redis::aio::MultiplexedConnection;
use deadpool_postgres::Pool as PgPool;
use actix_web::web::Data as webData;
use crate::cache::init_redis;
use std::sync::LazyLock;



pub struct AppState {
    pub pg_pool: PgPool,
    pub redis_cache: MultiplexedConnection,
}


pub static API_SETTINGS: LazyLock<ApiSettings> = LazyLock::new(|| {
    ApiSettings::from_env()
});


pub static RMQ_SETTINGS: LazyLock<RmqSettings> = LazyLock::new(|| {
    RmqSettings::from_env()
});


/// Determines if the given origin is allowed based on the API settings
pub fn cors_allowed_origin_fn(origin: &actix_web::http::header::HeaderValue, _: &actix_web::dev::RequestHead) -> bool {
    let origin_str = origin.to_str().unwrap_or("");
    let allowed_origins = &API_SETTINGS.allowed_origins;

    if allowed_origins.iter().any(|item| item == "*") {
        return true;
    }

    allowed_origins.iter().any(|item| item == origin_str)
}


/// Initializes the application state, including the Postgres pool, Redis cache, and in-memory cache
pub async fn initialize() -> webData<AppState> {
    // Preparing to start the server by collecting environment variables
    let app_settings: AppSettings = AppSettings::from_env();

    tracing::info!("Starting the server by initializing the application state");

    // Initialize the Postgres client
    let pg_pool = init_pg_pool(&app_settings.pg_settings);

    // Warm the Postgres pool based on the warm pool settings
    warm_pool(&pg_pool, &app_settings.pg_settings).await;

    // Initialize the Redis cache
    let redis_cache = init_redis(&app_settings.redis_url).await;

    // Start background jobs
    start_background_jobs(pg_pool.clone());

    // Wrap the state of the application and share it
    webData::new(AppState {
        pg_pool,
        redis_cache,
    })
}
