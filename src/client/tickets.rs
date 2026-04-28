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

        self.client.send_and_parse(request).await
    }

    /// Get a single ticket by ID.
    pub async fn get(&self, ticket_id: &str) -> Result<Ticket, AppError> {
        let request = self.client.get(&format!("/tickets/{}", ticket_id));
        self.client.send_and_parse(request).await
    }

    /// Update a ticket by ID. Accepts arbitrary JSON fields.
    pub async fn update(
        &self,
        ticket_id: &str,
        fields: serde_json::Value,
    ) -> Result<Ticket, AppError> {
        let request = self
            .client
            .put(&format!("/tickets/{}", ticket_id))
            .json(&fields);
        self.client.send_and_parse(request).await
    }

    /// Create a new ticket.
    pub async fn create(&self, fields: serde_json::Value) -> Result<Ticket, AppError> {
        let request = self.client.post("/tickets").json(&fields);
        self.client.send_and_parse(request).await
    }

    /// Full-text search tickets.
    pub async fn search(&self, query: &str) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get("/tickets/search")
            .query(&[("searchTerm", query)]);

        self.client.send_and_parse(request).await
    }

    /// Delete a ticket by ID.
    pub async fn delete(&self, ticket_id: &str) -> Result<serde_json::Value, AppError> {
        let request = self.client.delete(&format!("/tickets/{}", ticket_id));
        self.client.send_and_parse(request).await
    }

    /// Get activity logs for a ticket.
    pub async fn activity_logs(&self, ticket_id: &str) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/activity-logs", ticket_id));
        self.client.send_and_parse(request).await
    }

    /// Get console logs for a ticket.
    pub async fn console_logs(&self, ticket_id: &str) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/console-logs", ticket_id));
        self.client.send_and_parse(request).await
    }

    /// Get network logs for a ticket.
    pub async fn network_logs(&self, ticket_id: &str) -> Result<serde_json::Value, AppError> {
        let request = self
            .client
            .get(&format!("/tickets/{}/network-logs", ticket_id));
        self.client.send_and_parse(request).await
    }
}
