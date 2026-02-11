use crate::error::AppError;
use serde::{Deserialize, Serialize};

const SERVICE: &str = "gleap-cli";
const ACCOUNT: &str = "credentials";

#[derive(Serialize, Deserialize)]
struct Credentials {
    api_key: String,
    project_id: String,
}

/// Store API key and project ID as a single keychain entry.
pub fn store_credentials(api_key: &str, project_id: &str) -> Result<(), AppError> {
    let creds = Credentials {
        api_key: api_key.to_string(),
        project_id: project_id.to_string(),
    };
    let json = serde_json::to_string(&creds)
        .map_err(|e| AppError::Config(format!("Failed to serialize credentials: {e}")))?;

    let entry = keyring::Entry::new(SERVICE, ACCOUNT)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    entry
        .set_password(&json)
        .map_err(|e| AppError::Config(format!("Failed to store credentials in keychain: {e}")))?;

    Ok(())
}

/// Load API key and project ID from the system keychain.
pub fn load_credentials() -> Result<(String, String), AppError> {
    let entry = keyring::Entry::new(SERVICE, ACCOUNT)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    let json = entry
        .get_password()
        .map_err(|e| AppError::Config(format!("Failed to read credentials from keychain: {e}")))?;
    let creds: Credentials = serde_json::from_str(&json)
        .map_err(|e| AppError::Config(format!("Failed to parse credentials from keychain: {e}")))?;

    Ok((creds.api_key, creds.project_id))
}

/// Delete credentials from the system keychain.
pub fn delete_credentials() -> Result<(), AppError> {
    let entry = keyring::Entry::new(SERVICE, ACCOUNT)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(e) => {
            return Err(AppError::Config(format!(
                "Failed to delete credentials from keychain: {e}"
            )));
        }
    }

    Ok(())
}
