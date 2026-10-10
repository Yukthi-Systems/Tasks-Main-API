use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto};
use deadpool_postgres::Pool as PgPool;
use super::model::RecurringTask;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn create_recurring_task(db_pool: &PgPool, new_info: &CreateRecurringTaskDto, owner_id: &Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO recurring_tasks (
                owner_id,
                title,
                description,
                details,
                rrule
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
            &[
                owner_id,
                &new_info.title,
                &new_info.description,
                &new_info.details,
                &new_info.rrule,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn list_recurring_tasks(db_pool: &PgPool, owner_id: &Uuid) -> PgResult<Vec<RecurringTask>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT recurring_task_id, owner_id, title, description, details, rrule, created_at
            FROM recurring_tasks
            WHERE owner_id = $1
            "#,
            &[owner_id],
        )
        .await?;

    Ok(RecurringTask::from_rows(rows))
}


pub async fn get_one_recurring_task(db_pool: &PgPool, owner_id: &Uuid, recurring_task_id: i64) -> PgResult<Option<RecurringTask>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT recurring_task_id, owner_id, title, description, details, rrule, created_at
            FROM recurring_tasks
            WHERE owner_id = $1 AND recurring_task_id = $2
            "#,
            &[owner_id, &recurring_task_id],
        )
        .await?;

    Ok(row.map(RecurringTask::from))
}


pub async fn update_recurring_task(db_pool: &PgPool, owner_id: &Uuid, new_info: &UpdateRecurringTaskDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE recurring_tasks
            SET title = $1,
                description = $2,
                details = $3,
                rrule = $4
            WHERE owner_id = $5 AND recurring_task_id = $6
            "#,
            &[
                &new_info.title,
                &new_info.description,
                &new_info.details,
                &new_info.rrule,
                owner_id,
                &new_info.recurring_task_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_recurring_task(db_pool: &PgPool, owner_id: &Uuid, recurring_task_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM recurring_tasks
            WHERE owner_id = $1 AND recurring_task_id = $2
            "#,
            &[owner_id, &recurring_task_id],
        )
        .await?;

    Ok(result)
}
