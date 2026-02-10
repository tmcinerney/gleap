use crate::error::AppError;
use crate::models::ticket::{Ticket, TicketFilters, TicketListResponse};

use super::GleapClient;

pub struct TicketsClient<'a> {
    client: &'a GleapClient,
}

impl<'a> TicketsClient<'a> {
    pub(crate) fn new(client: &'a GleapClient) -> Self {
        Self { client }
    }

    /// List tickets with optional filters.
    pub async fn list(&self, filters: &TicketFilters) -> Result<TicketListResponse, AppError> {
        let mut request = self.client.get("/tickets");

        if let Some(ref status) = filters.status {
            request = request.query(&[("status", status.as_str())]);
        }
        if let Some(ref ticket_type) = filters.ticket_type {
            request = request.query(&[("type", ticket_type.as_str())]);
        }
        if let Some(ref priority) = filters.priority {
            request = request.query(&[("priority", priority.as_str())]);
        }
        if let Some(archived) = filters.archived {
            request = request.query(&[("archived", &archived.to_string())]);
        }
        if let Some(is_spam) = filters.is_spam {
            request = request.query(&[("isSpam", &is_spam.to_string())]);
        }
        if let Some(ref sort) = filters.sort {
            request = request.query(&[("sort", sort.as_str())]);
        }
        if let Some(limit) = filters.limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }
        if let Some(skip) = filters.skip {
            request = request.query(&[("skip", &skip.to_string())]);
        }

        let response = self.client.send(request).await?;
        let body = response.json::<TicketListResponse>().await?;
        Ok(body)
    }

    /// Get a single ticket by ID.
    pub async fn get(&self, ticket_id: &str) -> Result<Ticket, AppError> {
        let request = self.client.get(&format!("/tickets/{}", ticket_id));
        let response = self.client.send(request).await?;
        let ticket = response.json::<Ticket>().await?;
        Ok(ticket)
    }

    /// Update a ticket by ID. Accepts arbitrary JSON fields.
    pub async fn update(
        &self,
        ticket_id: &str,
        fields: serde_json::Value,
    ) -> Result<Ticket, AppError> {
        let request = self
            .client
            .patch(&format!("/tickets/{}", ticket_id))
            .json(&fields);
        let response = self.client.send(request).await?;
        let ticket = response.json::<Ticket>().await?;
        Ok(ticket)
    }

    /// Search tickets by text query.
    pub async fn search(
        &self,
        query: &str,
        limit: Option<u64>,
        skip: Option<u64>,
    ) -> Result<TicketListResponse, AppError> {
        let mut request = self.client.get("/tickets/search").query(&[("query", query)]);

        if let Some(limit) = limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }
        if let Some(skip) = skip {
            request = request.query(&[("skip", &skip.to_string())]);
        }

        let response = self.client.send(request).await?;
        let body = response.json::<TicketListResponse>().await?;
        Ok(body)
    }

    /// Get activity logs for a ticket.
    pub async fn activity_logs(
        &self,
        ticket_id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/activitylogs", ticket_id));
        let response = self.client.send(request).await?;
        let body = response.json::<serde_json::Value>().await?;
        Ok(body)
    }

    /// Get console logs for a ticket.
    pub async fn console_logs(
        &self,
        ticket_id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/consolelogs", ticket_id));
        let response = self.client.send(request).await?;
        let body = response.json::<serde_json::Value>().await?;
        Ok(body)
    }

    /// Get network logs for a ticket.
    pub async fn network_logs(
        &self,
        ticket_id: &str,
    ) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/networklogs", ticket_id));
        let response = self.client.send(request).await?;
        let body = response.json::<serde_json::Value>().await?;
        Ok(body)
    }
}
