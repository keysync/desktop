use anyhow::{Error, Result};
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

use crate::config;

use super::utils::TokenData;

const CLIENT_ID: &str = "Iv23li6c6mR30sy0chhJ";
const CLIENT_SECRET: &str = "c3666d466f82f6029eeb04ed92540d5b62dad3b1";
const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const REDIRECT_URI: &str = "keysync://auth/github/callback";

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

pub async fn exchange_code_for_token(code: String) -> Result<TokenData, Error> {
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

    Ok(token_data)
}

/*
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


    Ok(TokenData {
        access_token,
        refresh_token,
        expiry_timestamp: Utc::now().timestamp() + expires_in,
    })
}
*/
