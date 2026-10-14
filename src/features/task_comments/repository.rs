use super::dto::{AddAssigneeDto, RemoveAssigneeDto};
use deadpool_postgres::Pool as PgPool;
use serde_json::Value as JsonValue;
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
            SET content = $4
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
