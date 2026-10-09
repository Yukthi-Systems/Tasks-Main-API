use serde::{Deserialize, Serialize};
use super::model::TaskStatus;
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;



#[derive(Deserialize)]
pub struct CreateTaskViewDto {
    pub view_name: String,
    pub description: String,

    pub ui_info: serde_json::Value,
    pub status_filter: Vec<TaskStatus>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
}


#[derive(Deserialize)]
pub struct UpdateTaskViewDto {
    pub view_id: i64,
    pub view_name: String,
    pub description: String,

    pub ui_info: serde_json::Value,
    pub status_filter: Vec<TaskStatus>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
}


#[derive(Serialize, RowFrom)]
pub struct TaskViewDbDto {
    pub view_id: i64,
    pub owner_id: Uuid,

    pub view_name: String,
    pub description: String,

    pub ui_info: serde_json::Value,
    pub status_filter: Vec<String>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
}
