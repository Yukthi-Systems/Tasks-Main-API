use super::model::{EventInvitee, PubEventInvitee};
use deadpool_postgres::Pool as PgPool;
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


pub async fn get_invitee(db_pool: &PgPool, task_id: &i64, invitee_email: &str) -> PgResult<Option<EventInvitee>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT task_id, invitee_email, validation_code, response_status, response_info, invited_at
            FROM event_invites
            WHERE task_id = $1 AND invitee_email = $2
            "#,
            &[task_id, &invitee_email],
        )
        .await?;

    Ok(row.map(EventInvitee::from))
}


pub async fn delete_invitee(db_pool: &PgPool, task_id: &i64, invitee_email: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM event_invites
            WHERE task_id = $1 AND invitee_email = $2
            "#,
            &[task_id, &invitee_email],
        )
        .await?;

    Ok(result)
}


pub async fn list_all_invitees(db_pool: &PgPool, task_id: &i64) -> PgResult<Vec<PubEventInvitee>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT invitee_email, response_status, response_info, invited_at
            FROM event_invites
            WHERE task_id = $1
            "#,
            &[task_id],
        )
        .await?;

    Ok(PubEventInvitee::from_rows(rows))
}
