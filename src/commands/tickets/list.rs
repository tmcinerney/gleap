use gleap::client::GleapClient;
use gleap::error::AppError;
use gleap::models::ticket::TicketFilters;

pub async fn run(
    client: &GleapClient,
    status: Option<String>,
    ticket_type: Option<String>,
    priority: Option<String>,
    sort: String,
    limit: u64,
    skip: u64,
) -> Result<(), AppError> {
    let filters = TicketFilters {
        status,
        ticket_type,
        priority,
        sort: Some(sort),
        limit: Some(limit),
        skip: Some(skip),
        archived: Some(false),
        is_spam: Some(false),
    };

    let response = client.tickets().list(&filters).await?;
    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);
    Ok(())
}
