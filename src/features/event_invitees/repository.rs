use deadpool_postgres::Pool as PgPool;
// use serde_json::Value as JsonValue;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn create_invite(db_pool: &PgPool, task_id: &i64, invitee_email: &str, random_validation_code: &Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO event_invites (
                task_id,
                invitee_email,
                validation_code,
                response_info
            ) VALUES ($1, $2, $3, '{}'::jsonb)
            "#,
            &[
                task_id,
                &invitee_email,
                &random_validation_code,
            ],
        )
        .await?;

    Ok(result)
}
