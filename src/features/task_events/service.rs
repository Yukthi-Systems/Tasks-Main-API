use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto, CreateTaskDto, UpdateTaskDto};
use crate::features::task_views::{TaskTag, TaskStatus, get_task_view_permission};
use crate::cache::{get_redis_cache, set_redis_cache, delete_redis_cache};
use super::model::{RecurringTask, TaskOwnershipInfo};
use crate::errors::{ServiceResult, ServiceError};
use crate::state::AppState;
use super::repository;
use uuid::Uuid;


/// Get the ownership information of a task
async fn get_task_ownership_info(state: &AppState, task_id: i64) -> ServiceResult<TaskOwnershipInfo> {
    // Check if the ownership information of the task is cached in Redis
    let cache_key = format!("task:ownership:{}", task_id);
    let task_ownership_info: Option<TaskOwnershipInfo> = get_redis_cache(&state.redis_cache, &cache_key).await?;

    // See if the task has the session user as the owner or an assignee
    let task_ownership_info = if let Some(task_ownership_info) = task_ownership_info {
        task_ownership_info
    } else {
        let task_ownership_info = repository::get_task_ownership_info(&state.pg_pool, task_id).await?;
        if task_ownership_info.is_none() {
            return Err(ServiceError::BadRequest("Task does not exist".into()));
        }
        let task_ownership_info = task_ownership_info.unwrap();

        // Cache the result in Redis
        set_redis_cache(&state.redis_cache, &cache_key, &task_ownership_info, 1 * 60 * 60).await?;

        task_ownership_info
    };

    Ok(task_ownership_info)
}


/// Create a new recurring task for the session user and the specified view ID
pub async fn create_recurring_task(state: &AppState, session_user_id: &Uuid, task_info: &CreateRecurringTaskDto, view_id: i64) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to create a recurring task
    if !task_view_permission.can_create || !task_view_permission.show_recurring || !task_view_permission.check_tag(&task_info.task_tag) {
        return Err(ServiceError::Forbidden("You do not have permission to create a recurring task with this tag in this view".into()));
    }

    // Create the recurring task in the database
    let result = repository::create_recurring_task(&state.pg_pool, task_info, &task_view_permission.owner_id).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to create recurring task".into()));
    }

    Ok(())
}


/// List all recurring tasks for the session user and the specified view ID
pub async fn list_recurring_tasks(state: &AppState, session_user_id: &Uuid, view_id: i64, task_tags: Vec<TaskTag>) -> ServiceResult<Vec<RecurringTask>> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to view recurring tasks
    if !task_view_permission.show_recurring || !task_view_permission.check_all_tags(&task_tags) {
        return Err(ServiceError::Forbidden("You do not have permission to view recurring tasks with this tag in this view".into()));
    }

    // List the recurring tasks from the database
    let tasks = repository::list_recurring_tasks(&state.pg_pool, &task_view_permission.owner_id, &task_tags).await?;

    Ok(tasks)
}


/// Get one recurring task for the session user and the specified view ID and task ID
pub async fn get_one_recurring_task(state: &AppState, session_user_id: &Uuid, view_id: i64, recurring_task_id: i64, task_tag: TaskTag) -> ServiceResult<RecurringTask> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to view recurring tasks
    if !task_view_permission.show_recurring || !task_view_permission.check_tag(&task_tag) {
        return Err(ServiceError::Forbidden("You do not have permission to view recurring tasks with this tag in this view".into()));
    }

    // Get the recurring task from the database
    let task = repository::get_one_recurring_task(&state.pg_pool, &task_view_permission.owner_id, recurring_task_id, task_tag).await?;
    if task.is_none() {
        return Err(ServiceError::NotFound("Recurring task not found".into()));
    }

    Ok(task.unwrap())
}


/// Update a recurring task for the session user and the specified view ID and task ID
pub async fn update_recurring_task(state: &AppState, session_user_id: &Uuid, view_id: i64, new_info: &UpdateRecurringTaskDto) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to update recurring tasks
    if !task_view_permission.show_recurring || !task_view_permission.can_edit || !task_view_permission.check_tag(&new_info.task_tag) {
        return Err(ServiceError::Forbidden("You do not have permission to update recurring tasks in this view with the specified tag".into()));
    }

    // Update the recurring task in the database
    let result = repository::update_recurring_task(&state.pg_pool, &task_view_permission.owner_id, new_info).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Recurring task not found".into()));
    }

    Ok(())
}


/// Delete a recurring task for the session user and the specified view ID and task ID
pub async fn delete_recurring_task(state: &AppState, session_user_id: &Uuid, view_id: i64, recurring_task_id: i64, task_tag: TaskTag) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to delete recurring tasks
    if !task_view_permission.show_recurring || !task_view_permission.can_delete || !task_view_permission.check_tag(&task_tag) {
        return Err(ServiceError::Forbidden("You do not have permission to delete recurring tasks with this tag in this view".into()));
    }

    // Delete the recurring task from the database
    let result = repository::delete_recurring_task(&state.pg_pool, &task_view_permission.owner_id, recurring_task_id, task_tag).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Recurring task not found".into()));
    }

    Ok(())
}


/// Create a new task by the session user
pub async fn create_task(state: &AppState, session_user_id: &Uuid, view_id: i64, new_task: &CreateTaskDto) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to create tasks
    let is_sub_task_allowed = new_task.parent_task_id.is_none() || task_view_permission.show_subtasks;
    let can_create_task =
        is_sub_task_allowed
        && task_view_permission.can_create
        && task_view_permission.check_tag(&new_task.task_tag)
        && task_view_permission.check_status(&new_task.task_status);

    // If the user cannot create the task, return a forbidden error
    if !can_create_task {
        return Err(ServiceError::Forbidden("You do not have permission to create tasks in this view with the specified tag or status".into()));
    }

    // At this point, we have verified that the user has the necessary permissions to create the task
    // If the task has a parent, we need to ensure the parent task exists and the user has the necessary permissions
    if new_task.parent_task_id.is_some() {
        // Make sure the user has permission to create sub-tasks
        let parent_task_id = new_task.parent_task_id.unwrap();
        let task_ownership_info = get_task_ownership_info(&state, parent_task_id).await?;

        // Make sure the user can create sub-tasks under the parent task
        // We use the owner ID from the task view permission, because the session user might not be the owner of the parent task
        // this is to handle the shared view scenario where the session user might have no relationship with the parent task directly whatsoever
        if !task_ownership_info.can_create(&task_view_permission.owner_id) {
            return Err(ServiceError::Forbidden("You do not have permission to create sub-tasks under this parent task".into()));
        }
    }

    // Since all necessary checks have passed, proceed to create the task
    let result = repository::create_new_task(
        &state.pg_pool,
        &task_view_permission.owner_id,
        session_user_id,
        new_task,
    ).await?;

    // Check if the task creation was successful
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to create task".into()));
    }

    Ok(())
}


/// Update an existing task by the session user and the specified view ID and task ID
pub async fn update_task(state: &AppState, session_user_id: &Uuid, view_id: i64, updated_task: &UpdateTaskDto) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // See if the user has the necessary permissions to update tasks
    let can_update_task =
        task_view_permission.can_edit
        && task_view_permission.check_tag(&updated_task.task_tag)
        && task_view_permission.check_status(&updated_task.task_status);

    // If the user cannot update the task, return a forbidden error
    if !can_update_task {
        return Err(ServiceError::Forbidden("You do not have permission to update tasks in this view with the specified tag or status".into()));
    }

    // Fetch the task information for the task being updated
    let task_ownership_info = get_task_ownership_info(&state, updated_task.task_id).await?;

    // If the task is not a root task, ensure the user has permission to update it
    if !task_ownership_info.is_root_task {  // Not root task means it is a sub-task
        if !task_view_permission.show_subtasks {
            return Err(ServiceError::Forbidden("You do not have permission to update sub-tasks".into()));
        }
    }

    // Check if the task view user can edit the task
    if !task_ownership_info.can_edit(&task_view_permission.owner_id) {
        // Either the user is not the owner of the task or is not the creator of the task
        return Err(ServiceError::Forbidden("You do not have permission to edit this task".into()));
    }

    // Since all necessary checks have passed, proceed to update the task
    let result = repository::update_task(&state.pg_pool, updated_task).await?;

    // Check if the task update was successful
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to update task".into()));
    }

    Ok(())
}


/// Delete an existing task by the session user and the specified view ID and task ID
pub async fn delete_task(state: &AppState, session_user_id: &Uuid, view_id: i64, task_id: i64) -> ServiceResult<()> {
    // Get the task view permission for the user and view ID
    let task_view_permission = get_task_view_permission(&state, session_user_id, view_id).await?;

    // Fetch the task and see the Tag and Status for additional permission checks
    let task_info = repository::get_task(&state.pg_pool, task_id).await?;
    if task_info.is_none() {
        return Err(ServiceError::NotFound("Task not found or has been deleted".into()));
    }
    let task_info = task_info.unwrap();

    // Check the Tag and Status for additional permission checks
    let task_tag = &TaskTag::from_str(&task_info.task_tag).unwrap();
    let task_status = &TaskStatus::from_str(&task_info.task_status).unwrap();
    let can_delete_task =
        task_view_permission.can_delete
        && task_view_permission.check_tag(task_tag)
        && task_view_permission.check_status(task_status);
    if !can_delete_task {
        return Err(ServiceError::Forbidden("You do not have permission to delete this task based on its tag or status or your view permissions".into()));
    }

    // Fetch the task information for the task being deleted
    let task_ownership_info = get_task_ownership_info(&state, task_id).await?;

    // If the task is not a root task, ensure the user has permission to delete it
    if !task_ownership_info.is_root_task {  // Not root task means it is a sub-task
        if !task_view_permission.show_subtasks {
            return Err(ServiceError::Forbidden("You do not have permission to delete sub-tasks".into()));
        }
    }

    // Check if the task view user can edit the task
    if !task_ownership_info.can_delete(&task_view_permission.owner_id) {
        // Either the user is not the owner of the task or is not the creator of the task
        return Err(ServiceError::Forbidden("You do not have permission to delete this task".into()));
    }

    // Since all necessary checks have passed, proceed to delete the task
    let result = repository::delete_task(&state.pg_pool, task_id).await?;

    // Check if the task deletion was successful
    if result == 0 {
        return Err(ServiceError::BadRequest("Failed to delete task".into()));
    }

    // Remove the cached ownership information for the deleted task
    let cache_key = format!("task:ownership:{}", task_id);
    delete_redis_cache(&state.redis_cache, &cache_key).await?;

    Ok(())
}
