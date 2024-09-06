use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub accounts: Accounts,
    pub user: Option<UserCredentials>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
    pub github: ProviderConfig,
    pub discord: ProviderConfig,
    pub google: ProviderConfig,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfig {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

fn get_config_path() -> Result<PathBuf, Error> {
    let folder_name = if cfg!(debug_assertions) {
        ".keysync-dev"
    } else {
        ".keysync"
    };

    home_dir()
        .map(|path| path.join(folder_name).join("config.json"))
        .ok_or_else(|| anyhow!("Could not determine home directory"))
}

pub fn get_config() -> Result<Config, Error> {
    let config_path = get_config_path()?;
    let config_content = fs::read_to_string(&config_path)
        .map_err(|e| anyhow!("Failed to read config file from {:?}: {}", config_path, e))?;

    let config = serde_json::from_str::<Config>(&config_content)?;

    Ok(config)
}

pub fn set_config(config: &Config) -> Result<Config, Error> {
    let config_path = get_config_path()?;
    let config_content = serde_json::to_string_pretty(&config)?;

    if let Some(parent) = config_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            return Err(anyhow!("Failed to create config directory {:?}: {}", parent, e));
        }
    }

	fs::write(&config_path, config_content)
	.map_err(|e| anyhow!("Failed to write config file to {:?}: {}", config_path, e))?;

	Ok(config.clone())
}
