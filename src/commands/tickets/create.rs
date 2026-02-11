use gleap::client::GleapClient;
use gleap::error::AppError;

pub async fn run(
    client: &GleapClient,
    title: &str,
    ticket_type: Option<String>,
    status: Option<String>,
    priority: Option<String>,
    description: Option<String>,
    tags: Option<String>,
) -> Result<(), AppError> {
    let mut fields = serde_json::Map::new();

    fields.insert("title".into(), serde_json::Value::String(title.into()));

    if let Some(ticket_type) = ticket_type {
        fields.insert("type".into(), serde_json::Value::String(ticket_type));
    }
    if let Some(status) = status {
        fields.insert("status".into(), serde_json::Value::String(status));
    }
    if let Some(priority) = priority {
        fields.insert("priority".into(), serde_json::Value::String(priority));
    }
    if let Some(description) = description {
        fields.insert("description".into(), serde_json::Value::String(description));
    }

    // Always include "gleap-cli" tag, plus any user-provided tags
    let mut tag_list: Vec<serde_json::Value> = tags
        .iter()
        .flat_map(|t| t.split(','))
        .map(|t| serde_json::Value::String(t.trim().to_string()))
        .filter(|t| t.as_str() != Some(""))
        .collect();

    if !tag_list.iter().any(|t| t.as_str() == Some("gleap-cli")) {
        tag_list.push(serde_json::Value::String("gleap-cli".into()));
    }

    fields.insert("tags".into(), serde_json::Value::Array(tag_list));

    let ticket = client
        .tickets()
        .create(serde_json::Value::Object(fields))
        .await?;
    let json = serde_json::to_string_pretty(&ticket)?;
    println!("{}", json);
    Ok(())
}
