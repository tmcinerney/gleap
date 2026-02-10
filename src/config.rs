use crate::error::AppError;

pub struct GleapConfig {
    pub api_key: String,
    pub project_id: String,
    pub base_url: String,
}

impl GleapConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let api_key = std::env::var("GLEAP_API_KEY")
            .map_err(|_| AppError::Config("GLEAP_API_KEY environment variable not set".into()))?;

        let project_id = std::env::var("GLEAP_PROJECT_ID").map_err(|_| {
            AppError::Config("GLEAP_PROJECT_ID environment variable not set".into())
        })?;

        if api_key.is_empty() {
            return Err(AppError::Config("GLEAP_API_KEY is empty".into()));
        }
        if project_id.is_empty() {
            return Err(AppError::Config("GLEAP_PROJECT_ID is empty".into()));
        }

        let base_url = std::env::var("GLEAP_BASE_URL")
            .unwrap_or_else(|_| "https://api.gleap.io/v3".to_string());

        Ok(Self {
            api_key,
            project_id,
            base_url,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn with_env<F, R>(vars: &[(&str, Option<&str>)], test_fn: F) -> R
    where
        F: FnOnce() -> R,
    {
        let originals: Vec<_> = vars
            .iter()
            .map(|(key, _)| (*key, std::env::var(key).ok()))
            .collect();

        for (key, value) in vars {
            unsafe {
                if let Some(val) = value {
                    std::env::set_var(key, val);
                } else {
                    std::env::remove_var(key);
                }
            }
        }

        let result = test_fn();

        for (key, original) in originals {
            unsafe {
                if let Some(val) = original {
                    std::env::set_var(key, val);
                } else {
                    std::env::remove_var(key);
                }
            }
        }

        result
    }

    #[test]
    #[serial]
    fn test_from_env_success() {
        with_env(
            &[
                ("GLEAP_API_KEY", Some("test-key")),
                ("GLEAP_PROJECT_ID", Some("test-project")),
            ],
            || {
                let config = GleapConfig::from_env();
                assert!(config.is_ok());
                let config = config.unwrap();
                assert_eq!(config.api_key, "test-key");
                assert_eq!(config.project_id, "test-project");
                assert_eq!(config.base_url, "https://api.gleap.io/v3");
            },
        );
    }

    #[test]
    #[serial]
    fn test_from_env_missing_api_key() {
        with_env(
            &[
                ("GLEAP_API_KEY", None),
                ("GLEAP_PROJECT_ID", Some("test-project")),
            ],
            || {
                let result = GleapConfig::from_env();
                assert!(result.is_err());
                if let Err(AppError::Config(msg)) = result {
                    assert!(msg.contains("GLEAP_API_KEY"));
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_from_env_missing_project_id() {
        with_env(
            &[
                ("GLEAP_API_KEY", Some("test-key")),
                ("GLEAP_PROJECT_ID", None),
            ],
            || {
                let result = GleapConfig::from_env();
                assert!(result.is_err());
                if let Err(AppError::Config(msg)) = result {
                    assert!(msg.contains("GLEAP_PROJECT_ID"));
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_from_env_empty_api_key() {
        with_env(
            &[
                ("GLEAP_API_KEY", Some("")),
                ("GLEAP_PROJECT_ID", Some("test-project")),
            ],
            || {
                let result = GleapConfig::from_env();
                assert!(result.is_err());
                if let Err(AppError::Config(msg)) = result {
                    assert!(msg.contains("GLEAP_API_KEY"));
                    assert!(msg.contains("empty"));
                }
            },
        );
    }

    #[test]
    #[serial]
    fn test_from_env_custom_base_url() {
        with_env(
            &[
                ("GLEAP_API_KEY", Some("test-key")),
                ("GLEAP_PROJECT_ID", Some("test-project")),
                ("GLEAP_BASE_URL", Some("http://localhost:8080")),
            ],
            || {
                let config = GleapConfig::from_env().unwrap();
                assert_eq!(config.base_url, "http://localhost:8080");
            },
        );
    }
}
