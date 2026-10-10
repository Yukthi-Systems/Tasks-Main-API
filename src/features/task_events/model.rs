use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize)]
pub struct RecurringTask {
    pub recurring_task_id: i64,
    pub owner_id: Uuid,

    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub rrule: String,  // Recurrence rule in RFC 5545 format (e.g., "FREQ=WEEKLY;BYDAY=MO,WE,FR")
    pub task_tag: String,

    pub created_at: ChronoUtc,
}
