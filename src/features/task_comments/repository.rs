use deadpool_postgres::Pool as PgPool;
use serde_json::Value as JsonValue;
use super::dto::ListCommentsDto;
use super::model::TaskComment;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn create_comment(db_pool: &PgPool, org_id: &Uuid, task_id: &i64, session_user_id: &Uuid, comment_content: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO task_comments (
                task_id,
                author_id,
                content,
                reactions
            )
            SELECT
                t.task_id,
                u.user_id,
                $4,
                '{}'::jsonb
            FROM tasks t
            JOIN users owner
                ON owner.user_id = t.owner_id
            JOIN users u
                ON u.user_id = $2
            WHERE t.task_id = $1
              AND owner.organization_id = $3
              AND u.organization_id = $3
            "#,
            &[
                task_id,
                session_user_id,
                org_id,
                &comment_content,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn delete_comment(db_pool: &PgPool, session_user_id: &Uuid, task_id: &i64, comment_id: &i64) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM task_comments
            WHERE task_id = $1
              AND comment_id = $2
              AND author_id = $3
            "#,
            &[
                task_id,
                comment_id,
                session_user_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn add_reaction_to_comment(db_pool: &PgPool, comment_id: &i64, task_id: &i64, reaction_data: &JsonValue) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE task_comments
            SET reactions = reactions || $3::jsonb
            WHERE comment_id = $1
              AND task_id = $2
            "#,
            &[
                comment_id,
                task_id,
                &reaction_data,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn update_comment(db_pool: &PgPool, session_user_id: &Uuid, task_id: &i64, comment_id: &i64, comment_content: &String) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE task_comments
            SET content = $4,
                updated_at = NOW()
            WHERE task_id = $1
              AND comment_id = $2
              AND author_id = $3
            "#,
            &[
                task_id,
                comment_id,
                session_user_id,
                comment_content,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn list_comments(db_pool: &PgPool, task_id: &i64, list_query: ListCommentsDto) -> PgResult<Vec<TaskComment>> {
    let client = db_pool.get().await?;

    let rows = client
        .query(
            r#"
            SELECT
                comment_id,
                task_id,
                author_id,
                content,
                reactions,
                created_at,
                updated_at
            FROM task_comments
            WHERE task_id = $1
            ORDER BY
                CASE WHEN $2 THEN created_at END ASC,
                CASE WHEN NOT $2 THEN created_at END DESC,
                comment_id ASC
            LIMIT $3
            OFFSET $4
            "#,
            &[
                task_id,
                &list_query.created_at_ascending,
                &list_query.limit,
                &list_query.offset,
            ],
        )
        .await?;

    Ok(TaskComment::from_rows(rows))
}


pub async fn get_comment(db_pool: &PgPool, task_id: &i64, comment_id: &i64) -> PgResult<Option<TaskComment>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT
                comment_id,
                task_id,
                author_id,
                content,
                reactions,
                created_at,
                updated_at
            FROM task_comments
            WHERE task_id = $1
              AND comment_id = $2
            "#,
            &[
                task_id,
                comment_id,
            ],
        )
        .await?;

    Ok(row.map(TaskComment::from))
}
