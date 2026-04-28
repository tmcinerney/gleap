use gleap::client::GleapClient;
use gleap::error::AppError;

pub struct UpdateParams {
    pub title: Option<String>,
    pub content_file: Option<String>,
    pub language: String,
    pub published: Option<bool>,
    pub tags: Option<String>,
}

pub async fn run(
    client: &GleapClient,
    collection_id: &str,
    article_id: &str,
    params: UpdateParams,
) -> Result<(), AppError> {
    let UpdateParams {
        title,
        content_file,
        language,
        published,
        tags,
    } = params;
    let lang = &language;
    let mut fields = serde_json::Map::new();

    if let Some(title) = title {
        fields.insert("title".into(), serde_json::json!({ lang: title }));
    }

    if let Some(ref path) = content_file {
        let raw = std::fs::read_to_string(path).map_err(|e| {
            AppError::Config(format!("Failed to read content file '{}': {}", path, e))
        })?;
        let content_json: serde_json::Value = serde_json::from_str(&raw).map_err(|e| {
            AppError::Config(format!("Invalid JSON in content file '{}': {}", path, e))
        })?;
        fields.insert("content".into(), serde_json::json!({ lang: content_json }));
    }

    if let Some(published) = published {
        fields.insert("isDraft".into(), serde_json::Value::Bool(!published));
    }

    if let Some(tags) = tags {
        let tag_list: Vec<serde_json::Value> = tags
            .split(',')
            .map(|t| serde_json::Value::String(t.trim().to_string()))
            .filter(|t| t.as_str() != Some(""))
            .collect();
        fields.insert("tags".into(), serde_json::Value::Array(tag_list));
    }

    if fields.is_empty() {
        return Err(AppError::Config(
            "No fields to update. Provide --title, --content-file, --published, or --tags.".into(),
        ));
    }

    let article = client
        .articles()
        .update(collection_id, article_id, serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&article)?;
    println!("{}", json);
    Ok(())
}
