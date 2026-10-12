use crate::features::task_views::{TaskTag, TaskStatus};
use serde::Deserialize;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Deserialize)]
pub struct AddAssigneeDto {
    pub task_id: i64,
    pub assignee_id: Uuid,
}


// ------- Implementations ------- //


impl AddAssigneeDto {
    /// Validate the AddAssigneeDto
    pub fn validate(&self, session_user_id: &Uuid) -> bool {
        // User can not assign themselves to a task
        self.assignee_id != *session_user_id
    }
}
