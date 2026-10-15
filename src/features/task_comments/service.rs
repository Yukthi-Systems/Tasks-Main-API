use crate::features::task_events::{get_task_ownership_info, get_task_info};
use crate::features::task_views::get_task_view_permission;
use crate::errors::{ServiceResult, ServiceError};
use super::dto::ListCommentsDto;
use super::model::TaskComment;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Validate if a user has permission to comment on a task
async fn validate_user_comment(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, create_new_task: bool) -> ServiceResult<()> {
    // Get the task information from the database
    let task_info = get_task_info(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // If creating a new task, check if the user has the necessary ownership permissions
    if create_new_task {
        // Fetch the task ownership information for the task
        let task_ownership_info = get_task_ownership_info(&state, task_id).await?;

        // See if the session user is the owner of the task or an assignee
        if !task_ownership_info.can_create(&task_view_permission.owner_id) {
            return Err(ServiceError::Forbidden("You do not have permission to comment on this task".into()));
        }
    }

    let can_access_child_task = if task_info.parent_task_id.is_some() {
        task_view_permission.show_subtasks
    } else {
        true
    };
    let can_comment =
        can_access_child_task
        && task_view_permission.show_comments
        && task_view_permission.check_status(&task_info.task_status)
        && task_view_permission.check_tag(&task_info.task_tag);

    if !can_comment {
        return Err(ServiceError::Forbidden("You do not have permission to comment on this task".into()));
    }

    Ok(())
}


/// Create a comment for a task
pub async fn create_comment(state: &AppState, org_id: &Uuid, session_user_id: &Uuid, view_id: i64, task_id: i64, comment_content: &str) -> ServiceResult<()> {
    // Validate if the user has permission to create a comment for the task
    validate_user_comment(state, session_user_id, view_id, task_id, true).await?;

    // Create the comment in the database
    let result = repository::create_comment(&state.pg_pool, org_id, &task_id, session_user_id, comment_content).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to create comment for task".into()));
    }

    Ok(())
}


/// Add reaction to a comment (Any one can do it, with basic permissions)
pub async fn add_reaction(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, comment_id: i64, reaction_data: &serde_json::Value) -> ServiceResult<()> {
    // Validate if the user has permission to comment on the task associated with the comment
    validate_user_comment(state, session_user_id, view_id, task_id, true).await?;

    // Add the reaction to the comment in the database
    let result = repository::add_reaction_to_comment(&state.pg_pool, &comment_id, &task_id, reaction_data).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to add reaction to comment".into()));
    }

    Ok(())
}


/// Add reaction to a comment (Any one can do it, with basic permissions)
pub async fn list_comments(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, list_query: ListCommentsDto) -> ServiceResult<Vec<TaskComment>> {
    // Validate if the user has permission to comment on the task associated with the comment
    validate_user_comment(state, session_user_id, view_id, task_id, false).await?;

    // List the comments from the database
    let result = repository::list_comments(&state.pg_pool, &task_id, list_query).await?;

    Ok(result)
}


/// Get a specific comment (Any one can do it, with basic permissions)
pub async fn get_comment(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, comment_id: i64) -> ServiceResult<TaskComment> {
    // Validate if the user has permission to comment on the task associated with the comment
    validate_user_comment(state, session_user_id, view_id, task_id, false).await?;

    // Get the comment from the database
    let result = repository::get_comment(&state.pg_pool, &task_id, &comment_id).await?;
    if result.is_none() {
        return Err(ServiceError::NotFound("Comment not found for the task".into()));
    }

    Ok(result.unwrap())
}
