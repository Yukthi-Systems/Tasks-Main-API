use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    OnHold,
}


#[derive(Deserialize)]
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

    pub status_filter: Vec<String>,
    pub task_tag_filter: Vec<String>,

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
}


impl TaskTag {
    pub fn as_str(&self) -> &str {
        match self {
            TaskTag::Work => "WORK",
            TaskTag::Important => "IMPORTANT",
            TaskTag::Urgent => "URGENT",
            TaskTag::Optional => "OPTIONAL",
            TaskTag::Personal => "PERSONAL",
            TaskTag::Other => "OTHER",
        }
    }
}


impl TaskViewPermission {
    pub fn check_status(&self, status: &TaskStatus) -> bool {
        self.status_filter.contains(&status.as_str().to_string())
    }

    /// Check if the task view permission allows the specified task tag
    pub fn check_tag(&self, tag: &TaskTag) -> bool {
        self.task_tag_filter.contains(&tag.as_str().to_string())
    }

    /// Check if all the specified task tags are allowed by the task view permission
    pub fn check_all_tags(&self, tags: &[TaskTag]) -> bool {
        tags.iter().all(|tag| self.check_tag(tag))
    }
}
