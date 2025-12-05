use std::{fs, path::Path};

use serde::Deserialize;

use crate::core::error::FalconError;

#[derive(Debug, Clone, Deserialize)]
pub struct ProviderCfg {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub providers: Vec<ProviderCfg>,
    pub timeout_ms: u64,
    pub max_concurrent_scans: usize,
    pub cache_ttl_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                ProviderCfg {
                    name: "GitHub".to_string(),
                    url: "https://github.com/{username}".to_string(),
                },
                ProviderCfg {
                    name: "Reddit".to_string(),
                    url: "https://www.reddit.com/user/{username}".to_string(),
                },
                ProviderCfg {
                    name: "Steam".to_string(),
                    url: "https://steamcommunity.com/id/{username}".to_string(),
                },
                ProviderCfg {
                    name: "Twitter".to_string(),
                    url: "https://twitter.com/{username}".to_string(),
                },
                ProviderCfg {
                    name: "PSNProfiles".to_string(),
                    url: "https://psnprofiles.com/{username}".to_string(),
                },
            ],
            timeout_ms: 4000,
            max_concurrent_scans: 5,
            cache_ttl_secs: 900,
        }
    }
}

pub fn load_config(path: &Path) -> Result<AppConfig, FalconError> {
    if !path.exists() {
        return Ok(AppConfig::default());
    }

    let content = fs::read_to_string(path)?;
    let cfg: AppConfig =
        toml::from_str(&content).map_err(|e| FalconError::Config(e.to_string()))?;
    Ok(cfg)
}

pub fn apply_provider_filter(cfg: AppConfig, names: Option<&[String]>) -> AppConfig {
    if let Some(list) = names {
        let current = cfg.providers.clone();
        let filtered: Vec<ProviderCfg> = current
            .iter()
            .cloned()
            .filter(|p| list.iter().any(|n| n.eq_ignore_ascii_case(&p.name)))
            .collect();
        return AppConfig {
            providers: if filtered.is_empty() { current } else { filtered },
            ..cfg
        };
    }
    cfg
}
