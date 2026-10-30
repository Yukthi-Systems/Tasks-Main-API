use crate::features::task_events::{get_task_ownership_info, get_task_info};
use crate::features::task_views::get_task_view_permission;
use super::dto::{AddAssigneeDto, RemoveAssigneeDto};
use crate::errors::{ServiceResult, ServiceError};
use crate::cache::delete_redis_cache;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Validate if a user can assign another user to a task
async fn validate_user_assignment(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64, is_assignment: bool) -> ServiceResult<()> {
    // Get the task information from the database
    let task_info = get_task_info(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // Fetch the task ownership information for the task
    let task_ownership_info = get_task_ownership_info(&state, task_id).await?;

    // If its assign operation, any one can assign
    if !is_assignment && !task_ownership_info.can_edit(&task_view_permission.owner_id) {
        // Only the Owner or Creator of the task can remove an assignee
        return Err(ServiceError::Forbidden("You do not have permission to remove a user from this task".into()));
    }

    // See if the session user is the owner of the task or an assignee
    if !task_ownership_info.can_create(&task_view_permission.owner_id) {
        return Err(ServiceError::Forbidden("You do not have create permission for this task".into()));
    }

    let can_access_child_task = if task_info.parent_task_id.is_some() {
        task_view_permission.show_subtasks
    } else {
        true
    };
    let can_assign_user =
        can_access_child_task
        && task_view_permission.can_edit
        && task_view_permission.check_status(&task_info.task_status)
        && task_view_permission.check_tag(&task_info.task_tag);
    
    if !can_assign_user {
        return Err(ServiceError::Forbidden("You do not have permission to assign a user to this task".into()));
    }

    Ok(())
}


/// Assign a user to a task
pub async fn assign_user(state: &AppState, org_id: &Uuid, session_user_id: &Uuid, assignee_info: &AddAssigneeDto, view_id: i64) -> ServiceResult<()> {
    // Validate if the user has permission to assign another user to the task
    validate_user_assignment(state, session_user_id, view_id, assignee_info.task_id, true).await?;

    // Assign the user to the task in the database
    let result = repository::assign_user_to_task(&state.pg_pool, assignee_info, org_id).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to assign user to task".into()));
    }

    // Invalidate the cache for the task since its assignees have changed
    let cache_key = format!("task:ownership:{}", assignee_info.task_id);
    delete_redis_cache(&state.redis_cache, &cache_key).await?;

    Ok(())
}


/// Remove a user from a task
pub async fn remove_user(state: &AppState, session_user_id: &Uuid, assignee_info: &RemoveAssigneeDto, view_id: i64) -> ServiceResult<()> {
    // Validate if the user has permission to remove another user from the task
    validate_user_assignment(state, session_user_id, view_id, assignee_info.task_id, false).await?;

    // Remove the user from the task in the database
    // Can remove anyone for that Task, since the task is either owned or created by the session user / Shared with the session user
    let result = repository::remove_user_from_task(&state.pg_pool, assignee_info).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to remove user from task".into()));
    }

    // Invalidate the cache for the task since its assignees have changed
    let cache_key = format!("task:ownership:{}", assignee_info.task_id);
    delete_redis_cache(&state.redis_cache, &cache_key).await?;

    Ok(())
}
