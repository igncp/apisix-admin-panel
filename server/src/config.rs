use std::sync::Arc;

use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

pub struct ServerConfig {
    pub admin_url: String,
    pub api_key: String,
    pub apisix_url: String,
    pub config_file_path: String,
    pub control_url: String,
    pub jwt_secret: String,
    pub standalone_config_path: Option<String>,
    pub users: Vec<User>,
}

impl ServerConfig {
    pub fn new() -> Self {
        let api_key = std::env::var("APISIX_API_KEY")
            .unwrap_or_else(|_| "edd1c9f034335f136f87ad84b625c8f1".to_string());

        let admin_url = std::env::var("APISIX_ADMIN_URL")
            .unwrap_or_else(|_| "http://localhost:9180".to_string());

        let apisix_url = std::env::var("APISIX_URL").unwrap_or_else(|_| "".to_string());

        let config_file_path = std::env::var("APISIX_CONFIG_FILE")
            .unwrap_or_else(|_| "/usr/local/apisix/conf/config.yaml".to_string());

        let control_url = std::env::var("APISIX_CONTROL_URL")
            .unwrap_or_else(|_| "http://localhost:9090".to_string());

        let standalone_config_path = match std::env::var("APISIX_STANDALONE_CONFIG") {
            Ok(path) => Some(path),
            Err(_) => None,
        };

        let users_str = std::env::var("APISIX_USERS").unwrap_or_else(|_| "[]".to_string());
        let users =
            serde_json::from_str::<Vec<User>>(&users_str).expect("Failed to parse users JSON");
        let jwt_secret = std::env::var("APISIX_JWT_SECRET").unwrap_or_else(|_| "".to_string());

        Self {
            admin_url,
            api_key,
            apisix_url,
            config_file_path,
            control_url,
            jwt_secret,
            standalone_config_path,
            users,
        }
    }
}

pub type HandlerConfig = web::Data<Arc<ServerConfig>>;
