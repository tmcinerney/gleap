use serde::{Deserialize, Serialize};

use super::ticket::{SessionRef, UserRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Text,
    Note,
    Bot,
    BotReply,
    UserText,
    SharedComment,
    FeedbackUpdated,
    #[serde(other)]
    Unknown,
}

/// The `data` envelope returned by the API for message content.
/// The actual message text lives in `content`, which can be a plain
/// string or a rich document object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(default)]
    pub content: Option<serde_json::Value>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,

    #[serde(default)]
    pub ticket: Option<String>,

    /// Write-only: always null in API responses. Content lives in `data.content`.
    #[serde(default)]
    pub comment: Option<serde_json::Value>,

    #[serde(rename = "type", default)]
    pub message_type: Option<MessageType>,

    #[serde(default)]
    pub data: Option<MessageData>,

    #[serde(default)]
    pub bot: Option<bool>,

    /// Write-only: always null in API responses. Use `message_type` to check for notes.
    #[serde(rename = "isNote", default)]
    pub is_note: Option<bool>,

    #[serde(rename = "isReply", default)]
    pub is_reply: Option<bool>,

    #[serde(default)]
    pub user: Option<UserRef>,

    #[serde(default)]
    pub session: Option<SessionRef>,

    #[serde(default)]
    pub attachments: Option<Vec<Attachment>>,

    #[serde(default)]
    pub index: Option<u64>,

    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,

    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub url: Option<String>,

    #[serde(rename = "type", default)]
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessageRequest {
    pub ticket: String,

    pub comment: serde_json::Value,

    #[serde(rename = "isNote", skip_serializing_if = "Option::is_none")]
    pub is_note: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Debug, Clone, Default)]
pub struct MessageFilters {
    pub ticket: Option<String>,
    pub message_type: Option<String>,
    pub bot: Option<bool>,
    pub sort: Option<String>,
    pub limit: Option<u64>,
    pub skip: Option<u64>,
}
