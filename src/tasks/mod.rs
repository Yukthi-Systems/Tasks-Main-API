use deadpool_postgres::Pool as PgPool;

mod jobs;


pub fn start_background_jobs(pg_pool: PgPool) {
    // Remove Lock on File
    tokio::spawn(jobs::delete_old_sessions(pg_pool.clone()));

    // // Delete Trash after 30 days
    // tokio::spawn(jobs::delete_trash_after_30_days(pg_pool.clone()));

    // // Delete Orphaned Files
    // tokio::spawn(jobs::delete_orphaned_files(pg_pool.clone()));
}
