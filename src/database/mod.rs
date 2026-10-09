use crate::errors::{PgResult, PgPool};


pub mod pool;


/// DB working state Check
pub async fn health_check(db_pool: &PgPool) -> PgResult<()> {
    // Simple query to check if the database is responsive
    let client = db_pool.get().await?;

    let _ = client.query("SELECT 1", &[]).await?;

    Ok(())
}
