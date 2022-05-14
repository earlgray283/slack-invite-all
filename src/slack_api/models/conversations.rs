use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsResponse {
    pub ok: bool,
    pub channel: Option<Channel>,
    pub error: Option<String>,
    pub members: Option<Vec<String>>,
    #[serde(rename = "response_metadata")]
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMetadata {
    #[serde(rename = "next_cursor")]
    next_cursor: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: String,
    pub name: String,
    #[serde(rename = "is_channel")]
    pub is_channel: bool,
    #[serde(rename = "is_group")]
    pub is_group: bool,
    #[serde(rename = "is_im")]
    pub is_im: bool,
    pub created: i64,
    pub creator: String,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
    #[serde(rename = "is_general")]
    pub is_general: bool,
    pub unlinked: i64,
    #[serde(rename = "name_normalized")]
    pub name_normalized: String,
    #[serde(rename = "is_read_only")]
    pub is_read_only: Option<bool>,
    #[serde(rename = "is_shared")]
    pub is_shared: Option<bool>,
    #[serde(rename = "is_ext_shared")]
    pub is_ext_shared: Option<bool>,
    #[serde(rename = "is_org_shared")]
    pub is_org_shared: Option<bool>,
    #[serde(rename = "pending_shared")]
    pub pending_shared: Vec<Value>,
    #[serde(rename = "is_pending_ext_shared")]
    pub is_pending_ext_shared: Option<bool>,
    #[serde(rename = "is_member")]
    pub is_member: Option<bool>,
    #[serde(rename = "is_private")]
    pub is_private: Option<bool>,
    #[serde(rename = "is_mpim")]
    pub is_mpim: Option<bool>,
    #[serde(rename = "last_read")]
    pub last_read: String,
    pub topic: Topic,
    pub purpose: Purpose,
    #[serde(rename = "previous_names")]
    pub previous_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Topic {
    pub value: String,
    pub creator: String,
    #[serde(rename = "last_set")]
    pub last_set: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Purpose {
    pub value: String,
    pub creator: String,
    #[serde(rename = "last_set")]
    pub last_set: i64,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("already in channel")]
    AlreadyInChannel,
    #[error("undefined error {0}")]
    Undefined(String),
}

impl Error {
    pub fn from(msg: &str) -> Self {
        match msg {
            "already_in_channel" => Self::AlreadyInChannel,
            _ => Self::Undefined(msg.to_string()),
        }
    }
}
