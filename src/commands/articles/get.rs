use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    collection_id: &str,
    article_id: &str,
) -> Result<(), AppError> {
    let article = client.articles().get(collection_id, article_id).await?;
    let json = serde_json::to_string_pretty(&article)?;
    println!("{}", json);
    Ok(())
}
