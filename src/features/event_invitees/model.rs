use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize)]
pub struct TaskComment {
    pub comment_id: i64,
    pub task_id: i64,
    pub author_id: Uuid,

    pub content: String,
    pub reactions: serde_json::Value,

    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
}
