use crate::error::AppError;
use crate::models::message::{CreateMessageRequest, Message, MessageFilters};

use super::GleapClient;

pub struct MessagesClient<'a> {
    client: &'a GleapClient,
}

impl<'a> MessagesClient<'a> {
    pub(crate) fn new(client: &'a GleapClient) -> Self {
        Self { client }
    }

    /// List messages with optional filters.
    pub async fn list(&self, filters: &MessageFilters) -> Result<Vec<Message>, AppError> {
        let mut request = self.client.get("/messages");

        if let Some(ref ticket) = filters.ticket {
            request = request.query(&[("ticket", ticket.as_str())]);
        }
        if let Some(ref message_type) = filters.message_type {
            request = request.query(&[("type", message_type.as_str())]);
        }
        if let Some(bot) = filters.bot {
            request = request.query(&[("bot", &bot.to_string())]);
        }
        if let Some(limit) = filters.limit {
            request = request.query(&[("limit", &limit.to_string())]);
        }
        if let Some(skip) = filters.skip {
            request = request.query(&[("skip", &skip.to_string())]);
        }

        self.client.send_and_parse(request).await
    }

    /// Create a new message (comment or internal note) on a ticket.
    pub async fn create(&self, request: &CreateMessageRequest) -> Result<Message, AppError> {
        let req = self.client.post("/messages").json(request);
        self.client.send_and_parse(req).await
    }

    /// Convenience: create an internal note on a ticket.
    pub async fn create_note(&self, ticket_id: &str, text: &str) -> Result<Message, AppError> {
        let request = CreateMessageRequest {
            ticket: ticket_id.to_string(),
            comment: serde_json::Value::String(text.to_string()),
            is_note: Some(true),
            session: None,
            attachments: None,
        };
        self.create(&request).await
    }

    /// Convenience: create a comment reply on a ticket.
    pub async fn create_comment(&self, ticket_id: &str, text: &str) -> Result<Message, AppError> {
        let request = CreateMessageRequest {
            ticket: ticket_id.to_string(),
            comment: serde_json::Value::String(text.to_string()),
            is_note: None,
            session: None,
            attachments: None,
        };
        self.create(&request).await
    }
}
