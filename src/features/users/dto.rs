use serde::Deserialize;
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

#[derive(Deserialize)]
pub struct SessionTokensDto {
    pub refresh_token: Uuid,
    pub access_token: Uuid,
}
