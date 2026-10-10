use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto};
use crate::features::task_views::TaskTag;
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
                rrule,
                task_tag
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            &[
                owner_id,
                &new_info.title,
                &new_info.description,
                &new_info.details,
                &new_info.rrule,
                &new_info.task_tag.as_str(),
            ],
        )
        .await?;

    Ok(result)
}


pub async fn list_recurring_tasks(db_pool: &PgPool, owner_id: &Uuid, task_tags: &[TaskTag]) -> PgResult<Vec<RecurringTask>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT recurring_task_id, owner_id, title, description, details, rrule, task_tag, created_at
            FROM recurring_tasks
            WHERE owner_id = $1 AND task_tag = ANY($2)
            "#,
            &[owner_id, &task_tags.iter().map(|t| t.as_str()).collect::<Vec<_>>()],
        )
        .await?;

    Ok(RecurringTask::from_rows(rows))
}


pub async fn get_one_recurring_task(db_pool: &PgPool, owner_id: &Uuid, recurring_task_id: i64, task_tag: TaskTag) -> PgResult<Option<RecurringTask>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT recurring_task_id, owner_id, title, description, details, rrule, task_tag, created_at
            FROM recurring_tasks
            WHERE owner_id = $1 AND recurring_task_id = $2 AND task_tag = $3
            "#,
            &[owner_id, &recurring_task_id, &task_tag.as_str()],
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
                rrule = $4,
                task_tag = $5
            WHERE owner_id = $6 AND recurring_task_id = $7
            "#,
            &[
                &new_info.title,
                &new_info.description,
                &new_info.details,
                &new_info.rrule,
                &new_info.task_tag.as_str(),
                owner_id,
                &new_info.recurring_task_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_recurring_task(db_pool: &PgPool, owner_id: &Uuid, recurring_task_id: i64, task_tag: TaskTag) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM recurring_tasks
            WHERE owner_id = $1 AND recurring_task_id = $2 AND task_tag = $3
            "#,
            &[owner_id, &recurring_task_id, &task_tag.as_str()],
        )
        .await?;

    Ok(result)
}
