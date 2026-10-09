use super::dto::{CreateTaskViewDto, TaskViewDbDto, UpdateTaskViewDto};
use deadpool_postgres::Pool as PgPool;
use super::model::TaskViewPermission;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn create_task_view(db_pool: &PgPool, user_id: &Uuid, new_info: &CreateTaskViewDto) -> PgResult<()> {
    let client = db_pool.get().await?;

    // Create a user if not exists, then create the session with the provided details
    client
        .execute(
            r#"
            INSERT INTO task_views (
                owner_id,
                view_name,
                description,
                ui_info,
                status_filter,
                show_recurring,
                show_comments,
                show_subtasks,
                show_assigned
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (owner_id, view_name) DO NOTHING
            "#,
        &[
            &user_id,
            &new_info.view_name,
            &new_info.description,
            &new_info.ui_info,
            &new_info.status_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            &new_info.show_recurring,
            &new_info.show_comments,
            &new_info.show_subtasks,
            &new_info.show_assigned,
        ],
    )
    .await?;

    Ok(())
}


pub async fn list_task_views(db_pool: &PgPool, user_id: &Uuid) -> PgResult<Vec<TaskViewDbDto>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT view_id, owner_id, view_name, description,
            ui_info, status_filter, show_recurring, show_comments, show_subtasks, show_assigned
            FROM task_views
            WHERE owner_id = $1
            "#,
            &[user_id],
        )
        .await?;

    Ok(TaskViewDbDto::from_rows(rows))
}


pub async fn get_task_view(db_pool: &PgPool, user_id: &Uuid, view_id: i64) -> PgResult<Option<TaskViewDbDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT view_id, owner_id, view_name, description,
            ui_info, status_filter, show_recurring, show_comments, show_subtasks, show_assigned
            FROM task_views
            WHERE owner_id = $1 AND view_id = $2
            "#,
            &[user_id, &view_id],
        )
        .await?;

    Ok(row.map(TaskViewDbDto::from))
}


pub async fn check_task_view_ownership(db_pool: &PgPool, user_id: &Uuid, view_id: i64) -> PgResult<Option<TaskViewPermission>> {
    let client = db_pool.get().await?;

    // Null means the task view has not been found / User is not the owner + not in shared views
    let row = client
        .query_opt(
            r#"
            SELECT
                tv.view_id,
                tv.owner_id,
                COALESCE(tv.status_filter, ARRAY[]::task_status[])::TEXT[] AS status_filter,
                tv.show_recurring,
                tv.show_comments,
                tv.show_subtasks,
                tv.show_assigned,

                -- Owners have all permissions
                (tv.owner_id = $1 OR COALESCE(sv.can_create, FALSE)) AS can_create,
                (tv.owner_id = $1 OR COALESCE(sv.can_edit, FALSE)) AS can_edit,
                (tv.owner_id = $1 OR COALESCE(sv.can_delete, FALSE)) AS can_delete

            FROM task_views tv
            LEFT JOIN shared_views sv
                ON sv.view_id = tv.view_id
            AND sv.user_id = $1

            WHERE tv.view_id = $2
            AND (
                tv.owner_id = $1
                OR sv.user_id IS NOT NULL
            )
            LIMIT 1
            "#,
            &[user_id, &view_id],
        )
        .await?;

    Ok(row.map(TaskViewPermission::from))
}


pub async fn update_task_view(db_pool: &PgPool, user_id: &Uuid, new_info: &UpdateTaskViewDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE task_views
            SET
                view_name = $1,
                description = $2,
                ui_info = $3,
                status_filter = $4,
                show_recurring = $5,
                show_comments = $6,
                show_subtasks = $7,
                show_assigned = $8
            WHERE owner_id = $9 AND view_id = $10
            "#,
            &[
                &new_info.view_name,
                &new_info.description,
                &new_info.ui_info,
                &new_info.status_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                &new_info.show_recurring,
                &new_info.show_comments,
                &new_info.show_subtasks,
                &new_info.show_assigned,
                &user_id,
                &new_info.view_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_task_view(db_pool: &PgPool, user_id: &Uuid, view_id: i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM task_views
            WHERE owner_id = $1 AND view_id = $2
            "#,
            &[user_id, &view_id],
        )
        .await?;

    Ok(result)
}
