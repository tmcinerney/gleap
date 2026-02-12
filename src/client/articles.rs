use crate::error::AppError;
use crate::models::article::{Article, ArticleFilters};

use super::GleapClient;

pub struct ArticlesClient<'a> {
    client: &'a GleapClient,
}

impl<'a> ArticlesClient<'a> {
    pub(crate) fn new(client: &'a GleapClient) -> Self {
        Self { client }
    }

    fn base_path(collection_id: &str) -> String {
        format!("/helpcenter/collections/{}/articles", collection_id)
    }

    fn article_path(collection_id: &str, article_id: &str) -> String {
        format!(
            "/helpcenter/collections/{}/articles/{}",
            collection_id, article_id
        )
    }

    /// List articles in a collection.
    pub async fn list(
        &self,
        collection_id: &str,
        filters: &ArticleFilters,
    ) -> Result<Vec<Article>, AppError> {
        let mut request = self.client.get(&Self::base_path(collection_id));

        if let Some(limit) = filters.limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }
        if let Some(skip) = filters.skip {
            request = request.query(&[("skip", &skip.to_string())]);
        }

        self.client.send_and_parse(request).await
    }

    /// Get a single article by ID.
    pub async fn get(&self, collection_id: &str, article_id: &str) -> Result<Article, AppError> {
        let request = self
            .client
            .get(&Self::article_path(collection_id, article_id));
        self.client.send_and_parse(request).await
    }

    /// Create a new article in a collection.
    pub async fn create(
        &self,
        collection_id: &str,
        body: serde_json::Value,
    ) -> Result<Article, AppError> {
        let request = self
            .client
            .post(&Self::base_path(collection_id))
            .json(&body);
        self.client.send_and_parse(request).await
    }

    /// Update an article by ID.
    pub async fn update(
        &self,
        collection_id: &str,
        article_id: &str,
        body: serde_json::Value,
    ) -> Result<Article, AppError> {
        let request = self
            .client
            .put(&Self::article_path(collection_id, article_id))
            .json(&body);
        self.client.send_and_parse(request).await
    }

    /// Delete an article by ID.
    pub async fn delete(
        &self,
        collection_id: &str,
        article_id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .delete(&Self::article_path(collection_id, article_id));
        self.client.send_and_parse(request).await
    }

    /// Move an article to a different collection.
    pub async fn move_article(
        &self,
        collection_id: &str,
        article_id: &str,
        new_collection_id: &str,
    ) -> Result<Article, AppError> {
        let path = format!(
            "/helpcenter/collections/{}/articles/{}/move",
            collection_id, article_id
        );
        let body = serde_json::json!({ "newCollectionId": new_collection_id });
        let request = self.client.put(&path).json(&body);
        self.client.send_and_parse(request).await
    }
}
