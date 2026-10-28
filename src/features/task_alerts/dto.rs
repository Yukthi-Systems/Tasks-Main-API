use serde::Deserialize;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Deserialize)]
pub struct CreateTaskAlertDto {
    pub alert_at: ChronoUtc,
    pub notify_via_email: bool,
    pub notify_via_push: bool
}


#[derive(Deserialize)]
pub struct EditTaskAlertDto {
    pub current_alert_at: ChronoUtc,
    pub new_alert_at: ChronoUtc,
    pub notify_via_email: bool,
    pub notify_via_push: bool
}


#[derive(Deserialize)]
pub struct DeleteTaskAlertDto {
    pub alert_at: ChronoUtc,
}


// ------- Implementations ------- //


impl CreateTaskAlertDto {
    /// Validate the CreateTaskAlertDto
    pub fn validate(&self) -> bool {
        // Alert time must be in the future + at least 5 minutes from now
        self.alert_at > chrono::Utc::now() + chrono::Duration::minutes(5)
    }
}


impl EditTaskAlertDto {
    /// Validate the EditTaskAlertDto
    pub fn validate(&self) -> bool {
        // New alert time must be in the future + at least 5 minutes from now
        self.new_alert_at > chrono::Utc::now() + chrono::Duration::minutes(5)
    }
}
