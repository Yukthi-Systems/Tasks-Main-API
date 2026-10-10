use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto};
use crate::features::task_views::{TaskTag, get_task_view_permission};
use crate::errors::{ServiceResult, ServiceError};
use super::model::RecurringTask;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



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
