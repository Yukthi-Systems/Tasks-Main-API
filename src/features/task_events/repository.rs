use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto, CreateTaskDto};
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


pub async fn create_task(db_pool: &PgPool, owner_id: &Uuid, new_task: &CreateTaskDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO tasks (owner_id, parent_task_id, title, description, details, task_status, task_tag, start_at, end_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            &[
                owner_id,
                &new_task.parent_task_id,
                &new_task.title,
                &new_task.description,
                &new_task.details,
                &new_task.task_status.as_str(),
                &new_task.task_tag.as_str(),
                &new_task.start_at,
                &new_task.end_at,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn check_parent_task_owner_or_assignee(db_pool: &PgPool, session_user_id: &Uuid, parent_task_id: i64) -> PgResult<Option<bool>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT CASE
                WHEN t.owner_id = $1 THEN TRUE
                WHEN EXISTS (
                    SELECT 1
                    FROM task_assignees ta
                    WHERE ta.task_id = t.task_id
                      AND ta.assignee_id = $1
                ) THEN FALSE
                ELSE NULL
            END AS permission
            FROM tasks t
            WHERE t.task_id = $2
              AND (
                  t.owner_id = $1
                  OR EXISTS (
                      SELECT 1
                      FROM task_assignees ta
                      WHERE ta.task_id = t.task_id
                        AND ta.assignee_id = $1
                  )
              )
            "#,
            &[session_user_id, &parent_task_id],
        )
        .await?;

    // True = session user is the owner
    // False = session user is an assignee
    // NULL = session user has no permission

    Ok(row.map(|r| r.get::<_, bool>("permission")))
}
