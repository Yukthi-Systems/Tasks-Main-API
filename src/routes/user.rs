use actix_web::{HttpResponse, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::database::users;
use crate::state::AppState;
use uuid::Uuid;



#[patch("/update-fcm-token")]
pub async fn update_fcm_token(session_user: web::ReqData<SessionUser>, data: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let new_fcm_token = data.into_inner();

    // Update the FCM token for the session user
    let result = users::replace_fcm_token(&state.pg_pool, &session_user.user_id, &new_fcm_token).await?;
    if result == 0 {
        return Ok(HttpResponse::NotFound().body("FCM token update failed"));
    }

    Ok(HttpResponse::Ok().body("FCM token updated successfully"))
}


#[get("/info/{user_id}")]
pub async fn get_user_info(session_user: web::ReqData<SessionUser>, path: web::Path<Uuid>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let user_id = path.into_inner();

    // Fetch the user information from the database
    let user_info = users::get_user_info(&state.pg_pool, &user_id).await?;
    if user_info.is_none() {
        return Ok(HttpResponse::NotFound().body("User not found"));
    }
    let user_info = user_info.unwrap();

    // Check if the user is from same organization as the session user
    if user_info.organization_id != session_user.organization_id {
        return Ok(HttpResponse::Forbidden().body("Access denied"));
    }

    // If the User ID is same as the session user, return full info, otherwise return public info
    if user_info.user_id == session_user.user_id {
        return Ok(HttpResponse::Ok().json(user_info));
    } else {
        return Ok(HttpResponse::Ok().json(user_info.into_public()));
    }
}


#[post("/info/{is_public_info}")]
pub async fn update_user_info(session_user: web::ReqData<SessionUser>, path: web::Path<bool>, data: web::Json<serde_json::Value>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let is_public_info = path.into_inner();
    let new_info = data.into_inner();

    // Update the user information based on whether it is public or private
    let result = if is_public_info {
        users::update_public_info(&state.pg_pool, &session_user.user_id, &new_info).await?
    } else {
        users::update_private_info(&state.pg_pool, &session_user.user_id, &new_info).await?
    };

    // Check if the update affected any rows
    if result == 0 {
        return Ok(HttpResponse::NotFound().body("User info update failed"));
    }

    Ok(HttpResponse::Ok().body("User info updated successfully"))
}
