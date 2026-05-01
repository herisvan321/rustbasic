/* ---------------------------------------------------------
 * 📑 LABEL: CONFIG LOADER (config/mod.rs)
 * File ini mengambil pengaturan dari file .env
 * --------------------------------------------------------- */

use std::env;

pub struct Config {
    pub app_name: String,
    pub app_port: u16,
    pub app_host: String,
    #[allow(dead_code)] 
    pub app_key: String,
    
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
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("APP_PORT harus berupa angka"),
            app_host: env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_key: env::var("APP_KEY").unwrap_or_else(|_| "default_secret_key".to_string()),
            
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
}
