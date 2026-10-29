use crate::features::task_events::get_task_ownership_info;
use crate::errors::{ServiceResult, ServiceError};
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


/// Create an invite for a task
pub async fn create_invite(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_invite(state, session_user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to invite for this task".into()));
    }

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

    // Send an invite notification to the invitee (e.g., via email)
    // TODO: Make RMQ call to send an invite email to the invitee

    Ok(())
}


/// Resend an invite for a task
pub async fn resend_invite(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_invite(state, session_user_id, view_id, task_id).await? {
        return Err(ServiceError::Forbidden("You do not have permission to invite for this task".into()));
    }

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

    // Resend the invite notification to the invitee (e.g., via email)
    // TODO: Make RMQ call to send an invite email to the invitee

    Ok(())
}
