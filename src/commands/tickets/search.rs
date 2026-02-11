use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, query: &str) -> Result<(), AppError> {
    let response = client.tickets().search(query).await?;
    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);
    Ok(())
}
