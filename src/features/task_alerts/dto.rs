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


#[derive(Deserialize)]
pub struct ListAlertsDto {
    pub alert_at_ascending: bool,
    pub alert_at_after: Option<ChronoUtc>,
    pub offset: i64,
    pub limit: i64,
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


impl ListAlertsDto {
    /// Validate the ListAlertsDto
    pub fn validate(&self) -> bool {
        // Can be only positive values for offset and limit
        if self.offset < 0 || self.limit <= 0 {
            return false;
        }

        // Per page it cant exceed a reasonable limit of 50
        if self.limit > 50 {
            return false;
        }

        true
    }
}
