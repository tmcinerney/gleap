use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,

    #[serde(default)]
    pub title: Option<serde_json::Value>,

    #[serde(default)]
    pub description: Option<serde_json::Value>,

    #[serde(default)]
    pub content: Option<serde_json::Value>,

    #[serde(rename = "plainContent", default)]
    pub plain_content: Option<serde_json::Value>,

    #[serde(rename = "helpcenterCollection", default)]
    pub helpcenter_collection: Option<String>,

    #[serde(default)]
    pub author: Option<serde_json::Value>,

    #[serde(rename = "isDraft", default)]
    pub is_draft: Option<bool>,

    #[serde(default)]
    pub tags: Option<Vec<String>>,

    #[serde(rename = "targetAudience", default)]
    pub target_audience: Option<String>,

    #[serde(default)]
    pub upvotes: Option<serde_json::Value>,

    #[serde(default)]
    pub lexorank: Option<String>,

    #[serde(rename = "externalId", default)]
    pub external_id: Option<String>,

    #[serde(rename = "docId", default)]
    pub doc_id: Option<u64>,

    #[serde(rename = "sourceUsage", default)]
    pub source_usage: Option<u64>,

    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,

    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,

    /// Catch-all for fields we don't explicitly model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleFilters {
    pub limit: Option<u64>,
    pub skip: Option<u64>,
}
