use gleap::client::GleapClient;
use gleap::error::AppError;
use gleap::models::message::MessageFilters;

pub async fn run(
    client: &GleapClient,
    ticket: &str,
    sort: String,
    limit: u64,
    skip: u64,
) -> Result<(), AppError> {
    let filters = MessageFilters {
        ticket: Some(ticket.to_string()),
        sort: Some(sort),
        limit: Some(limit),
        skip: Some(skip),
        ..Default::default()
    };

    let messages = client.messages().list(&filters).await?;
    let json = serde_json::to_string_pretty(&messages)?;
    println!("{}", json);
    Ok(())
}
