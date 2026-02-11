use clap::Subcommand;

use super::shared::Pagination;

#[derive(Subcommand, Debug)]
pub enum TicketsAction {
    /// List tickets with optional filters
    List {
        /// Filter by status (e.g. OPEN, INPROGRESS, DONE)
        #[arg(long)]
        status: Option<String>,

        /// Filter by type (e.g. BUG, FEATURE_REQUEST, INQUIRY)
        #[arg(long = "type")]
        ticket_type: Option<String>,

        /// Filter by priority (e.g. LOW, MEDIUM, HIGH)
        #[arg(long)]
        priority: Option<String>,

        /// Sort field with direction prefix (e.g. -createdAt, priority)
        #[arg(long, default_value = "-createdAt", allow_hyphen_values = true)]
        sort: String,

        #[command(flatten)]
        pagination: Pagination,
    },

    /// Get a single ticket by ID
    Get {
        /// Ticket ID
        id: String,
    },

    /// Full-text search tickets (returns up to 30 results ranked by relevance)
    Search {
        /// Search query text
        query: String,
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

    /// View logs captured with a ticket
    Logs {
        #[command(subcommand)]
        action: LogsAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum LogsAction {
    /// Get JavaScript console output
    Console {
        /// Ticket ID
        id: String,
    },

    /// Get HTTP request/response data
    Network {
        /// Ticket ID
        id: String,
    },

    /// Get ticket history (status changes, assignments, etc.)
    Activity {
        /// Ticket ID
        id: String,
    },
}
