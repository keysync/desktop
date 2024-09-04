use tauri::command;

use crate::oauth::{discord, github, google};

#[command]
pub async fn github_login() -> Result<(), String> {
    github::initiate_github_oauth()
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn discord_login() -> Result<(), String> {
    discord::initiate_discord_oauth()
        .await
        .map_err(|e| e.to_string())
}

#[command]
pub async fn google_login() -> Result<(), String> {
    google::initiate_google_oauth()
        .await
        .map_err(|e| e.to_string())
}
