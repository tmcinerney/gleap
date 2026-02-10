use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(client: &GleapClient, ticket: &str, text: &str) -> Result<(), AppError> {
    let message = client.messages().create_comment(ticket, text).await?;
    let json = serde_json::to_string_pretty(&message)?;
    println!("{}", json);
    Ok(())
}
