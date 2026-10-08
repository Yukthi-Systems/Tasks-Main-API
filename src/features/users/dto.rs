use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Deserialize)]
pub struct SingleSignOnInfoDTO {
    pub email: String,
    pub domain_name: String,

    pub first_name: String,
    pub last_name: Option<String>,

    pub organization_id: Uuid,
    pub organization_name: String,

    pub is_external_sharing_enabled: bool,
}
