use crate::database::health_check::health_check as pgsql_db_health_check;
use actix_web::{get, web, HttpResponse};
use crate::cache::redis_health_check;
use crate::errors::ApiResponse;
use crate::state::AppState;


// Health check endpoint
#[get("/api")]
async fn api_health_check(state: web::Data<AppState>) -> ApiResponse {

    // Redis health check
    redis_health_check(&state.redis_cache).await?;

    // PGSQL - DB health check
    pgsql_db_health_check(&state.pg_pool).await?;

    Ok(HttpResponse::Ok().body("All systems are running!"))
}
