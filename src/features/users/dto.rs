use serde::{Deserialize, Serialize};
use pg_row_derive::RowFrom;
use uuid::Uuid;


type ChronoUtc = chrono::DateTime<chrono::Utc>;


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


#[derive(RowFrom, Serialize)]
pub struct UserSearchInfoDbDto {
    pub user_id: Uuid,
    pub email: String,
    pub domain: String,
    pub public_info: serde_json::Value,
    pub created_at: ChronoUtc,
}


#[derive(RowFrom, Serialize)]
pub struct UserInfoDbDto {
    pub user_id: Uuid,
    pub email: String,
    pub domain: String,

    pub organization_id: Uuid,
    pub organization_name: String,

    pub private_info: serde_json::Value,
    pub public_info: serde_json::Value,

    pub is_external_sharing_enabled: bool,
    pub created_at: ChronoUtc,
}


#[derive(Serialize)]
pub struct UserInfoPublicDto {
    pub user_id: Uuid,
    pub email: String,
    pub domain: String,

    pub organization_id: Uuid,
    pub organization_name: String,

    pub public_info: serde_json::Value,
    pub is_external_sharing_enabled: bool,
    pub created_at: ChronoUtc,
}


// ------- Implementations ------- //


impl From<UserInfoDbDto> for UserInfoPublicDto {
    fn from(user_info: UserInfoDbDto) -> Self {
        UserInfoPublicDto {
            user_id: user_info.user_id,
            email: user_info.email,
            domain: user_info.domain,
            organization_id: user_info.organization_id,
            organization_name: user_info.organization_name,
            public_info: user_info.public_info,
            is_external_sharing_enabled: user_info.is_external_sharing_enabled,
            created_at: user_info.created_at,
        }
    }
}


impl UserInfoDbDto {
    pub fn into_public(self) -> UserInfoPublicDto {
        UserInfoPublicDto::from(self)
    }
}
