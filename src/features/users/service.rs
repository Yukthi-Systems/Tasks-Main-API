use crate::cache::{set_redis_cache, delete_redis_cache};
use crate::errors::{ServiceResult, ServiceError};
use super::dto::SingleSignOnInfoDTO;
use crate::state::API_SETTINGS;
use super::model::SessionUser;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;


/// Call the SSO API for information about the user using SSO Cookie
async fn fetch_sso_info(sso_cookie_token: &str) -> ServiceResult<SingleSignOnInfoDTO> {
    let client = reqwest::Client::new();
    let url = format!("{}/internal/tasks/user-info/{}", API_SETTINGS.sso_api_url, sso_cookie_token);

    let resp = client
        .get(&url)
        .header("accept", "application/json")
        .header("X-API-Key", &API_SETTINGS.sso_api_key)
        .send()
        .await?
        .json::<SingleSignOnInfoDTO>()
        .await?;

    Ok(resp)
}


/// Create a new user session using the provided SSO session ID, refresh token, and access token
/// After successfully creating the user session, it will store the session information in both the PostgreSQL database and the Redis cache
pub async fn create_new_user_session(state: &AppState, sso_session_id: &str, refresh_token: Uuid, access_token: Uuid) -> ServiceResult<SessionUser> {
    // Fetch the SSO information for the user using the provided SSO session ID
    let sso_info = fetch_sso_info(&sso_session_id).await?;

    // Build the session object for the user
    // We create a new session for the user, by using the SSO session ID and the newly generated refresh token
    let session_user = SessionUser::new(
        sso_info,
        refresh_token,
        sso_session_id.into(),
    );

    // Create a PgSQL session entry (Will have Refresh Token and User Info) - Long lived (like 30 days or so)
    repository::create_user_session(&state.pg_pool, &session_user).await?;

    // Create a Redis cache entry (Will have Access Token linked with Refresh Token and User Info too) - Short lived (like 3 Hrs or so)
    let redis_cache_key = format!("session:{}", access_token);
    set_redis_cache(&state.redis_cache, &redis_cache_key, &session_user, 60 * 60 * 3).await?;

    Ok(session_user)
}


/// Validate the user session by checking its existence in the PostgreSQL database and refreshing the Redis cache if valid
pub async fn validate_session(state: &AppState, session_user: &SessionUser, access_token: String) -> ServiceResult<()> {
    let is_session_valid: bool = repository::check_user_session(&state.pg_pool, &session_user.refresh_token, &session_user.sso_token, &session_user.user_id).await?;
    if !is_session_valid {
        // Clear the Redis cache for this session as it is no longer valid
        let redis_cache_key = format!("session:{}", access_token);
        delete_redis_cache(&state.redis_cache, &redis_cache_key).await?;

        return Err(ServiceError::Unauthorized("Session is no longer valid".into()));
    }

    // If the session is still valid, we can refresh the Redis cache to extend its TTL
    let redis_cache_key = format!("session:{}", access_token);
    set_redis_cache(&state.redis_cache, &redis_cache_key, &session_user, 60 * 60 * 3).await?;

    Ok(())
}


/// Delete the user session from both PostgreSQL and Redis cache
pub async fn delete_user_session(state: &AppState, refresh_token: &Uuid, access_token: String) -> ServiceResult<()> {
    // Delete the session from the PostgreSQL database
    let deleted_count = repository::delete_user_session(&state.pg_pool, refresh_token).await?;
    tracing::info!("Deleted {} user session(s) from PostgreSQL", deleted_count);

    if deleted_count == 0 {
        tracing::warn!("No user session found for the given refresh token in PostgreSQL");
    }

    // Delete the session from the Redis cache
    let redis_cache_key = format!("session:{}", access_token);
    delete_redis_cache(&state.redis_cache, &redis_cache_key).await?;

    Ok(())
}
