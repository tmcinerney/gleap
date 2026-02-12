use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    id: &str,
    title: Option<String>,
    description: Option<String>,
) -> Result<(), AppError> {
    let mut fields = serde_json::Map::new();

    if let Some(title) = title {
        fields.insert("title".into(), serde_json::Value::String(title));
    }
    if let Some(description) = description {
        fields.insert("description".into(), serde_json::Value::String(description));
    }

    if fields.is_empty() {
        return Err(AppError::Config(
            "No fields to update. Provide --title or --description.".into(),
        ));
    }

    let collection = client
        .collections()
        .update(id, serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&collection)?;
    println!("{}", json);
    Ok(())
}
