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


#[derive(RowFrom, Serialize, Deserialize)]
pub struct TaskOwnershipInfo {
    task_id: i64,
    owner_id: Uuid,
    pub is_root_task: bool,
    created_by: Option<Uuid>,
    assignee_ids: Vec<Uuid>,
}


#[derive(RowFrom, Serialize)]
pub struct TaskInfo {
    pub task_id: i64,
    pub parent_task_id: Option<i64>,

    pub owner_id: Uuid,
    pub created_by: Option<Uuid>,
    pub assignee_ids: Vec<Uuid>,

    pub title: String,
    pub description: String,
    pub details: serde_json::Value,

    pub task_status: String,
    pub task_tag: String,

    pub start_at: ChronoUtc,
    pub end_at: ChronoUtc,

    pub created_at: ChronoUtc,
    pub updated_at: ChronoUtc,
}


// ------- Implementations ------- //


impl TaskOwnershipInfo {
    pub fn is_owner(&self, user_id: &Uuid) -> bool {
        &self.owner_id == user_id
    }

    pub fn is_assignee(&self, user_id: &Uuid) -> bool {
        self.assignee_ids.contains(user_id)
    }

    pub fn is_creator(&self, user_id: &Uuid) -> bool {
        match &self.created_by {
            Some(creator_id) => creator_id == user_id,
            None => false,
        }
    }

    pub fn can_create(&self, user_id: &Uuid) -> bool {
        // Can be any of the owner, assignee, or creator
        // To create sub-tasks or comments
        self.is_owner(user_id) || self.is_assignee(user_id) || self.is_creator(user_id)
    }

    pub fn can_edit(&self, user_id: &Uuid) -> bool {
        // Can edit if the user is the owner or the creator
        self.is_owner(user_id) || self.is_creator(user_id)
    }

    pub fn can_delete(&self, user_id: &Uuid) -> bool {
        // Can delete if the user is the owner or the creator
        self.is_owner(user_id) || self.is_creator(user_id)
    }
}
