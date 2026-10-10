use super::model::{TaskStatus, TaskTag};
use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;



#[derive(Deserialize)]
pub struct CreateTaskViewDto {
    pub view_name: String,
    pub description: String,

    pub ui_info: serde_json::Value,
    pub status_filter: Vec<TaskStatus>,
    pub task_tag_filter: Vec<TaskTag>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
}


#[derive(Deserialize)]
pub struct CreateSharedTaskViewDto {
    pub view_id: i64,
    pub user_id: Uuid,  // Shared with this user

    pub share_notes: String,
    pub ui_info: serde_json::Value, // UI-related information for the shared user

    pub can_create: bool,
    pub can_edit: bool,
    pub can_delete: bool,
}


#[derive(Deserialize)]
pub struct UpdateTaskViewDto {
    pub view_id: i64,
    pub view_name: String,
    pub description: String,

    pub ui_info: serde_json::Value,
    pub status_filter: Vec<TaskStatus>,
    pub task_tag_filter: Vec<TaskTag>,

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
    pub task_tag_filter: Vec<String>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
}
