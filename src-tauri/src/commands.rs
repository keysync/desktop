use anyhow::Result;
use tauri::{command, ipc::InvokeError, AppHandle};

use crate::{
    config::{self, Config},
    oauth::{discord, github, google},
};

#[command]
pub fn get_config() -> Result<Config, InvokeError> {
    config::get_config().map_err(|e| InvokeError::from_anyhow(e))
}

#[command]
pub fn set_config(config: Config) -> Result<Config, InvokeError> {
	config::set_config(&config).map_err(|e| InvokeError::from_anyhow(e))
}

#[command]
pub async fn github_login(handle: AppHandle) -> Result<(), InvokeError> {
    github::initiate_github_oauth(&handle)
        .await
        .map_err(|e| InvokeError::from_anyhow(e))
}

#[command]
pub async fn discord_login(_handle: AppHandle) -> Result<(), InvokeError> {
    discord::initiate_discord_oauth()
        .await
        .map_err(|e| InvokeError::from_anyhow(e))
}

#[command]
pub async fn google_login(_handle: AppHandle) -> Result<(), InvokeError> {
    google::initiate_google_oauth()
        .await
        .map_err(|e| InvokeError::from_anyhow(e))
}
