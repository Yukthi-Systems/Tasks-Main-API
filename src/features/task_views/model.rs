use postgres_types::{ToSql, FromSql};
use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Debug, PartialEq, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "task_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Pending,    // Task is pending and yet to be started
    InProgress, // Task is currently in progress
    Completed,  // Task has been completed
    Cancelled,  // Task has been cancelled
    OnHold,     // Task is on hold
}


#[derive(Debug, PartialEq, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "task_tag", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskTag {
    Work,       // Work-related tasks
    Important,  // High significance
    Urgent,     // Requires prompt attention
    Optional,   // Can be skipped
    Personal,   // Personal tasks
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
