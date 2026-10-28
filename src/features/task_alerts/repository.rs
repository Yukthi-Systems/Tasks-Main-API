use deadpool_postgres::Pool as PgPool;
use super::dto::TaskAlertDto;
use crate::errors::PgResult;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


pub async fn create_task_alert(db_pool: &PgPool, session_user_id: &Uuid, task_id: i64, task_alert_dto: &TaskAlertDto) -> PgResult<u64> {
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
                &task_alert_dto.alert_at,
                &task_alert_dto.notify_via_email,
                &task_alert_dto.notify_via_push,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_task_alert(db_pool: &PgPool, session_user_id: &Uuid, task_id: i64, alert_at: &ChronoUtc) -> PgResult<u64> {
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
                &alert_at,
            ],
        )
        .await?;

    Ok(result)
}
