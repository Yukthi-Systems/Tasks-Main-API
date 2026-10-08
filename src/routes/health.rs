use crate::database::health_check::health_check;
use crate::cache::{redis_cache, moka_cache};
use crate::errors::{ApiResponse, ServiceError};
use actix_web::{get, web, HttpResponse};
use crate::state::AppState;
use std::sync::Arc;


#[derive(serde::Serialize, serde::Deserialize)]
struct RedisCacheHealthCheck {
    rank: i32,
    user_id: i32,
}


// Health check endpoint
#[get("/api")]
async fn api_health_check() -> HttpResponse {
    HttpResponse::Ok().body("Server is running!")
}


// Database health check
#[get("/pgsql")]
async fn db_health_check(state: web::Data<AppState>) -> ApiResponse {
    health_check(&state.pg_pool).await?;
    Ok(HttpResponse::Ok().body("Database is running!"))
}


// Cache health check
#[get("/cache/redis")]
async fn redis_cache_health_check(state: web::Data<AppState>) -> ApiResponse {
    let cache = &state.redis_cache;

    const CACHE_KEY: &str = "health_check";
    let cache_value = RedisCacheHealthCheck {
        rank: 1,
        user_id: 1,
    };
    
    // Insert the cache value into Redis
    redis_cache::set_redis_cache(cache, CACHE_KEY, &cache_value, 60).await?;

    let cached_value: Option<RedisCacheHealthCheck> = redis_cache::get_redis_cache(cache, CACHE_KEY).await?;
    if cached_value.is_none() {
        return Err(ServiceError::PreconditionFailed("Redis cache health check failed!".into()));
    }

    Ok(HttpResponse::Ok().body("Redis cache is running!"))
}


// Cache health check
#[get("/cache/in-mem")]
async fn in_mem_cache_health_check(state: web::Data<AppState>) -> ApiResponse {
    let cache = &state.in_mem_cache.number_based;

    const CACHE_KEY: &str = "health_check";
    let cache_value: u64 = 1;

    let cache_value: Arc<u64> = Arc::new(cache_value);

    // Insert the cache value into the in-memory cache
    moka_cache::cache_data(cache, CACHE_KEY, cache_value).await;

    let cached_value: Option<Arc<u64>> = moka_cache::get_cached_data(cache, CACHE_KEY).await;

    if cached_value.is_none() {
        return Err(ServiceError::PreconditionFailed("In-memory cache health check failed!".into()));
    }

    Ok(HttpResponse::Ok().body("In-memory cache is running!"))
}
