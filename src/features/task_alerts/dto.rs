use serde::Deserialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Deserialize)]
pub struct TaskAlertDto {   // Same for Create / Edit
    pub alert_at: ChronoUtc,
    pub notify_via_email: bool,
    pub notify_via_push: bool
}


#[derive(Deserialize)]
pub struct DeleteTaskAlertDto {
    pub alert_at: ChronoUtc,
}


// ------- Implementations ------- //


impl TaskAlertDto {
    /// Validate the TaskAlertDto
    pub fn validate(&self) -> bool {
        // Alert time must be in the future + at least 5 minutes from now
        self.alert_at > chrono::Utc::now() + chrono::Duration::minutes(5)
    }
}
