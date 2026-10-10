use crate::features::task_views::{TaskTag, TaskStatus};
use serde::Deserialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Deserialize)]
pub struct CreateRecurringTaskDto {
    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub rrule: String,  // Recurrence rule in RFC 5545 format (e.g., "FREQ=WEEKLY;BYDAY=MO,WE,FR")
    pub task_tag: TaskTag,
}


#[derive(Deserialize)]
pub struct UpdateRecurringTaskDto {
    pub recurring_task_id: i64,

    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub rrule: String,  // Recurrence rule in RFC 5545 format (e.g., "FREQ=WEEKLY;BYDAY=MO,WE,FR")
    pub task_tag: TaskTag,
}


#[derive(Deserialize)]
pub struct CreateTaskDto {
    pub parent_task_id: Option<i64>,

    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub task_status: TaskStatus,
    pub task_tag: TaskTag,

    pub start_at: ChronoUtc,
    pub end_at: ChronoUtc,
}
