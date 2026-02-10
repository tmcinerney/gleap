use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    query: &str,
    limit: u64,
    skip: u64,
) -> Result<(), AppError> {
    let response = client
        .tickets()
        .search(query, Some(limit), Some(skip))
        .await?;
    let json = serde_json::to_string_pretty(&response)?;
    println!("{}", json);
    Ok(())
}
