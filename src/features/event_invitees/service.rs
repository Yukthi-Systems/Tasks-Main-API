use crate::features::task_events::{get_task_ownership_info, get_task_info};
use super::model::{PubEventInvitee, EventInvitee};
use crate::errors::{ServiceResult, ServiceError};
use crate::features::users::SessionUser;
use super::dto::PubInviteResponseDto;
use crate::messaging::rabbitmq;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Check if a user can invite for a task
/// Returns `Ok(true)` if the user can invite, `Ok(false)` otherwise
async fn can_invite(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64) -> ServiceResult<bool> {
    // Fetch the task ownership information for the task
    let task_ownership_info = get_task_ownership_info(&state, task_id).await?;

    // See if the session user is either the Assignee or Owner or Creator of the task
    if !task_ownership_info.can_create(&session_user_id) {
        return Ok(false);
    }

    Ok(true)
}


/// Check if a user can delete an invite for a task
/// Returns `Ok(true)` if the user can delete an invite, `Ok(false)` otherwise
async fn can_delete_invite(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64) -> ServiceResult<bool> {
    // Fetch the task ownership information for the task
    let task_ownership_info = get_task_ownership_info(&state, task_id).await?;

    // See if the session user is either the Assignee or Owner or Creator of the task
    if !task_ownership_info.can_delete(&session_user_id) {
        return Ok(false);
    }

    Ok(true)
}


/// Validate Validation Code for an invitee and get the public invitee information [Used with out session user]
pub async fn get_public_invitee_info(state: &AppState, task_id: &i64, invitee_email: &str, validation_code: &Uuid) -> ServiceResult<EventInvitee> {
    // Fetch the event invitee information based on task_id and the email id
    let existing_invitee = repository::get_invitee(
        &state.pg_pool,
        task_id,
        invitee_email,
    ).await?;

    // Check if the invitee exists, else return an error
    if existing_invitee.is_none() {
        return Err(ServiceError::NotFound("Invitee not found".into()));
    }
    let existing_invitee = existing_invitee.unwrap();

    if &existing_invitee.validation_code != validation_code {
        return Err(ServiceError::Unauthorized("Invalid validation code".into()));
    }

    Ok(existing_invitee)
}


/// Create an invite for a task
pub async fn create_invite(state: &AppState, session_user: &SessionUser, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_invite(state, &session_user.user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to invite for this task".into()));
    }

    // Fetch the task information associated with the task ID
    let task_info = get_task_info(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Generate a random validation code for the invitee
    let random_validation_code = Uuid::new_v4();

    // Create the invite in the database
    let result = repository::create_invite(
        &state.pg_pool,
        &task_id,
        invitee_email,
        &random_validation_code
    ).await?;

    if result == 0 {
        return Err(ServiceError::NotFound("Failed to create invite for task".into()));
    }

    // Send an invite notification to the invitee (via email)
    rabbitmq::send_notification_to_rmq(
        "email",
        &serde_json::json!({
            "to": invitee_email,
            "template": "initial_event_invite",
            "variables": {
                "session_user_email": &session_user.email,
                "session_user_first_name": &session_user.first_name,
                "session_user_last_name": &session_user.last_name,
                "organization_id": &session_user.organization_id,
                "organization_name": &session_user.organization_name,
                "validation_code": random_validation_code,
                "invitee_email": invitee_email,
                "task_info": task_info,
                "timestamp": chrono::Utc::now()
            }
        })
    ).await?;

    Ok(())
}


/// Resend an invite for a task
pub async fn resend_invite(state: &AppState, session_user: &SessionUser, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_invite(state, &session_user.user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to invite for this task".into()));
    }

    // Fetch the task information associated with the task ID
    let task_info = get_task_info(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Resend the invite in the database
    let result = repository::get_invitee(
        &state.pg_pool,
        &task_id,
        invitee_email,
    ).await?;

    // If the invitee does not exist, return an error
    if result.is_none() {
        return Err(ServiceError::NotFound("Invitee not found".into()));
    }
    let result = result.unwrap();

    // Resend the invite notification to the invitee (via email)
    rabbitmq::send_notification_to_rmq(
        "email",
        &serde_json::json!({
            "to": invitee_email,
            "template": "resend_event_invite",
            "variables": {
                "validation_code": &result.validation_code,
                "response_status": &result.response_status,
                "response_info": &result.response_info,
                "invited_at": &result.invited_at,
                "session_user_email": &session_user.email,
                "session_user_first_name": &session_user.first_name,
                "session_user_last_name": &session_user.last_name,
                "organization_id": &session_user.organization_id,
                "organization_name": &session_user.organization_name,
                "invitee_email": invitee_email,
                "task_info": task_info,
                "timestamp": chrono::Utc::now()
            }
        })
    ).await?;

    Ok(())
}


/// Delete an invite for a task
pub async fn delete_invite(state: &AppState, session_user: &SessionUser, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_delete_invite(state, &session_user.user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to delete invite for this task".into()));
    }

    // Fetch the invitee to ensure it exists before attempting deletion
    let existing_invitee = repository::get_invitee(
        &state.pg_pool,
        &task_id,
        invitee_email,
    ).await?;

    if existing_invitee.is_none() {
        return Err(ServiceError::NotFound("Invitee not found".into()));
    }

    // Fetch the task information associated with the invitee for notification purposes
    let task_info = get_task_info(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Delete the invite in the database
    let result = repository::delete_invitee(
        &state.pg_pool,
        &task_id,
        invitee_email,
    ).await?;

    if result == 0 {
        return Err(ServiceError::NotFound("Failed to delete invite for task".into()));
    }

    // Send a notification to the invitee about the cancellation of the invite (via email)
    // Uses the existing_invitee information to personalize the cancellation email as needed
    rabbitmq::send_notification_to_rmq(
        "email",
        &serde_json::json!({
            "to": invitee_email,
            "template": "cancel_event_invite",
            "variables": {
                "task_info": task_info,
                "existing_invitee": existing_invitee,
                "session_user_email": &session_user.email,
                "session_user_first_name": &session_user.first_name,
                "session_user_last_name": &session_user.last_name,
                "organization_id": &session_user.organization_id,
                "organization_name": &session_user.organization_name,
                "timestamp": chrono::Utc::now()
            }
        })
    ).await?;

    Ok(())
}


/// List all invitees for a task
pub async fn list_all_invitees(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64) -> ServiceResult<Vec<PubEventInvitee>> {
    // Validate if the user has permission to view the invitees for the task
    if !can_invite(state, session_user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to view invitees for this task".into()));
    }

    let result = repository::list_all_invitees(&state.pg_pool, &task_id).await?;

    Ok(result)
}


/// Respond to an invite for a task
pub async fn respond_to_invite(state: &AppState, event_info: &EventInvitee, resp_dto: &PubInviteResponseDto) -> ServiceResult<()> {
    // Update the response in the database
    let result = repository::update_invite_response(
        &state.pg_pool,
        &event_info.validation_code,
        &resp_dto
    ).await?;

    if result == 0 {
        return Err(ServiceError::NotFound("Failed to update invite response".into()));
    }

    // TODO: Send a reply notification back to the task owner (e.g., via email) about the invitee's response

    Ok(())
}
