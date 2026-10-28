use pg_row_derive::RowFrom;
use serde::Serialize;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(RowFrom, Serialize)]
pub struct TaskAlert {
    pub task_id: i64,
    pub alert_recipient_id: Uuid,
    pub alert_at: ChronoUtc,

    pub notify_via_email: bool,
    pub notify_via_push: bool,
    pub notification_sent: bool,

    pub created_at: ChronoUtc,
}
