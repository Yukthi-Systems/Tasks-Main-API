use super::dto::{AddAssigneeDto, RemoveAssigneeDto};
use deadpool_postgres::Pool as PgPool;
use crate::errors::PgResult;
use uuid::Uuid;



pub async fn assign_user_to_task(db_pool: &PgPool, assignee_info: &AddAssigneeDto, org_id: &Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            INSERT INTO task_assignees (
                task_id,
                assignee_id
            )
            SELECT $1, $2
            WHERE EXISTS (
                SELECT 1
                FROM users u
                WHERE u.user_id = $2
                  AND u.organization_id = $3
            )
            AND EXISTS (
                SELECT 1
                FROM tasks t
                JOIN users owner ON owner.user_id = t.owner_id
                WHERE t.task_id = $1
                  AND owner.organization_id = $3
            )
            "#,
            &[
                &assignee_info.task_id,
                &assignee_info.assignee_id,
                &org_id,
            ],
        )
        .await?;

    Ok(result)
}


pub async fn remove_user_from_task(db_pool: &PgPool, assignee_info: &RemoveAssigneeDto) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM task_assignees
            WHERE task_id = $1
              AND assignee_id = $2
            "#,
            &[
                &assignee_info.task_id,
                &assignee_info.assignee_id,
            ],
        )
        .await?;

    Ok(result)
}
