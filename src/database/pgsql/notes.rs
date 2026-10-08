use crate::errors::{PgResult, PgPool};
use crate::features::notes::Notes;



/// Sample private function to create a new note
async fn create_single_note(db_pool: &PgPool, note: Notes) -> PgResult<i32> {
    let client = db_pool.get().await?;
    let result = client
        .query(
            r#"
            INSERT INTO notes (title, content)
            VALUES ($1, $2)
            RETURNING id
            "#,
            &[&note.title, &note.content],
        )
        .await?;
    Ok(result[0].get("id"))
}


/// Add few sample data in DB
pub async fn add_new_notes(db_pool: &PgPool, values: Vec<Notes>) -> PgResult<()> {
    for note in values {
        // We can do like this to purely put the query in one function and call it in another function
        // We can even do some processing before calling the query (but all db related stuff should be in db module only)
        let pool = db_pool.clone(); // We can even clone the pool and spawn it to be fully parallel
        tokio::spawn(async move {
            let _ = create_single_note(&pool, note).await;
        });
    }

    Ok(())
}


/// Fetch all notes from DB
pub async fn fetch_all_notes(db_pool: &PgPool) -> PgResult<Vec<Notes>> {
    let client = db_pool.get().await?;
    let rows = client
        .query(
            r#"
            SELECT id, title, content FROM notes
            "#,
            &[],
        )
        .await?;

    Ok(Notes::from_rows(rows))
}


/// Delete notes older than 30 days from DB
pub async fn delete_old_notes(db_pool: &PgPool) -> PgResult<()> {
    let client = db_pool.get().await?;
    client
        .execute(
            r#"
            DELETE FROM notes
            WHERE created_at < NOW() - INTERVAL '30 days'
            "#,
            &[],
        )
        .await?;
    Ok(())
}
