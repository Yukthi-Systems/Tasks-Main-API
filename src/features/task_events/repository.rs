use super::dto::{CreateRecurringTaskDto, UpdateRecurringTaskDto, CreateTaskDto, UpdateTaskDto, TaskFilterDto};
use super::model::{RecurringTask, TaskOwnershipInfo, TaskInfo};
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
                &new_info.task_tag,
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
            &[owner_id, &task_tags],
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
            &[owner_id, &recurring_task_id, &task_tag],
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
                &new_info.task_tag,
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
            &[owner_id, &recurring_task_id, &task_tag],
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
                &new_task.task_tag,
                &new_task.task_status,
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
                end_at = $7,
                updated_at = NOW()
            WHERE task_id = $8
            "#,
            &[
                &updated_task.title,
                &updated_task.description,
                &updated_task.details,
                &updated_task.task_tag,
                &updated_task.task_status,
                &updated_task.start_at,
                &updated_task.end_at,
                &updated_task.task_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn get_task(db_pool: &PgPool, task_id: i64) -> PgResult<Option<TaskInfo>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                t.task_id,
                t.owner_id,
                t.created_by,
                t.parent_task_id,
                t.title,
                t.description,
                t.details,
                t.task_status,
                t.task_tag,
                t.start_at,
                t.end_at,
                t.created_at,
                t.updated_at,
                ARRAY(
                    SELECT ta.assignee_id
                    FROM task_assignees ta
                    WHERE ta.task_id = t.task_id
                ) AS assignee_ids,
                (
                    SELECT COUNT(*)
                    FROM tasks sub
                    WHERE sub.parent_task_id = t.task_id
                ) AS total_sub_tasks
            FROM tasks t
            WHERE t.task_id = $1
            "#,
            &[&task_id],
        )
        .await?;

    Ok(row.map(TaskInfo::from))
}


pub async fn delete_task(db_pool: &PgPool, task_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM tasks
            WHERE task_id = $1
            "#,
            &[&task_id],
        )
        .await?;

    Ok(result)
}


pub async fn get_child_tasks(db_pool: &PgPool, parent_task_id: i64, filter: &TaskFilterDto) -> PgResult<Vec<TaskInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                t.task_id,
                t.owner_id,
                t.created_by,
                t.parent_task_id,
                t.title,
                t.description,
                t.details,
                t.task_status,
                t.task_tag,
                t.start_at,
                t.end_at,
                t.created_at,
                t.updated_at,
                ARRAY(
                    SELECT ta.assignee_id
                    FROM task_assignees ta
                    WHERE ta.task_id = t.task_id
                ) AS assignee_ids,
                (
                    SELECT COUNT(*)
                    FROM tasks sub
                    WHERE sub.parent_task_id = t.task_id
                ) AS total_sub_tasks
            FROM tasks t
            WHERE t.parent_task_id = $1
            AND ($2::TEXT IS NULL OR t.title ILIKE '%' || $2 || '%')
            AND t.task_status = ANY($3)
            AND t.task_tag = ANY($4)
            AND t.start_at >= $5
            AND t.end_at <= $6
            AND (
                $7::TIMESTAMPTZ IS NULL
                OR (t.start_at, t.task_id) > ($7, $8)
            )
            ORDER BY t.start_at ASC, t.task_id ASC
            LIMIT $9
            "#,
            &[
                &parent_task_id,
                &filter.title_contains,
                &filter.any_of_task_status,
                &filter.any_of_task_tag,
                &filter.start_at,
                &filter.end_at,
                &filter.cursor_start_at,
                &filter.cursor_task_id,
                &filter.limit,
            ],
        )
        .await?;

    Ok(TaskInfo::from_rows(rows))
}


pub async fn get_all_root_tasks(db_pool: &PgPool, owner_id: &Uuid, filter: &TaskFilterDto, show_assigned: &bool) -> PgResult<Vec<TaskInfo>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                t.task_id,
                t.owner_id,
                t.created_by,
                t.parent_task_id,
                t.title,
                t.description,
                t.details,
                t.task_status,
                t.task_tag,
                t.start_at,
                t.end_at,
                t.created_at,
                t.updated_at,
                ARRAY(
                    SELECT ta.assignee_id
                    FROM task_assignees ta
                    WHERE ta.task_id = t.task_id
                ) AS assignee_ids,
                (
                    SELECT COUNT(*)
                    FROM tasks sub
                    WHERE sub.parent_task_id = t.task_id
                ) AS total_sub_tasks
            FROM tasks t
            WHERE
                (
                    t.owner_id = $1
                    OR
                    EXISTS (
                        SELECT 1
                        FROM task_assignees ta
                        WHERE ta.task_id = t.task_id
                        AND ta.assignee_id = $1
                        AND $10::BOOLEAN
                    )
                )
                AND t.parent_task_id IS NULL
                AND ($2::TEXT IS NULL OR t.title ILIKE '%' || $2 || '%')
                AND t.task_status = ANY($3)
                AND t.task_tag = ANY($4)
                AND t.start_at >= $5
                AND t.end_at <= $6
                AND (
                    $7::TIMESTAMPTZ IS NULL
                    OR (t.start_at, t.task_id) > ($7, $8)
                )
            ORDER BY t.start_at ASC, t.task_id ASC
            LIMIT $9
            "#,
            &[
                &owner_id,
                &filter.title_contains,
                &filter.any_of_task_status,
                &filter.any_of_task_tag,
                &filter.start_at,
                &filter.end_at,
                &filter.cursor_start_at,
                &filter.cursor_task_id,
                &filter.limit,
                &show_assigned,
            ],
        )
        .await?;

    Ok(TaskInfo::from_rows(rows))
}
