use crate::features::task_events::get_task_ownership_info;
use crate::errors::{ServiceResult, ServiceError};
use super::model::PubEventInvitee;
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


/// Delete an invite for a task
pub async fn delete_invite(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, invitee_email: &str) -> ServiceResult<()> {
    // Validate if the user has permission to invite for the task
    if !can_delete_invite(state, session_user_id, view_id, task_id).await? {
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

    // Delete the invite in the database
    let result = repository::delete_invitee(
        &state.pg_pool,
        &task_id,
        invitee_email,
    ).await?;

    if result == 0 {
        return Err(ServiceError::NotFound("Failed to delete invite for task".into()));
    }

    // Send a notification to the invitee about the cancellation of the invite (e.g., via email)
    // TODO: Make RMQ call to send a cancellation email to the invitee
    // Use the existing_invitee information to personalize the cancellation email as needed

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
