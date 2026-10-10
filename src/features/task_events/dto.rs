use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct CreateRecurringTaskDto {
    pub title: String,
    pub description: String,
    pub details: serde_json::Value,
    pub rrule: String,  // Recurrence rule in RFC 5545 format (e.g., "FREQ=WEEKLY;BYDAY=MO,WE,FR")
}
