use crate::database::users::delete_all_expired_sessions;
use deadpool_postgres::Pool as PgPool;
use std::time::Duration;


const OLD_SESSIONS_CLEANUP_INTERVAL: u64 = 3 * 60 * 60;


pub async fn delete_old_sessions(pg_pool: PgPool) {
    tracing::info!("Starting to delete old sessions job");

    let mut interval = tokio::time::interval(Duration::from_secs(OLD_SESSIONS_CLEANUP_INTERVAL));

    loop {
        interval.tick().await;

        tracing::info!("Running delete old sessions job");

        // Delete all expired sessions
        match delete_all_expired_sessions(&pg_pool).await {
            Ok(deleted_count) => tracing::info!("Deleted {} old sessions", deleted_count),
            Err(err) => tracing::error!(%err, "Failed to delete old sessions"),
        }
    }
}
