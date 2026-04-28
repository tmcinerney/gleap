use crate::error::AppError;
use crate::models::collection::{Collection, CollectionFilters};

use super::GleapClient;

pub struct CollectionsClient<'a> {
    client: &'a GleapClient,
}

impl<'a> CollectionsClient<'a> {
    pub(crate) fn new(client: &'a GleapClient) -> Self {
        Self { client }
    }

    /// List all collections with optional filters.
    pub async fn list(&self, filters: &CollectionFilters) -> Result<Vec<Collection>, AppError> {
        let mut request = self.client.get("/helpcenter/collections");

        if let Some(limit) = filters.limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }
        if let Some(skip) = filters.skip {
            request = request.query(&[("skip", &skip.to_string())]);
        }

        self.client.send_and_parse(request).await
    }

    /// Get a single collection by ID.
    pub async fn get(&self, id: &str) -> Result<Collection, AppError> {
        let request = self.client.get(&format!("/helpcenter/collections/{}", id));
        self.client.send_and_parse(request).await
    }

    /// Create a new collection.
    pub async fn create(&self, body: serde_json::Value) -> Result<Collection, AppError> {
        let request = self.client.post("/helpcenter/collections").json(&body);
        self.client.send_and_parse(request).await
    }

    /// Update a collection by ID.
    pub async fn update(&self, id: &str, body: serde_json::Value) -> Result<Collection, AppError> {
        let request = self
            .client
            .put(&format!("/helpcenter/collections/{}", id))
            .json(&body);
        self.client.send_and_parse(request).await
    }

    /// Delete a collection by ID.
    pub async fn delete(&self, id: &str) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .delete(&format!("/helpcenter/collections/{}", id));
        self.client.send_and_parse(request).await
    }
}
