use clap::Subcommand;

use super::shared::Pagination;

#[derive(Subcommand, Debug)]
pub enum ArticlesAction {
    /// List articles in a collection
    List {
        /// Collection ID
        #[arg(long)]
        collection: String,

        #[command(flatten)]
        pagination: Pagination,
    },

    /// Get a single article by ID
    Get {
        /// Article ID
        id: String,

        /// Collection ID
        #[arg(long)]
        collection: String,
    },

    /// Create a new article
    Create {
        /// Collection ID
        #[arg(long)]
        collection: String,

        /// Article title
        #[arg(long)]
        title: String,

        /// Path to a JSON file containing Tiptap/ProseMirror content
        #[arg(long)]
        content_file: Option<String>,

        /// Language code for title and content (default: en)
        #[arg(long, default_value = "en")]
        language: String,

        /// Publish the article (default: draft)
        #[arg(long)]
        published: bool,

        /// Comma-separated tags
        #[arg(long)]
        tags: Option<String>,
    },

    /// Update an existing article
    Update {
        /// Article ID
        id: String,

        /// Collection ID
        #[arg(long)]
        collection: String,

        /// New title
        #[arg(long)]
        title: Option<String>,

        /// Path to a JSON file containing updated Tiptap/ProseMirror content
        #[arg(long)]
        content_file: Option<String>,

        /// Language code for title and content (default: en)
        #[arg(long, default_value = "en")]
        language: String,

        /// Publish the article
        #[arg(long)]
        published: Option<bool>,

        /// Comma-separated tags
        #[arg(long)]
        tags: Option<String>,
    },

    /// Delete an article
    Delete {
        /// Article ID
        id: String,

        /// Collection ID
        #[arg(long)]
        collection: String,
    },

    /// Move an article to a different collection
    Move {
        /// Article ID
        id: String,

        /// Current collection ID
        #[arg(long)]
        collection: String,

        /// Target collection ID
        #[arg(long)]
        to: String,
    },
}
