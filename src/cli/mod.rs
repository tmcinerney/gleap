pub mod auth;
pub mod collections;
pub mod messages;
pub mod shared;
pub mod tickets;

use clap::{ArgAction, Parser, Subcommand};

pub use auth::AuthAction;
pub use collections::CollectionsAction;
pub use messages::MessagesAction;
pub use tickets::{LogsAction, TicketsAction};

#[derive(Parser, Debug)]
#[command(
    name = "gleap",
    about = "Unofficial CLI for the Gleap customer support API",
    version,
    after_help = "Environment variables:\n  GLEAP_API_KEY       Gleap API key (required)\n  GLEAP_PROJECT_ID    Gleap project ID (required)\n  GLEAP_BASE_URL      API base URL (optional, defaults to https://api.gleap.io/v3)\n\nCredentials:\n  Run `gleap auth login` to store credentials in the system keychain"
)]
pub struct Cli {
    #[command(subcommand)]
    pub domain: Domain,

    /// Increase output verbosity (-v for requests, -vv for responses, -vvv for full debug)
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,
}

#[derive(Subcommand, Debug)]
pub enum Domain {
    /// Manage authentication credentials
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },

    /// Manage support tickets
    Tickets {
        #[command(subcommand)]
        action: TicketsAction,
    },

    /// Manage ticket messages and conversations
    Messages {
        #[command(subcommand)]
        action: MessagesAction,
    },

    /// Manage help center collections
    Collections {
        #[command(subcommand)]
        action: CollectionsAction,
    },
}
