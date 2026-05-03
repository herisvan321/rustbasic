/* ---------------------------------------------------------
 * 📑 LABEL: APP CONFIG (config/app.rs)
 * Definisi struct Config dan pengisian datanya dari .env
 * --------------------------------------------------------- */

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub app_name: String,
    pub app_port: u16,
    pub app_host: String,
    pub app_key: String,
    pub app_debug: bool,
    pub app_timezone: String,
    pub app_limit_request: u64,
    
    // 🗄️ Database
    pub db_connection: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_database: String,
    pub db_username: String,
    pub db_password: String,
    
    // 🔑 Session
    pub session_driver: String,
}

impl Config {
    pub fn load() -> Self {
        Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "RustBasic".to_string()),
            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "4000".to_string())
                .parse()
                .expect("APP_PORT harus berupa angka"),
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_key: env::var("APP_KEY").unwrap_or_else(|_| "default_secret_key".to_string()),
            app_debug: env::var("APP_DEBUG").unwrap_or_else(|_| "false".to_string()) == "true",
            app_timezone: env::var("APP_TIMEZONE").unwrap_or_else(|_| "UTC".to_string()),
            app_limit_request: env::var("APP_LIMIT_REQUEST")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .expect("APP_LIMIT_REQUEST harus berupa angka"),
            
            // Database
            db_connection: env::var("DB_CONNECTION").unwrap_or_else(|_| "sqlite".to_string()),
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            db_port: env::var("DB_PORT")
                .unwrap_or_else(|_| "3306".to_string())
                .parse()
                .expect("DB_PORT harus berupa angka"),
            db_database: env::var("DB_DATABASE").unwrap_or_else(|_| "rustbasic".to_string()),
            db_username: env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or_default(),
            
            // Session
            session_driver: env::var("SESSION_DRIVER").unwrap_or_else(|_| "database".to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn db_backend(&self) -> sea_orm::DbBackend {
        if self.db_connection == "mysql" {
            sea_orm::DbBackend::MySql
        } else {
            sea_orm::DbBackend::Sqlite
        }
    }
}
