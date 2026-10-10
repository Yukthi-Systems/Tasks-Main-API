use super::dto::{CreateTaskViewDto, TaskViewDbDto, UpdateTaskViewDto, CreateSharedTaskViewDto};
use super::model::{TaskViewPermission, SharedTaskView};
use deadpool_postgres::Pool as PgPool;
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
                task_tag_filter,
                show_recurring,
                show_comments,
                show_subtasks,
                show_assigned
            )
            VALUES (
                $1, $2, $3, $4,
                ARRAY(SELECT unnest($5::text[])::task_status),
                ARRAY(SELECT unnest($6::text[])::task_tag),
                $7, $8, $9, $10
            )
            "#,
        &[
            &user_id,
            &new_info.view_name,
            &new_info.description,
            &new_info.ui_info,
            &new_info.status_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            &new_info.task_tag_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
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
            ui_info, status_filter::TEXT[], task_tag_filter::TEXT[], show_recurring, show_comments, show_subtasks, show_assigned
            FROM task_views
            WHERE owner_id = $1
            "#,
            &[user_id],
        )
        .await?;

    Ok(TaskViewDbDto::from_rows(rows))
}


pub async fn list_my_shared_views(db_pool: &PgPool, user_id: &Uuid) -> PgResult<Vec<TaskViewDbDto>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT tv.view_id, tv.owner_id, tv.view_name, tv.description,
            tv.ui_info, tv.status_filter::TEXT[], tv.task_tag_filter::TEXT[], tv.show_recurring, tv.show_comments, tv.show_subtasks, tv.show_assigned
            FROM task_views tv
            INNER JOIN shared_views sv
                ON sv.view_id = tv.view_id
            WHERE sv.user_id = $1
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
            ui_info, status_filter::TEXT[], task_tag_filter::TEXT[], show_recurring, show_comments, show_subtasks, show_assigned
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
                tv.status_filter::TEXT[] AS status_filter,
                tv.task_tag_filter::TEXT[] AS task_tag_filter,
                tv.show_recurring,
                tv.show_comments,
                tv.show_subtasks,
                tv.show_assigned,

                -- Owners have all permissions
                (tv.owner_id = $1 OR COALESCE(sv.can_create, FALSE)) AS can_create,
                (tv.owner_id = $1 OR COALESCE(sv.can_edit, FALSE)) AS can_edit,
                (tv.owner_id = $1 OR COALESCE(sv.can_delete, FALSE)) AS can_delete,
                (tv.owner_id = $1) AS is_owner

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
                task_tag_filter = $5,
                show_recurring = $6,
                show_comments = $7,
                show_subtasks = $8,
                show_assigned = $9
            WHERE owner_id = $10 AND view_id = $11
            "#,
            &[
                &new_info.view_name,
                &new_info.description,
                &new_info.ui_info,
                &new_info.status_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                &new_info.task_tag_filter.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
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


pub async fn create_shared_task_view(db_pool: &PgPool, organization_id: &Uuid, user_id: &Uuid, new_info: &CreateSharedTaskViewDto) -> PgResult<u64> {
    if new_info.user_id == *user_id {
        // The user cannot create a shared task view for themselves
        return Ok(0);
    }

    let client = db_pool.get().await?;

    // Check if the user is valid for the given organization before creating the shared task view
    let is_valid_row = client
        .query_one(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM users
                WHERE organization_id = $1 AND user_id = $2
            )
            "#,
            &[organization_id, user_id],
        )
        .await?;

    let is_valid: bool = is_valid_row.get(0);
    if !is_valid {
        // The user is not valid for the given organization, so we return 0 to indicate failure
        return Ok(0);
    }

    // Check if the user owns the task view before creating the shared task view
    let owns_view_row = client
        .query_one(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM task_views
                WHERE owner_id = $1 AND view_id = $2
            )
            "#,
            &[user_id, &new_info.view_id],
        )
        .await?;

    let owns_view: bool = owns_view_row.get(0);
    if !owns_view {
        // The user does not own the task view, so we return 0 to indicate failure
        return Ok(0);
    }

    let result = client
        .execute(
            r#"
            INSERT INTO shared_views (view_id, user_id, share_notes, ui_info, can_create, can_edit, can_delete)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            &[
                &new_info.view_id,
                &new_info.user_id,
                &new_info.share_notes,
                &new_info.ui_info,
                &new_info.can_create,
                &new_info.can_edit,
                &new_info.can_delete,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_shared_task_view(db_pool: &PgPool, owner_id: &Uuid, shared_view_id: i64, user_id: &Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM shared_views AS sv
            USING task_views AS tv
            WHERE sv.view_id = tv.view_id
                AND tv.owner_id = $1
                AND sv.view_id = $2
                AND sv.user_id = $3
            "#,
            &[owner_id, &shared_view_id, user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_shared_task_view(db_pool: &PgPool, owner_id: &Uuid, new_info: &CreateSharedTaskViewDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE shared_views AS sv
            SET share_notes = $3,
                ui_info = $4,
                can_create = $5,
                can_edit = $6,
                can_delete = $7
            FROM task_views AS tv
            WHERE sv.view_id = tv.view_id
                AND tv.owner_id = $1
                AND sv.view_id = $2
            "#,
            &[
                owner_id,
                &new_info.view_id,
                &new_info.share_notes,
                &new_info.ui_info,
                &new_info.can_create,
                &new_info.can_edit,
                &new_info.can_delete,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn list_all_shared_view_users(db_pool: &PgPool, view_id: i64) -> PgResult<Vec<SharedTaskView>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                view_id,
                user_id,
                share_notes,
                ui_info,
                can_create,
                can_edit,
                can_delete,
                shared_at
            FROM shared_views
            WHERE view_id = $1
            "#,
            &[&view_id],
        )
        .await?;

    Ok(SharedTaskView::from_rows(rows))
}


pub async fn get_one_shared_view_user_info(db_pool: &PgPool, view_id: i64, user_id: &Uuid) -> PgResult<Option<SharedTaskView>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                view_id,
                user_id,
                share_notes,
                ui_info,
                can_create,
                can_edit,
                can_delete,
                shared_at
            FROM shared_views
            WHERE view_id = $1
                AND user_id = $2
            "#,
            &[&view_id, user_id],
        )
        .await?;

    Ok(row.map(SharedTaskView::from))
}
