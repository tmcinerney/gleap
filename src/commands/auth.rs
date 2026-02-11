use gleap::config::keychain;
use gleap::error::AppError;
use std::io::{self, BufRead, Write};

pub fn login() -> Result<(), AppError> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    write!(stdout, "Gleap API key: ").map_err(AppError::Io)?;
    stdout.flush().map_err(AppError::Io)?;
    let mut api_key = String::new();
    stdin.read_line(&mut api_key).map_err(AppError::Io)?;
    let api_key = api_key.trim();

    if api_key.is_empty() {
        return Err(AppError::Config("API key cannot be empty".into()));
    }

    write!(stdout, "Gleap project ID: ").map_err(AppError::Io)?;
    stdout.flush().map_err(AppError::Io)?;
    let mut project_id = String::new();
    stdin.read_line(&mut project_id).map_err(AppError::Io)?;
    let project_id = project_id.trim();

    if project_id.is_empty() {
        return Err(AppError::Config("Project ID cannot be empty".into()));
    }

    keychain::store_credentials(api_key, project_id)?;
    println!("Credentials stored in system keychain.");
    Ok(())
}

pub fn logout() -> Result<(), AppError> {
    keychain::delete_credentials()?;
    println!("Credentials removed from system keychain.");
    Ok(())
}

pub fn status() -> Result<(), AppError> {
    // Check env vars first
    let env_api_key = std::env::var("GLEAP_API_KEY").ok();
    let env_project_id = std::env::var("GLEAP_PROJECT_ID").ok();

    if env_api_key.is_some() && env_project_id.is_some() {
        println!("Authenticated via environment variables");
        println!("  GLEAP_API_KEY:    set");
        println!("  GLEAP_PROJECT_ID: set");
        return Ok(());
    }

    // Check keychain
    match keychain::load_credentials() {
        Ok((_, project_id)) => {
            println!("Authenticated via system keychain");
            println!("  Project ID: {project_id}");
        }
        Err(_) => {
            println!("Not authenticated");
            println!();
            println!("Run `gleap auth login` to store credentials in the system keychain,");
            println!("or set GLEAP_API_KEY and GLEAP_PROJECT_ID environment variables.");
        }
    }

    Ok(())
}
