use super::dto::{AddAssigneeDto};
use crate::features::task_views::{TaskTag, TaskStatus, get_task_view_permission};
use crate::features::task_events::{get_task_ownership_info, get_task_info};
use crate::cache::{get_redis_cache, set_redis_cache, delete_redis_cache};
use super::model::{RecurringTask, TaskOwnershipInfo, TaskInfo};
use crate::errors::{ServiceResult, ServiceError};
use crate::state::AppState;
// use super::repository;
use uuid::Uuid;



/// Assign a user to a task
pub async fn assign_user(state: &AppState, session_user_id: &Uuid, task_info: &AddAssigneeDto, view_id: i64) -> ServiceResult<()> {
    // Fetch the task information for the task that the user is trying to assign someone to
    let task_ownership_info = get_task_ownership_info(&state, task_info.task_id).await?;

    // See if the session user is the owner of the task or an assignee
    if !task_ownership_info.can_create(session_user_id) {
        return Err(ServiceError::Forbidden("You do not have permission to assign a user to this task".into()));
    }

    // Fetch the task information for the task that the user is trying to assign someone to
    let task_info = get_task_info(&state.pg_pool, task_info.task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found".into()));
    }
    let task_info = task_info.unwrap();

    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // Check if the user has the necessary permissions to assign a user to this task based on the task view permissions
    let is_task_a_child_task = task_info.parent_task_id.is_some();
    let can_access_child_task = if is_task_a_child_task {
        task_view_permission.show_subtasks
    } else {
        true
    };

    let can_assign_user =
        can_access_child_task
        && task_view_permission.check_status(&task_info.task_status)
        && task_view_permission.check_tag(&task_info.task_tag);

    

    // See if the user has the necessary permissions to create a recurring task
    if !task_view_permission.can_create {
        return Err(ServiceError::Forbidden("You do not have permission to assign a user to this task".into()));
    }

    // Create the recurring task in the database
    // let result = repository::create_recurring_task(&state.pg_pool, task_info, &task_view_permission.owner_id).await?;
    // if result == 0 {
    //     return Err(ServiceError::NotFound("Failed to create recurring task".into()));
    // }

    Ok(())
}
