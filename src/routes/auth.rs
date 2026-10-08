use actix_web::{cookie::Cookie, delete, get, post, web, HttpResponse};
use crate::cache::redis_cache::{set_redis_cache, delete_redis_cache};
use crate::features::users::SessionUser;
use crate::state::AppState;
use crate::errors::ApiResponse;



#[post("/session")]
pub async fn create_session_handler(
    user_name: String,         // The request body (For now accept anything)
    state: web::Data<AppState>, // The state containing the Cache
) -> ApiResponse {
    // Generate a new session ID
    let session = SessionUser::create(user_name);

    let session_cache_key = format!("session:{}", session.session_id);

    set_redis_cache(&state.redis_cache, &session_cache_key, &session, 120).await?;

    tracing::info!("Created new session: {}", session);

    Ok(HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("X-CSRF-Token", session.csrf_token))
        .cookie(
            Cookie::build("Session-ID", &session.session_id)
                .path("/")
                .http_only(true)
                .finish(),
        )
        .body("Session created successfully!"))
}


#[get("/session")]
pub async fn get_session_handler(session_user: web::ReqData<SessionUser>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    Ok(HttpResponse::Ok().body(format!("Hello, {}", session_user.user_name)))
}


#[delete("/session")]
pub async fn delete_session_handler(session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    let session_cache_key = format!("session:{}", session_user.session_id);
    delete_redis_cache(&state.redis_cache, &session_cache_key).await?;

    let mut cookie = Cookie::build("Session-ID", "")
        .path("/")
        .http_only(true)
        .finish();

    cookie.make_removal();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body("Session deleted successfully!"))
}
