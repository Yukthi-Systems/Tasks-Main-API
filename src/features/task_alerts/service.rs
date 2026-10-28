use super::dto::{CreateTaskAlertDto, EditTaskAlertDto, DeleteTaskAlertDto};
use crate::errors::{ServiceResult, ServiceError};
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Create a new task alert
pub async fn create_alert(state: &AppState, session_user_id: &Uuid, task_id: i64, create_task_alert_dto: CreateTaskAlertDto) -> ServiceResult<()> {
    // Create the task alert in the database
    let result = repository::create_task_alert(&state.pg_pool, session_user_id, task_id, &create_task_alert_dto).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to create task alert, maybe check if same task alert already exists".into()));
    }

    Ok(())
}


/// Delete a task alert
pub async fn delete_alert(state: &AppState, session_user_id: &Uuid, task_id: i64, delete_task_alert_dto: DeleteTaskAlertDto) -> ServiceResult<()> {
    // Delete the task alert from the database
    let result = repository::delete_task_alert(&state.pg_pool, session_user_id, task_id, &delete_task_alert_dto).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to delete task alert, maybe it does not exist".into()));
    }

    Ok(())
}


/// Edit a task alert
pub async fn edit_alert(state: &AppState, session_user_id: &Uuid, task_id: i64, edit_task_alert_dto: EditTaskAlertDto) -> ServiceResult<()> {
    // Edit the task alert in the database
    let result = repository::edit_task_alert(&state.pg_pool, session_user_id, task_id, &edit_task_alert_dto).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to edit task alert, maybe it does not exist".into()));
    }

    Ok(())
}
