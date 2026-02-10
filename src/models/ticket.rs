use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketType {
    Bug,
    FeatureRequest,
    Inquiry,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketStatus {
    Open,
    InProgress,
    Done,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(default)]
    pub title: Option<String>,

    #[serde(rename = "type", default)]
    pub ticket_type: Option<TicketType>,

    #[serde(default)]
    pub status: Option<TicketStatus>,

    #[serde(default)]
    pub priority: Option<TicketPriority>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(rename = "formData", default)]
    pub form_data: Option<serde_json::Value>,

    #[serde(rename = "customData", default)]
    pub custom_data: Option<serde_json::Value>,

    #[serde(rename = "processingUser", default)]
    pub processing_user: Option<UserRef>,

    #[serde(default)]
    pub session: Option<SessionRef>,

    #[serde(rename = "latestComment", default)]
    pub latest_comment: Option<serde_json::Value>,

    #[serde(default)]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "imageUrl", default)]
    pub image_url: Option<String>,

    #[serde(default)]
    pub archived: Option<bool>,

    #[serde(rename = "isSpam", default)]
    pub is_spam: Option<bool>,

    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,

    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,

    /// Catch-all for fields we don't explicitly model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRef {
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(default)]
    pub email: Option<String>,

    #[serde(rename = "firstName", default)]
    pub first_name: Option<String>,

    #[serde(rename = "lastName", default)]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRef {
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(default)]
    pub email: Option<String>,

    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketListResponse {
    pub tickets: Vec<Ticket>,

    #[serde(default)]
    pub count: Option<u64>,

    #[serde(rename = "totalCount", default)]
    pub total_count: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct TicketFilters {
    pub status: Option<String>,
    pub ticket_type: Option<String>,
    pub priority: Option<String>,
    pub archived: Option<bool>,
    pub is_spam: Option<bool>,
    pub sort: Option<String>,
    pub limit: Option<u64>,
    pub skip: Option<u64>,
}
