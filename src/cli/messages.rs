use clap::Subcommand;

use super::shared::Pagination;

#[derive(Subcommand, Debug)]
pub enum MessagesAction {
    /// List messages for a ticket
    List {
        /// Ticket ID to list messages for
        #[arg(long)]
        ticket: String,

        #[command(flatten)]
        pagination: Pagination,
    },

    /// Add an internal note to a ticket
    Note {
        /// Ticket ID
        #[arg(long)]
        ticket: String,

        /// Note text
        text: String,
    },

    /// Delete a message
    Delete {
        /// Message ID
        id: String,
    },

    /// Add a comment reply to a ticket
    Reply {
        /// Ticket ID
        #[arg(long)]
        ticket: String,

        /// Comment text
        text: String,
    },
}
