use clap::Subcommand;

use super::shared::Pagination;

#[derive(Subcommand, Debug)]
pub enum TicketsAction {
    /// List tickets with optional filters
    List {
        /// Filter by status (e.g. OPEN, IN_PROGRESS, DONE)
        #[arg(long)]
        status: Option<String>,

        /// Filter by type (e.g. BUG, FEATURE_REQUEST, INQUIRY)
        #[arg(long, name = "type")]
        ticket_type: Option<String>,

        /// Filter by priority (e.g. LOW, MEDIUM, HIGH)
        #[arg(long)]
        priority: Option<String>,

        /// Sort field with direction prefix (e.g. -createdAt, priority)
        #[arg(long, default_value = "-createdAt")]
        sort: String,

        #[command(flatten)]
        pagination: Pagination,
    },

    /// Get a single ticket by ID
    Get {
        /// Ticket ID
        id: String,
    },

    /// Search tickets by text query
    Search {
        /// Search query text
        query: String,

        #[command(flatten)]
        pagination: Pagination,
    },

    /// Update a ticket
    Update {
        /// Ticket ID
        id: String,

        /// New status
        #[arg(long)]
        status: Option<String>,

        /// New priority
        #[arg(long)]
        priority: Option<String>,

        /// New title
        #[arg(long)]
        title: Option<String>,
    },

    /// Get console logs for a ticket
    ConsoleLogs {
        /// Ticket ID
        id: String,
    },

    /// Get network logs for a ticket
    NetworkLogs {
        /// Ticket ID
        id: String,
    },

    /// Get activity logs for a ticket
    ActivityLogs {
        /// Ticket ID
        id: String,
    },
}
