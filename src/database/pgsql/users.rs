use crate::features::users::{SessionUser, UserInfoDbDto};
use deadpool_postgres::Pool as PgPool;
use crate::errors::PgResult;
use uuid::Uuid;


pub async fn create_user_session(db_pool: &PgPool, user_session: &SessionUser) -> PgResult<()> {
    let client = db_pool.get().await?;

    // Create a user if not exists, then create the session with the provided details
    client
        .execute(
            r#"
            WITH upsert_user AS (
                INSERT INTO users (
                    user_id,
                    email,
                    domain,

                    organization_id,
                    organization_name,

                    private_info,
                    public_info
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (email) DO UPDATE
                SET
                    domain = EXCLUDED.domain,
                    organization_id = EXCLUDED.organization_id,
                    organization_name = EXCLUDED.organization_name,
                    private_info = EXCLUDED.private_info,
                    public_info = EXCLUDED.public_info
                RETURNING user_id
            )

            INSERT INTO sessions (
                user_id,
                refresh_token,
                sso_token
            )
            SELECT 
                user_id,
                $8,     -- refresh_token
                $9     -- sso_token
            FROM upsert_user
            "#,
        &[
            &user_session.user_id,
            &user_session.email,
            &user_session.domain_name,
            &user_session.organization_id,
            &user_session.organization_name,
            &serde_json::json!({}), // private_info
            &serde_json::json!({}), // public_info
            &user_session.refresh_token,
            &user_session.sso_token,
        ],
    )
    .await?;

    Ok(())
}


pub async fn check_user_session(db_pool: &PgPool, refresh_token: &Uuid, sso_token: &str, user_id: &Uuid) -> PgResult<bool> {
    let client = db_pool.get().await?;

    let row = client
        .query_one(
            r#"
            SELECT EXISTS (
                SELECT 1
                FROM sessions s
                INNER JOIN users u
                    ON u.user_id = s.user_id
                WHERE s.user_id = $1
                  AND s.refresh_token = $2
                  AND s.sso_token = $3
                  AND s.expires_at > CURRENT_TIMESTAMP
            )
            "#,
            &[user_id, refresh_token, &sso_token],
        )
        .await?;

    Ok(row.get(0))
}


pub async fn delete_all_expired_sessions(db_pool: &PgPool) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM sessions
            WHERE expires_at <= CURRENT_TIMESTAMP
            "#,
            &[],
        )
        .await?;

    Ok(result)
}


pub async fn delete_user_session(db_pool: &PgPool, refresh_token: &Uuid) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            DELETE FROM sessions
            WHERE refresh_token = $1
            "#,
            &[refresh_token],
        )
        .await?;

    Ok(result)
}


pub async fn replace_fcm_token(db_pool: &PgPool, user_id: &Uuid, new_fcm_token: &str) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE sessions
            SET fcm_token = $1
            WHERE user_id = $2
            "#,
            &[&new_fcm_token, &user_id],
        )
        .await?;

    Ok(result)
}


pub async fn get_user_info(db_pool: &PgPool, user_id: &Uuid) -> PgResult<Option<UserInfoDbDto>> {
    let client = db_pool.get().await?;

    let row = client
        .query_opt(
            r#"
            SELECT 
                user_id,
                email,
                domain,
                organization_id,
                organization_name,
                private_info,
                public_info,
                is_external_sharing_enabled,
                created_at
            FROM users
            WHERE user_id = $1
            "#,
            &[user_id],
        )
        .await?;

    Ok(row.map(UserInfoDbDto::from))
}


pub async fn update_public_info(db_pool: &PgPool, user_id: &Uuid, new_info: &serde_json::Value) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE users
            SET public_info = $1
            WHERE user_id = $2
            "#,
            &[new_info, user_id],
        )
        .await?;

    Ok(result)
}


pub async fn update_private_info(db_pool: &PgPool, user_id: &Uuid, new_info: &serde_json::Value) -> PgResult<u64> {
    let client = db_pool.get().await?;

    let result = client
        .execute(
            r#"
            UPDATE users
            SET private_info = $1
            WHERE user_id = $2
            "#,
            &[new_info, user_id],
        )
        .await?;

    Ok(result)
}
