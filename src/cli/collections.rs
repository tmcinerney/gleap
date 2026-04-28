use clap::Subcommand;

use super::shared::Pagination;

#[derive(Subcommand, Debug)]
pub enum CollectionsAction {
    /// List all collections
    List {
        #[command(flatten)]
        pagination: Pagination,
    },

    /// Get a single collection by ID
    Get {
        /// Collection ID
        id: String,
    },

    /// Create a new collection
    Create {
        /// Collection title
        #[arg(long)]
        title: String,

        /// Collection description
        #[arg(long)]
        description: Option<String>,
    },

    /// Update an existing collection
    Update {
        /// Collection ID
        id: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// New description
        #[arg(long)]
        description: Option<String>,
    },

    /// Delete a collection
    Delete {
        /// Collection ID
        id: String,
    },
}
