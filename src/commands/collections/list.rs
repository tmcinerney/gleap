use gleap::client::GleapClient;
use gleap::error::AppError;
use gleap::models::collection::CollectionFilters;

pub async fn run(client: &GleapClient, limit: u64, skip: u64) -> Result<(), AppError> {
    let filters = CollectionFilters {
        limit: Some(limit),
        skip: Some(skip),
    };
    let collections = client.collections().list(&filters).await?;
    let json = serde_json::to_string_pretty(&collections)?;
    println!("{}", json);
    Ok(())
}
