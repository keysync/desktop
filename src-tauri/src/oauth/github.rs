use anyhow::{anyhow, Error, Ok, Result};
use chrono::Utc;
use dotenvy_macro::dotenv;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, RefreshToken, Scope, TokenResponse, TokenUrl,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;

use crate::{
    config::{self, UserProfile},
    Payload,
};

use super::utils::TokenData;

const CLIENT_ID: &str = dotenv!("GITHUB_CLIENT_ID");
const CLIENT_SECRET: &str = dotenv!("GITHUB_CLIENT_SECRET");
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const REDIRECT_URI: &str = "keysync://auth/github/callback";

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubUserProfile {
    pub login: String,
    pub email: Option<String>,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubEmail {
    email: String,
    primary: bool,
    verified: bool,
    visibility: Option<String>,
}

pub async fn initiate_github_oauth(handle: &AppHandle) -> Result<(), Error> {
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(AUTH_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string())?);

    let (auth_url, _csrf_token) = client
        .authorize_url(|| CsrfToken::new_random())
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    let shell = handle.shell();
    shell
        .open(auth_url.to_string(), None)
        .expect("Failed to open GitHub OAuth URL");

    Ok(())
}

pub async fn exchange_code_for_token(handle: &AppHandle, code: String) -> Result<TokenData, Error> {
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(AUTH_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string())?);

    let token_result = client
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await
        .unwrap();

    println!("Token result: {:?}", &token_result);

    let access_token = token_result.access_token().secret().to_string();
    let refresh_token = token_result.refresh_token().unwrap().secret().to_string();
    let expires_in = token_result
        .expires_in()
        .map(|dur| dur.as_secs() as i64)
        .unwrap_or(0);

    let token_data = TokenData {
        access_token: access_token.clone(),
        refresh_token: refresh_token.clone(),
        expiry_timestamp: Utc::now().timestamp() + expires_in,
    };

    let mut config = config::get_config().unwrap_or_default();

    config.accounts.github.access_token = token_data.access_token.clone();
    config.accounts.github.refresh_token = token_data.refresh_token.clone();
    config.accounts.github.expiry_timestamp = token_data.expiry_timestamp;

    config::set_config(&config)?;
    println!("Saved GitHub token data to config: {:?}", &config);

    let window = handle.get_webview_window("main").unwrap();
    let _ = window.emit(
        "github_login",
        Payload {
            message: "GitHub login successful",
        },
    )?;

    Ok(token_data)
}

pub async fn refresh_access_token(refresh_token: String) -> Result<TokenData, Error> {
    let client = BasicClient::new(
        ClientId::new(CLIENT_ID.to_string()),
        Some(ClientSecret::new(CLIENT_SECRET.to_string())),
        AuthUrl::new(AUTH_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string())?);

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await
        .unwrap();

    let access_token = token_result.access_token().secret().to_string();
    let refresh_token = token_result.refresh_token().unwrap().secret().to_string();
    let expires_in = token_result
        .expires_in()
        .map(|dur| dur.as_secs() as i64)
        .unwrap_or(0);

    let token_data = TokenData {
        access_token,
        refresh_token,
        expiry_timestamp: Utc::now().timestamp() + expires_in,
    };

    let mut config = config::get_config().unwrap_or_default();

    config.accounts.github.access_token = token_data.access_token.clone();
    config.accounts.github.refresh_token = token_data.refresh_token.clone();
    config.accounts.github.expiry_timestamp = token_data.expiry_timestamp;

    config::set_config(&config)?;
    println!("Saved refreshed GitHub token data to config: {:?}", &config);

    Ok(token_data)
}

pub async fn get_github_user_info() -> Result<GitHubUserProfile, Error> {
    let mut config = config::get_config()?;
    let current_timestamp = Utc::now().timestamp();
    let access_token = &config.accounts.github.access_token;

    if config.accounts.github.expiry_timestamp <= current_timestamp + 300 {
        println!("GitHub access token has expired or is about to expire, refreshing it");
        let refresh_token = &config.accounts.github.refresh_token;
        let _ = refresh_access_token(refresh_token.to_string()).await?;
    }

    let client = Client::new();
    let url = "https://api.github.com/user";

    let response = client
        .get(url)
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "KeySync")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch user profile: {}",
            response.status()
        ));
    }

    let mut profile: GitHubUserProfile = response.json().await?;

    if profile.email.is_none() {
        println!("GitHub user profile does not have an email, fetching it separately");
        let email = get_github_user_email(access_token).await?;
        profile.email = Some(email);
    }

	config.user_profiles.retain(|p| p.provider != "github");

    config.user_profiles.push(UserProfile {
        provider: "github".to_string(),
        email: profile.email.clone().unwrap(),
        name: profile.login.clone(),
        avatar_url: profile.avatar_url.clone(),
    });
	config::set_config(&config)?;

    Ok(profile)
}

async fn get_github_user_email(access_token: &str) -> Result<String, Error> {
    let client = Client::new();
    let url = "https://api.github.com/user/emails";

    let response = client
        .get(url)
        .header("Authorization", format!("token {}", access_token))
        .header("User-Agent", "KeySync")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch user email: {}",
            response.status()
        ));
    }

    let emails: Vec<GitHubEmail> = response
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse email JSON: {}", e))?;
    let email = emails
        .iter()
        .find(|email| email.primary)
        .or_else(|| emails.iter().find(|email| email.verified))
        .or_else(|| {
            emails
                .iter()
                .find(|email| email.email.ends_with("users.noreply.github.com"))
        })
        .map(|email| email.email.to_string())
        .unwrap_or_else(|| emails[0].email.to_string());

    Ok(email)
}
