use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    title: &str,
    description: Option<String>,
) -> Result<(), AppError> {
    let mut fields = serde_json::Map::new();
    fields.insert("title".into(), serde_json::Value::String(title.into()));

    if let Some(description) = description {
        fields.insert("description".into(), serde_json::Value::String(description));
    }

    let collection = client
        .collections()
        .create(serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&collection)?;
    println!("{}", json);
    Ok(())
}
