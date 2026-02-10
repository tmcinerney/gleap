use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, id: &str) -> Result<(), AppError> {
    let ticket = client.tickets().get(id).await?;
    let json = serde_json::to_string_pretty(&ticket)?;
    println!("{}", json);
    Ok(())
}
