use gleap::client::GleapClient;
use gleap::error::AppError;
use gleap::models::article::ArticleFilters;

pub async fn run(
    client: &GleapClient,
    collection_id: &str,
    limit: u64,
    skip: u64,
) -> Result<(), AppError> {
    let filters = ArticleFilters {
        limit: Some(limit),
        skip: Some(skip),
    };
    let articles = client.articles().list(collection_id, &filters).await?;
    let json = serde_json::to_string_pretty(&articles)?;
    println!("{}", json);
    Ok(())
}
