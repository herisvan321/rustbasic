/* ---------------------------------------------------------
 * 📑 LABEL: CONFIG LOADER (config/mod.rs)
 * File ini mengambil pengaturan dari file .env
 * --------------------------------------------------------- */

use std::env;

pub struct Config {
    pub app_name: String,
    pub app_port: u16,
    pub app_host: String,
    #[allow(dead_code)] // 🛠️ Sembunyikan warning karena akan digunakan nanti
    pub app_key: String, 
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
            // Ambil APP_KEY dari .env
            app_key: env::var("APP_KEY").unwrap_or_else(|_| "default_secret_key".to_string()),
        }
    }
}
