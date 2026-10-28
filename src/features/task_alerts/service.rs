use crate::errors::{ServiceResult, ServiceError};
use super::dto::TaskAlertDto;
use crate::state::AppState;
use super::repository;
use uuid::Uuid;



/// Create a new task alert
pub async fn create_alert(state: &AppState, session_user_id: &Uuid, task_id: i64, task_alert_dto: TaskAlertDto) -> ServiceResult<()> {
    // Create the task alert in the database
    let result = repository::create_task_alert(&state.pg_pool, session_user_id, task_id, &task_alert_dto).await?;
    if result == 0 {
        return Err(ServiceError::NotFound("Failed to create task alert, maybe check if same task alert already exists".into()));
    }

    Ok(())
}
