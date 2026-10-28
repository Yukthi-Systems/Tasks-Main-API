use super::dto::{CreateTaskAlertDto, EditTaskAlertDto, DeleteTaskAlertDto};
use deadpool_postgres::Pool as PgPool;
use crate::errors::PgResult;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


pub async fn create_task_alert(db_pool: &PgPool, session_user_id: &Uuid, task_id: i64, create_task_alert_dto: &CreateTaskAlertDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO task_alerts (
                task_id,
                alert_recipient_id,
                alert_at,
                notify_via_email,
                notify_via_push
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            &[
                &task_id,
                &session_user_id,
                &create_task_alert_dto.alert_at,
                &create_task_alert_dto.notify_via_email,
                &create_task_alert_dto.notify_via_push,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_task_alert(db_pool: &PgPool, session_user_id: &Uuid, task_id: i64, delete_task_alert_dto: &DeleteTaskAlertDto) -> PgResult<u64> {
    let client = db_pool.get().await?;
    let result = client
        .execute(
            r#"
            DELETE FROM task_alerts
            WHERE task_id = $1
              AND alert_recipient_id = $2
              AND alert_at = $3
            "#,
            &[
                &task_id,
                &session_user_id,
                &delete_task_alert_dto.alert_at,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn edit_task_alert(db_pool: &PgPool, session_user_id: &Uuid, task_id: i64, edit_task_alert_dto: &EditTaskAlertDto) -> PgResult<u64> {
    let client = db_pool.get().await?;
    let result = client
        .execute(
            r#"
            UPDATE task_alerts
            SET
                alert_at = $4,
                notify_via_email = $5,
                notify_via_push = $6
            WHERE task_id = $1
              AND alert_recipient_id = $2
              AND alert_at = $3
            "#,
            &[
                &task_id,
                &session_user_id,
                &edit_task_alert_dto.current_alert_at,
                &edit_task_alert_dto.new_alert_at,
                &edit_task_alert_dto.notify_via_email,
                &edit_task_alert_dto.notify_via_push,
            ],
        )
        .await?;

    Ok(result)
}
