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


#[derive(Deserialize)]
pub struct UpdateTaskDto {
    pub task_id: i64,

    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub task_status: TaskStatus,
    pub task_tag: TaskTag,

    pub start_at: ChronoUtc,
    pub end_at: ChronoUtc,
}


#[derive(Deserialize)]
pub struct TaskFilterDto {
    pub parent_task_id: Option<i64>,

    pub title_contains: Option<String>,
    pub any_of_task_status: Vec<TaskStatus>,
    pub any_of_task_tag: Vec<TaskTag>,

    pub start_at: ChronoUtc,
    pub end_at: ChronoUtc,

    // Cursor from the last task returned
    // None on the first request
    pub cursor_start_at: Option<ChronoUtc>,
    pub cursor_task_id: Option<i64>,
    pub limit: i64,
}


// ------- Implementations ------- //


impl TaskFilterDto {
    /// Check if the filter has any task statuses specified
    fn has_task_status_filter(&self) -> bool {
        !self.any_of_task_status.is_empty()
    }

    /// Check if the filter has any task tags specified
    fn has_task_tag_filter(&self) -> bool {
        !self.any_of_task_tag.is_empty()
    }

    /// Check if the time filter is valid (start_at is before or equal to end_at)
    fn has_valid_time_filter(&self) -> bool {
        self.start_at <= self.end_at
    }

    /// If title filter is specified, check if it is not having an empty string + Special characters
    fn has_valid_title_filter(&self) -> bool {
        if let Some(title) = &self.title_contains {
            !title.trim().is_empty() && title.chars().all(|c| c.is_alphanumeric() || c.is_whitespace())
        } else {
            true
        }
    }

    /// Check if the cursor is valid if it is specified
    fn has_valid_cursor(&self) -> bool {
        // Cursor start at must be given if cursor_task_id is specified
        if self.cursor_task_id.is_some() && self.cursor_start_at.is_none() {
            return false;
        }

        // It should lie within the specified start_at and end_at range if provided
        if let Some(cursor_start_at) = &self.cursor_start_at {
            if cursor_start_at < &self.start_at || cursor_start_at > &self.end_at {
                return false;
            }
        }

        // Limit can only be from 10 to 100
        if self.limit < 10 || self.limit > 100 {
            return false;
        }

        true
    }

    /// Handle all filter basic checks
    pub fn basic_checks(&self) -> bool {
        // All basic checks must pass
        self.has_task_status_filter() && self.has_task_tag_filter() && self.has_valid_time_filter() && self.has_valid_title_filter() && self.has_valid_cursor()
    }
}
