use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, id: &str) -> Result<(), AppError> {
    let logs = client.tickets().activity_logs(id).await?;
    let json = serde_json::to_string_pretty(&logs)?;
    println!("{}", json);
    Ok(())
}
