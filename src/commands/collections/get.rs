use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, id: &str) -> Result<(), AppError> {
    let collection = client.collections().get(id).await?;
    let json = serde_json::to_string_pretty(&collection)?;
    println!("{}", json);
    Ok(())
}
