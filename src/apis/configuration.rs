// src/config.rs

use once_cell::sync::Lazy;
use reqwest::Client;
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub authorization: String,
    pub user_agent: Option<String>,
    pub client: Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

// Global configuration instance
pub static CONFIG: Lazy<Configuration> = Lazy::new(|| {
    dotenv::dotenv().ok(); // Load environment variables from `.env`

    Configuration {
        base_path: env::var("API_URL").unwrap_or_else(|_| "https://ny.solana.dex.blxrbdn.com".to_string()),
        authorization: env::var("AUTH_HEADER").unwrap_or_else(|_| "MGRiNWNjNGUtYWFjYS00MThjLTgzYjctNmVkMDVmZDMzOThlOjg4Y2VlNzdkZjVmM2NhZWYwZmFjNzFkNmQ0OWVmOGY0".to_string()),
        user_agent: env::var("USER_AGENT").ok(),
        client: Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    }
});

impl Configuration {
    pub fn new() -> Self {
        CONFIG.clone() // Use the static CONFIG instance
    }
}

impl Default for Configuration {
    fn default() -> Self {
        CONFIG.clone() // Use the static CONFIG instance
    }
}
