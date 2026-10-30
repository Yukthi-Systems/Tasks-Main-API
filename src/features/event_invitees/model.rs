use postgres_types::{ToSql, FromSql};
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


#[derive(Debug, PartialEq, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "invite_response_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InviteResponseStatus {
    Pending,    // Invite sent but no response yet
    Maybe,      // Invitee is unsure / Tentative response
    Accepted,   // Invite accepted
    Declined,   // Invite declined
}


#[derive(RowFrom, Serialize)]
pub struct EventInvitee {
    pub task_id: i64,
    pub invitee_email: String,

    pub validation_code: Uuid,

    pub response_status: InviteResponseStatus,
    pub response_info: JsonValue,

    pub invited_at: ChronoUtc,
}


#[derive(RowFrom, Serialize)]
pub struct PubEventInvitee {
    pub invitee_email: String,

    pub response_status: InviteResponseStatus,
    pub response_info: JsonValue,

    pub invited_at: ChronoUtc,
}
