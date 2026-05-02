use crate::config::Config;
use sqlx::AnyPool;

pub async fn setup_database(cfg: &Config) {
    // 1. Susun URL Koneksi
    let db_url = if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    // 2. Install Drivers & Hubungkan
    sqlx::any::install_default_drivers();
    let _main_pool = AnyPool::connect(&db_url).await.expect("Gagal terhubung ke database utama (AnyPool)");
    
    // 3. Migrasi sekarang dijalankan secara manual melalui: cargo rustbasic migrate
    // database::run_migrations_any(&main_pool).await;
}
