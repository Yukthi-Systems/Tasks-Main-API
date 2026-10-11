use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto, CreateTaskDto, UpdateTaskDto};
use super::model::{RecurringTask, TaskOwnershipInfo};
use crate::features::task_views::TaskTag;
use deadpool_postgres::Pool as PgPool;
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


pub async fn get_task_ownership_info(db_pool: &PgPool, task_id: i64) -> PgResult<Option<TaskOwnershipInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                t.task_id,
                t.owner_id,
                t.created_by,
                t.parent_task_id IS NULL AS is_root_task,
                COALESCE(
                    (
                        SELECT ARRAY_AGG(ta.assignee_id)
                        FROM task_assignees ta
                        WHERE ta.task_id = t.task_id
                    ),
                    ARRAY[]::UUID[]
                ) AS assignee_ids
            FROM tasks t
            WHERE t.task_id = $1
            "#,
            &[&task_id],
        )
        .await?;

    Ok(row.map(TaskOwnershipInfo::from))
}


pub async fn create_new_task(db_pool: &PgPool, owner_id: &Uuid, session_user_id: &Uuid, new_task: &CreateTaskDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO tasks (
                owner_id,
                title,
                description,
                details,
                task_tag,
                task_status,
                start_at,
                end_at,
                parent_task_id,
                created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            &[
                owner_id,
                &new_task.title,
                &new_task.description,
                &new_task.details,
                &new_task.task_tag.as_str(),
                &new_task.task_status.as_str(),
                &new_task.start_at,
                &new_task.end_at,
                &new_task.parent_task_id,
                session_user_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn update_task(db_pool: &PgPool, updated_task: &UpdateTaskDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE tasks
            SET
                title = $1,
                description = $2,
                details = $3,
                task_tag = $4,
                task_status = $5,
                start_at = $6,
                end_at = $7
            WHERE task_id = $8
            "#,
            &[
                &updated_task.title,
                &updated_task.description,
                &updated_task.details,
                &updated_task.task_tag.as_str(),
                &updated_task.task_status.as_str(),
                &updated_task.start_at,
                &updated_task.end_at,
                &updated_task.task_id,
            ],
        )
        .await?;

    Ok(result)
}
