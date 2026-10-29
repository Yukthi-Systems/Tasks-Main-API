use actix_web::{HttpResponse, delete, get, patch, post, web};
use crate::features::event_invitees::{self, PubInviteeDto};
use crate::features::users::SessionUser;
use crate::features::task_events;
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


#[delete("/delete/{view_id}/{task_id}")]
pub async fn delete_invite(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, body: web::Json<String>, state: web::Data<AppState>) -> ApiResponse {
    // Get SessionUser from request data
    let session_user = session_user.into_inner();
    let (view_id, task_id) = path.into_inner();
    let invitee_email = body.into_inner();

    // Any user associated with the task can invite another user
    event_invitees::delete_invite(
        &state,
        &session_user.user_id,
        view_id,
        task_id,
        &invitee_email,
    ).await?;

    Ok(HttpResponse::Ok().body("Invite deleted successfully"))
}


#[get("/list/{view_id}/{task_id}")]
pub async fn list_invites(session_user: web::ReqData<SessionUser>, path: web::Path<(i64, i64)>, state: web::Data<AppState>) -> ApiResponse {
    let session_user = session_user.into_inner();
    let (view_id, task_id) = path.into_inner();

    let invites = event_invitees::list_all_invitees(
        &state,
        &session_user.user_id,
        view_id,
        task_id,
    ).await?;

    Ok(HttpResponse::Ok().json(invites))
}


#[get("/event/invite/info/{validation_code}")]
pub async fn get_event_info(path: web::Path<uuid::Uuid>, query: web::Query<PubInviteeDto>, state: web::Data<AppState>) -> ApiResponse {
    let validation_code = path.into_inner();
    let pub_invitee_dto = query.into_inner();

    // Fetch the event invitee information based on task_id and the email id
    let event_info = event_invitees::get_event_invitee_info(
        &state.pg_pool,
        &pub_invitee_dto.task_id,
        &pub_invitee_dto.invitee_email,
    ).await?;

    // If it doesn't exist, return an error
    if event_info.is_none() {
        return Ok(HttpResponse::NotFound().body("Event invitee not found"));
    }
    let event_info = event_info.unwrap();

    // Check if the validation code matches
    if event_info.validation_code != validation_code {
        return Ok(HttpResponse::Unauthorized().body("Invalid validation code"));
    }

    // Fetch the task information associated with the event invitee
    let task_info = task_events::get_task_info(
        &state.pg_pool,
        event_info.task_id,
    ).await?;

    // Return the combined event and task information as JSON response
    Ok(HttpResponse::Ok().json(
        serde_json::json!({
            "event_info": event_info,
            "task_info": task_info,
        })
    ))
}
