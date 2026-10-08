use crate::utils::initial::{ApiSettings, AppSettings, RmqSettings};
use crate::cache::moka_cache::{AppCache, moka_builder};
use crate::database::pool::{init_pg_pool, warm_pool};
use crate::cache::redis_cache::init_redis;
use crate::tasks::start_background_jobs;
use redis::aio::MultiplexedConnection;
use deadpool_postgres::Pool as PgPool;
use actix_web::web::Data as webData;
use crate::features::notes::Notes;
use crate::utils::logging;
use std::time::Duration;
use std::sync::LazyLock;



pub struct AppState {
    pub pg_pool: PgPool,
    pub redis_cache: MultiplexedConnection,
    pub in_mem_cache: InMemCache,
}


pub struct InMemCache {
    pub string_based: AppCache<String>,
    pub number_based: AppCache<u64>,
    pub notes_based: AppCache<Notes>,
}


pub static API_SETTINGS: LazyLock<ApiSettings> = LazyLock::new(|| {
    ApiSettings::from_env()
});


pub static RMQ_SETTINGS: LazyLock<RmqSettings> = LazyLock::new(|| {
    RmqSettings::from_env()
});


fn init_in_mem_cache(cache_size: u64, expiration_time: Duration) -> InMemCache {
    // Build the in-memory cache (Moka)
    // Note: You can configure each type of cache separately if needed,
    // this shown below is only an example
    InMemCache {
        string_based: moka_builder(cache_size, expiration_time),
        number_based: moka_builder(cache_size, expiration_time),
        notes_based: moka_builder(cache_size, expiration_time)
    }
}


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

    // Initialize the in-memory cache (Moka)    [You can configure for each type of cache separately also, this is just an example]
    let in_mem_cache = init_in_mem_cache(app_settings.cache_settings.cache_size, app_settings.cache_settings.expiration_time);

    // Initialize the Redis cache
    let redis_cache = init_redis(&app_settings.redis_url).await;

    // Start background jobs
    start_background_jobs(pg_pool.clone());

    // Wrap the state of the application and share it
    webData::new(AppState {
        pg_pool,
        redis_cache,
        in_mem_cache,
    })
}
