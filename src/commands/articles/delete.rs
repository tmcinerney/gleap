use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    collection_id: &str,
    article_id: &str,
) -> Result<(), AppError> {
    let result = client.articles().delete(collection_id, article_id).await?;
    let json = serde_json::to_string_pretty(&result)?;
    println!("{}", json);
    Ok(())
}
