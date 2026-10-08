use serde::{Deserialize, Serialize};
use super::dto::SingleSignOnInfoDTO;
use uuid::Uuid;




#[derive(Serialize, Deserialize, Clone)]
pub struct SessionUser {
    pub refresh_token: Uuid,
    pub sso_token: String,
    pub fcm_token: Option<String>,

    pub organization_id: Uuid,
    pub organization_name: String,

    pub user_id: Uuid,
    pub email: String,
    pub domain_name: String,

    pub first_name: String,
    pub last_name: Option<String>,

    pub is_external_sharing_enabled: bool,
}


// ------- Implementations ------- //


impl SessionUser {
    pub fn new(sso_info: SingleSignOnInfoDTO, refresh_token: Uuid, sso_token: String) -> Self {
        // TODO: Make sure the user id is from SSO and not generated locally (currently its fake)
        let generated_user_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, sso_info.email.as_bytes());

        SessionUser {
            refresh_token,
            sso_token,
            fcm_token: None,

            organization_id: sso_info.organization_id,
            organization_name: sso_info.organization_name,

            user_id: generated_user_id,
            email: sso_info.email,
            domain_name: sso_info.domain_name,

            first_name: sso_info.first_name,
            last_name: sso_info.last_name,

            is_external_sharing_enabled: sso_info.is_external_sharing_enabled,
        }
    }
}
