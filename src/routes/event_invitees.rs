use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::users::SessionUser;
use crate::features::event_invitees;
use crate::errors::ApiResponse;
use crate::state::AppState;



#[post("/create/{view_id}/{task_id}")]
pub async fn create_invitee(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, task_id) = path.into_inner();
    let invitee_email = body.into_inner();

    // Any user associated with the task can invite another user
    event_invitees::create_invite(
        &state,
        &session_user.user_id,
        view_id,
        task_id,
        &invitee_email,
    ).await?;

    Ok(HttpResponse::Created().body("Invite created successfully, notification sent to the invitee"))
}


#[patch("/resend/{view_id}/{task_id}")]
pub async fn resend_invite(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, task_id) = path.into_inner();
    let invitee_email = body.into_inner();

    // Any user associated with the task can invite another user
    event_invitees::resend_invite(
        &state,
        &session_user.user_id,
        view_id,
        task_id,
        &invitee_email,
    ).await?;

    Ok(HttpResponse::Created().body("Invite resent successfully, notification sent to the invitee"))
}
