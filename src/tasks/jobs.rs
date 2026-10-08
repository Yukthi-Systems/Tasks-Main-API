use crate::database::notes::delete_old_notes;
use deadpool_postgres::Pool as PgPool;
use std::future::Future;
use std::time::Duration;


const ORPHANED_FILE_CLEANUP_INTERVAL: u64 = 3 * 60 * 60;
const FILE_LOCK_CLEANUP_INTERVAL: u64 = 3 * 60 * 60;


async fn run_every<F, Fut, E>(seconds: u64, mut job: F)
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<(), E>>,
        E: std::fmt::Debug,
{
    let mut interval = tokio::time::interval(Duration::from_secs(seconds));

    loop {
        interval.tick().await;

        if let Err(err) = job().await {
            tracing::error!("Background job failed: {:?}", err);
        }
    }
}


pub async fn remove_expired_file_locks(pg_pool: PgPool) {
    tracing::info!("Starting remove_expired_file_locks job");

    run_every(FILE_LOCK_CLEANUP_INTERVAL, || {
        delete_old_notes(&pg_pool)
    })
    .await;
}


pub async fn delete_orphaned_files(pg_pool: PgPool) {
    tracing::info!("Starting delete_orphaned_files job");

    run_every(ORPHANED_FILE_CLEANUP_INTERVAL, || {
        delete_old_notes(&pg_pool)
    })
    .await;
}


pub async fn delete_trash_after_30_days(pg_pool: PgPool) {
    tracing::info!("Starting delete_trash_after_30_days job");

    run_every(ORPHANED_FILE_CLEANUP_INTERVAL, || {
        delete_old_notes(&pg_pool)
    })
    .await;
}
