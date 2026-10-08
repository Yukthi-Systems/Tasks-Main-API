use crate::features::users::{SessionUser, create_new_user_session, validate_session, delete_user_session};
use actix_web::{cookie::Cookie, delete, get, post, web, HttpResponse, HttpRequest};
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/login")]
pub async fn user_login(request: HttpRequest, state: web::Data<AppState>) -> ApiResponse {
    // Get the SSO Cookie from the request
    let sso_session_id_cookie = request.cookie("SSO-Session-ID").map(|c| c.value().to_string());
    if sso_session_id_cookie.is_none() {
        return Ok(HttpResponse::BadRequest().body("SSO Session undefined, please login to SSO first"));
    }
    let sso_session_id_cookie: String = sso_session_id_cookie.unwrap();

    // Generate new refresh and access tokens for the session
    let refresh_token = uuid::Uuid::new_v4();
    let access_token = uuid::Uuid::new_v4();

    // Create a new user session using the SSO session ID, refresh token, and access token
    let session_user = create_new_user_session(
        &state,
        &sso_session_id_cookie,
        refresh_token,
        access_token
    ).await?;

    // Return a X-Sesson-Refresh-ID along with X-Sesson-Access-ID
    Ok(HttpResponse::Ok()
        .insert_header(("Cache-Control", "no-cache"))
        .insert_header(("X-Refresh-ID-Token", refresh_token.to_string()))
        .insert_header(("X-Session-Expiry", (60 * 60 * 3).to_string())) // Inform client about access token expiry time in seconds
        .insert_header(("Access-Control-Expose-Headers", "X-Refresh-ID-Token, X-Session-Expiry"))
        .json(serde_json::json!({
            "access_token": access_token,
            "user_info": {
                "email": session_user.email,
                "user_id": session_user.user_id,
                "domain_name": session_user.domain_name,
                "first_name": session_user.first_name,
                "last_name": session_user.last_name,
                "organization_id": session_user.organization_id,
                "organization_name": session_user.organization_name,
                "is_external_sharing_enabled": session_user.is_external_sharing_enabled
            }
        })))
}


#[get("/session")]
pub async fn get_session(request: HttpRequest, session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    // Get the access token from the request headers
    let session_access_token = request
        .headers()
        .get("x-session-access-id")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string())
        .unwrap();  // Safe to unwrap as the access token header is expected to be present

    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    // Validate the session using the access token and session user information
    validate_session(&state, &session_user, session_access_token).await?;

    Ok(HttpResponse::Ok().json(
        serde_json::json!({
            "user_info": {
                "email": session_user.email,
                "user_id": session_user.user_id,
                "domain_name": session_user.domain_name,
                "first_name": session_user.first_name,
                "last_name": session_user.last_name,
                "organization_id": session_user.organization_id,
                "organization_name": session_user.organization_name,
                "is_external_sharing_enabled": session_user.is_external_sharing_enabled
            }
        })
    ))
}


#[delete("/logout")]
pub async fn user_logout(request: HttpRequest, session_user: web::ReqData<SessionUser>, state: web::Data<AppState>) -> ApiResponse {
    // Get the access token from the request headers
    let session_access_token = request
        .headers()
        .get("x-session-access-id")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string())
        .unwrap();  // Safe to unwrap as the access token header is expected to be present

    // Get SessionUser from request data
    let session_user = session_user.into_inner();

    // Delete the session from DB and any associated cache
    delete_user_session(&state, &session_user, session_access_token).await?;

    let mut cookie = Cookie::build("SSO-Session-ID", "")
        .path("/")
        .http_only(true)
        .finish();

    cookie.make_removal();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .body("Session deleted successfully!"))
}
