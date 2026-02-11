use crate::error::AppError;

const SERVICE: &str = "gleap-cli";
const ACCOUNT_API_KEY: &str = "api-key";
const ACCOUNT_PROJECT_ID: &str = "project-id";

/// Store API key and project ID in the system keychain.
pub fn store_credentials(api_key: &str, project_id: &str) -> Result<(), AppError> {
    let key_entry = keyring::Entry::new(SERVICE, ACCOUNT_API_KEY)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    key_entry
        .set_password(api_key)
        .map_err(|e| AppError::Config(format!("Failed to store API key in keychain: {e}")))?;

    let project_entry = keyring::Entry::new(SERVICE, ACCOUNT_PROJECT_ID)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    project_entry
        .set_password(project_id)
        .map_err(|e| AppError::Config(format!("Failed to store project ID in keychain: {e}")))?;

    Ok(())
}

/// Load API key and project ID from the system keychain.
pub fn load_credentials() -> Result<(String, String), AppError> {
    let key_entry = keyring::Entry::new(SERVICE, ACCOUNT_API_KEY)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    let api_key = key_entry
        .get_password()
        .map_err(|e| AppError::Config(format!("Failed to read API key from keychain: {e}")))?;

    let project_entry = keyring::Entry::new(SERVICE, ACCOUNT_PROJECT_ID)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    let project_id = project_entry
        .get_password()
        .map_err(|e| AppError::Config(format!("Failed to read project ID from keychain: {e}")))?;

    Ok((api_key, project_id))
}

/// Delete credentials from the system keychain.
pub fn delete_credentials() -> Result<(), AppError> {
    let key_entry = keyring::Entry::new(SERVICE, ACCOUNT_API_KEY)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    // Ignore NoEntry errors — credential might not exist
    match key_entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(e) => {
            return Err(AppError::Config(format!(
                "Failed to delete API key from keychain: {e}"
            )));
        }
    }

    let project_entry = keyring::Entry::new(SERVICE, ACCOUNT_PROJECT_ID)
        .map_err(|e| AppError::Config(format!("Failed to access keychain: {e}")))?;
    match project_entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => {}
        Err(e) => {
            return Err(AppError::Config(format!(
                "Failed to delete project ID from keychain: {e}"
            )));
        }
    }

    Ok(())
}
