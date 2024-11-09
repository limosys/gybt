use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;

pub struct Config {
    pub api_url: String,
    pub auth_header: String,
    pub user_agent: String,
    pub timeout: u32,
}

// Lazy-load the configuration once
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok(); // Load environment variables from `.env`
    
    Config {
        api_url: env::var("API_URL").expect("API_URL must be set"),
        auth_header: env::var("AUTH_HEAER").expect("AUTH_EADER must be set"),
        user_agent: env::var("USER_AGENT").expect("USER_AGENT must be set"),
        timeout: env::var("TIMEOUT")
            .expect("TIMEOUT must be set")
            .parse()
            .expect("TIMEOUT must be a valid integer"),
    }
});
