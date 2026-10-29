use super::model::InviteResponseStatus;
use serde::Deserialize;



#[derive(Deserialize)]
pub struct PubInviteeDto {
    pub task_id: i64,
    pub invitee_email: String,
}


#[derive(Deserialize)]
pub struct PubInviteResponseDto {
    pub response_status: InviteResponseStatus,
    pub response_info: serde_json::Value
}
