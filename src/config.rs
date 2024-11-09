// src/config.rs

use once_cell::sync::Lazy;
use reqwest::{Client, Error as ReqwestError};
use std::env;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub api_url: String,
    pub auth_header: String,
    pub user_agent: Option<String>,
}

// Static configuration loaded from environment variables
pub static CONFIG: Lazy<Configuration> = Lazy::new(|| {
    dotenv::dotenv().ok(); // Load environment variables from `.env`

    Configuration {
        api_url: env::var("API_URL").unwrap_or_else(|_| "https://ny.solana.dex.blxrbdn.com".to_string()),
        auth_header: env::var("AUTH_HEADER").unwrap_or_else(|_| "your-default-auth-token".to_string()),
        user_agent: env::var("USER_AGENT").ok(),
    }
});

// Static instance of the ApiClient using the static CONFIG
pub static API_CLIENT: Lazy<ApiClient> = Lazy::new(|| {
    ApiClient::new(CONFIG.clone()) // Use the static CONFIG
});

pub struct ApiClient {
    config: Configuration,
    client: Client,
}

impl ApiClient {
    // Initialize a new ApiClient with configuration
    pub fn new(config: Configuration) -> Self {
        ApiClient {
            config,
            client: Client::new(),
        }
    }

    // Generic GET request function with only additional path
    pub async fn get(&self, additional_path: &str) -> Result<String, ReqwestError> {
        // Concatenate base API URL with the additional path
        let url = format!("{}/{}", self.config.api_url.trim_end_matches('/'), additional_path.trim_start_matches('/'));

        // Build and send the request
        let response = self
            .client
            .get(&url)
            .header("Authorization", &self.config.auth_header)
            .header("User-Agent", self.config.user_agent.clone().unwrap_or_else(|| "DefaultUserAgent".to_string()))
            .send()
            .await?;

        response.text().await
    }

    // Add other HTTP methods (POST, PUT, DELETE) as needed in a similar fashion
}
