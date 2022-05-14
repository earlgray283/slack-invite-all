use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersListResponse {
    pub ok: bool,
    pub members: Option<Vec<Member>>,
    #[serde(rename = "cache_ts")]
    pub cache_ts: Option<i64>,
    #[serde(rename = "response_metadata")]
    pub response_metadata: Option<ResponseMetadata>,
    pub error: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub id: String,
    #[serde(rename = "team_id")]
    pub team_id: String,
    pub name: String,
    pub deleted: bool,
    pub color: Option<String>,
    #[serde(rename = "real_name")]
    pub real_name: Option<String>,
    pub tz: Option<String>,
    #[serde(rename = "tz_label")]
    pub tz_label: Option<String>,
    #[serde(rename = "tz_offset")]
    pub tz_offset: Option<i64>,
    pub profile: Profile,
    #[serde(rename = "is_admin")]
    pub is_admin: Option<bool>,
    #[serde(rename = "is_owner")]
    pub is_owner: Option<bool>,
    #[serde(rename = "is_primary_owner")]
    pub is_primary_owner: Option<bool>,
    #[serde(rename = "is_restricted")]
    pub is_restricted: Option<bool>,
    #[serde(rename = "is_ultra_restricted")]
    pub is_ultra_restricted: Option<bool>,
    #[serde(rename = "is_bot")]
    pub is_bot: Option<bool>,
    pub updated: i64,
    #[serde(rename = "is_app_user")]
    pub is_app_user: Option<bool>,
    #[serde(rename = "has_2fa")]
    pub has_2fa: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "avatar_hash")]
    pub avatar_hash: String,
    #[serde(rename = "status_text")]
    pub status_text: Option<String>,
    #[serde(rename = "status_emoji")]
    pub status_emoji: Option<String>,
    #[serde(rename = "real_name")]
    pub real_name: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "real_name_normalized")]
    pub real_name_normalized: String,
    #[serde(rename = "display_name_normalized")]
    pub display_name_normalized: String,
    pub email: Option<String>,
    #[serde(rename = "image_24")]
    pub image_24: String,
    #[serde(rename = "image_32")]
    pub image_32: String,
    #[serde(rename = "image_48")]
    pub image_48: String,
    #[serde(rename = "image_72")]
    pub image_72: String,
    #[serde(rename = "image_192")]
    pub image_192: String,
    #[serde(rename = "image_512")]
    pub image_512: String,
    pub team: Option<String>,
    #[serde(rename = "image_1024")]
    pub image_1024: Option<String>,
    #[serde(rename = "image_original")]
    pub image_original: Option<String>,
    #[serde(rename = "first_name")]
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    pub last_name: Option<String>,
    pub title: Option<String>,
    pub phone: Option<String>,
    pub skype: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(rename = "next_cursor")]
    pub next_cursor: String,
}
