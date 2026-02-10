pub mod messages;
pub mod shared;
pub mod tickets;

use clap::{Parser, Subcommand};

pub use messages::MessagesAction;
pub use tickets::{LogsAction, TicketsAction};

#[derive(Parser, Debug)]
#[command(
    name = "gleap",
    about = "Unofficial CLI for the Gleap customer support API",
    version,
    after_help = "Environment variables:\n  GLEAP_API_KEY       Gleap API key (required)\n  GLEAP_PROJECT_ID    Gleap project ID (required)\n  GLEAP_BASE_URL      API base URL (optional, defaults to https://api.gleap.io/v3)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub domain: Domain,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Domain {
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
}
