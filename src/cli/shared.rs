use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Pagination {
    /// Maximum number of results to return
    #[arg(short, long, default_value = "20")]
    pub limit: u64,

    /// Number of results to skip (for pagination)
    #[arg(short, long, default_value = "0")]
    pub skip: u64,
}

#[derive(Args, Debug, Clone)]
pub struct OutputFormat {
    /// Output format: json (default) or table
    #[arg(long, default_value = "json")]
    pub format: String,
}
