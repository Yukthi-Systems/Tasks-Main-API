use postgres_types::{ToSql, FromSql};
use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Debug, PartialEq, Serialize, Deserialize, ToSql, FromSql)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[postgres(name = "task_status")]
pub enum TaskStatus {
    #[postgres(name = "PENDING")]
    Pending,    // Task is pending and yet to be started

    #[postgres(name = "IN_PROGRESS")]
    InProgress, // Task is currently in progress

    #[postgres(name = "COMPLETED")]
    Completed,  // Task has been completed

    #[postgres(name = "CANCELLED")]
    Cancelled,  // Task has been cancelled

    #[postgres(name = "ON_HOLD")]
    OnHold,     // Task is on hold
}


#[derive(Debug, PartialEq, Serialize, Deserialize, ToSql, FromSql)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[postgres(name = "task_tag")]
pub enum TaskTag {
    #[postgres(name = "WORK")]
    Work,       // Work-related tasks

    #[postgres(name = "IMPORTANT")]
    Important,  // High significance

    #[postgres(name = "URGENT")]
    Urgent,     // Requires prompt attention

    #[postgres(name = "OPTIONAL")]
    Optional,   // Can be skipped

    #[postgres(name = "PERSONAL")]
    Personal,   // Personal tasks

    #[postgres(name = "OTHER")]
    Other,      // Anything else
}


#[derive(RowFrom, Serialize, Deserialize)]
pub struct TaskViewPermission {
    pub view_id: i64,
    pub owner_id: Uuid,

    pub status_filter: Vec<TaskStatus>,
    pub task_tag_filter: Vec<TaskTag>,

    pub show_recurring: bool,
    pub show_comments: bool,
    pub show_subtasks: bool,
    pub show_assigned: bool,

    pub can_create: bool,
    pub can_edit: bool,
    pub can_delete: bool,

    pub is_owner: bool,
}


#[derive(RowFrom, Serialize, Deserialize)]
pub struct SharedTaskView {
    pub view_id: i64,
    pub user_id: Uuid,

    pub share_notes: String,
    pub ui_info: serde_json::Value,

    pub can_create: bool,
    pub can_edit: bool,
    pub can_delete: bool,

    pub shared_at: ChronoUtc,
}


// ------- Implementations ------- //


impl TaskViewPermission {
    pub fn check_status(&self, status: &TaskStatus) -> bool {
        // If it is empty, it means all statuses are allowed
        if self.status_filter.is_empty() {
            return true;
        }

        self.status_filter.contains(status)
    }

    /// Check if the task view permission allows the specified task tag
    pub fn check_tag(&self, tag: &TaskTag) -> bool {
        // If it is empty, it means all tags are allowed
        if self.task_tag_filter.is_empty() {
            return true;
        }

        self.task_tag_filter.contains(tag)
    }

    /// Check if all the specified task tags are allowed by the task view permission
    pub fn check_all_tags(&self, tags: &[TaskTag]) -> bool {
        tags.iter().all(|tag| self.check_tag(tag))
    }

    /// Check if all the specified task statuses are allowed by the task view permission
    pub fn check_all_statuses(&self, statuses: &[TaskStatus]) -> bool {
        statuses.iter().all(|status| self.check_status(status))
    }
}
