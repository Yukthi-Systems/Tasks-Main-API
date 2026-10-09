use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;



#[derive(Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    OnHold,
}


#[derive(RowFrom, Serialize, Deserialize)]
pub struct TaskViewPermission {
    pub view_id: i64,
    pub owner_id: Uuid,
    pub status_filter: Vec<String>,
    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,
    pub can_create: bool,
    pub can_edit: bool,
    pub can_delete: bool,
}


// ------- Implementations ------- //


impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "PENDING",
            TaskStatus::InProgress => "IN_PROGRESS",
            TaskStatus::Completed => "COMPLETED",
            TaskStatus::Cancelled => "CANCELLED",
            TaskStatus::OnHold => "ON_HOLD",
        }
    }

    pub fn from_str(s: &str) -> Option<TaskStatus> {
        match s {
            "PENDING" => Some(TaskStatus::Pending),
            "IN_PROGRESS" => Some(TaskStatus::InProgress),
            "COMPLETED" => Some(TaskStatus::Completed),
            "CANCELLED" => Some(TaskStatus::Cancelled),
            "ON_HOLD" => Some(TaskStatus::OnHold),
            _ => None,
        }
    }
}
