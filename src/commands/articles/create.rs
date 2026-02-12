use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    collection_id: &str,
    title: &str,
    content_file: Option<String>,
    language: &str,
    published: bool,
    tags: Option<String>,
) -> Result<(), AppError> {
    let mut fields = serde_json::Map::new();

    // Wrap title in language key: {"title": {"en": "..."}}
    fields.insert("title".into(), serde_json::json!({ language: title }));

    // Read and wrap content file if provided
    if let Some(ref path) = content_file {
        let raw = std::fs::read_to_string(path).map_err(|e| {
            AppError::Config(format!("Failed to read content file '{}': {}", path, e))
        })?;
        let content_json: serde_json::Value = serde_json::from_str(&raw).map_err(|e| {
            AppError::Config(format!("Invalid JSON in content file '{}': {}", path, e))
        })?;
        fields.insert(
            "content".into(),
            serde_json::json!({ language: content_json }),
        );
    }

    if published {
        fields.insert("isDraft".into(), serde_json::Value::Bool(false));
    }

    if let Some(tags) = tags {
        let tag_list: Vec<serde_json::Value> = tags
            .split(',')
            .map(|t| serde_json::Value::String(t.trim().to_string()))
            .filter(|t| t.as_str() != Some(""))
            .collect();
        fields.insert("tags".into(), serde_json::Value::Array(tag_list));
    }

    let article = client
        .articles()
        .create(collection_id, serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&article)?;
    println!("{}", json);
    Ok(())
}
