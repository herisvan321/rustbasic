pub mod session_manager;

use sea_orm::{Database, DatabaseConnection, ConnectOptions};
use crate::config::Config;
use std::time::Duration;

pub async fn connect(cfg: &Config) -> DatabaseConnection {
    // 1. Susun URL Koneksi berdasarkan pilihan di .env
    let db_url = if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        // Default ke SQLite (disimpan di folder database/)
        format!("sqlite://database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    // 2. Konfigurasi Opsi Koneksi
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(10)
       .min_connections(5)
       .connect_timeout(Duration::from_secs(8))
       .idle_timeout(Duration::from_secs(8))
       .max_lifetime(Duration::from_secs(8))
       .sqlx_logging(true);

    // 3. Hubungkan ke Database
    Database::connect(opt)
        .await
        .expect("Gagal terhubung ke database")
}

pub async fn init_sessions(cfg: &Config) {
    let db_url = if cfg.session_driver == "file" {
        "sqlite://database/sessions.sqlite?mode=rwc".to_string()
    } else {
        format!("sqlite://database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    let pool = sqlx::SqlitePool::connect(&db_url).await.expect("Gagal terhubung ke database session");

    // Eksekusi SQL untuk membuat tabel session ala Laravel
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id VARCHAR(255) PRIMARY KEY,
            user_id VARCHAR(255) NULL,
            ip_address VARCHAR(45) NULL,
            user_agent TEXT NULL,
            payload TEXT NOT NULL,
            last_activity INTEGER NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Gagal membuat tabel sessions");

    sqlx::query("CREATE INDEX IF NOT EXISTS sessions_last_activity_index ON sessions(last_activity)")
        .execute(&pool)
        .await
        .expect("Gagal membuat index sessions");
}
