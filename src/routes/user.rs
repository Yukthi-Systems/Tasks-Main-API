use actix_web::{patch, web, HttpResponse};
use crate::features::users::SessionUser;
use crate::errors::ApiResponse;
use crate::database::users;
use crate::state::AppState;



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
