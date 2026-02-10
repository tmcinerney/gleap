use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    id: &str,
    status: Option<String>,
    priority: Option<String>,
    title: Option<String>,
) -> Result<(), AppError> {
    let mut fields = serde_json::Map::new();

    if let Some(status) = status {
        fields.insert("status".into(), serde_json::Value::String(status));
    }
    if let Some(priority) = priority {
        fields.insert("priority".into(), serde_json::Value::String(priority));
    }
    if let Some(title) = title {
        fields.insert("title".into(), serde_json::Value::String(title));
    }

    if fields.is_empty() {
        return Err(AppError::Config(
            "No fields to update. Provide --status, --priority, or --title.".into(),
        ));
    }

    let ticket = client
        .tickets()
        .update(id, serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&ticket)?;
    println!("{}", json);
    Ok(())
}
