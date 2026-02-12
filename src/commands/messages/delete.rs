use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, id: &str) -> Result<(), AppError> {
    let result = client.messages().delete(id).await?;
    let json = serde_json::to_string_pretty(&result)?;
    println!("{}", json);
    Ok(())
}
