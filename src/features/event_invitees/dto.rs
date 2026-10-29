use serde::Deserialize;



#[derive(Deserialize)]
pub struct PubInviteeDto {
    pub task_id: i64,
    pub invitee_email: String,
}
