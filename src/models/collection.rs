use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,

    #[serde(default)]
    pub title: Option<serde_json::Value>,

    #[serde(default)]
    pub description: Option<serde_json::Value>,

    #[serde(rename = "iconUrl", default)]
    pub icon_url: Option<String>,

    #[serde(default)]
    pub parent: Option<String>,

    #[serde(rename = "targetAudience", default)]
    pub target_audience: Option<String>,

    #[serde(rename = "articlesCount", default)]
    pub articles_count: Option<u64>,

    #[serde(rename = "subCollectionsCount", default)]
    pub sub_collections_count: Option<u64>,

    #[serde(default)]
    pub lexorank: Option<String>,

    #[serde(rename = "externalId", default)]
    pub external_id: Option<String>,

    #[serde(rename = "docId", default)]
    pub doc_id: Option<u64>,

    #[serde(rename = "createdAt", default)]
    pub created_at: Option<String>,

    #[serde(rename = "updatedAt", default)]
    pub updated_at: Option<String>,

    /// Catch-all for fields we don't explicitly model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct CollectionFilters {
    pub limit: Option<u64>,
    pub skip: Option<u64>,
}
