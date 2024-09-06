use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expiry_timestamp: i64,
}

pub fn extract_provider_and_code(link: &str) -> Option<(String, String)> {
    let provider_and_code: Vec<&str> = link
        .split("keysync://auth/")
        .nth(1)?
        .split("/callback?code=")
        .collect();

    if provider_and_code.len() == 2 {
        let provider = provider_and_code[0].to_string();
        let code = provider_and_code[1].split("&").next().unwrap_or("").to_string();
        Some((provider, code))
    } else {
        None
    }
}
