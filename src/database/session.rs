use crate::config::Config;

pub async fn init_sessions(cfg: &Config) {
    let db_url = if cfg.session_driver == "file" {
        "sqlite:database/sessions.sqlite?mode=rwc".to_string()
    } else if cfg.db_connection == "mysql" {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.db_username, cfg.db_password, cfg.db_host, cfg.db_port, cfg.db_database
        )
    } else {
        format!("sqlite:database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    sqlx::any::install_default_drivers();
    let _pool = sqlx::AnyPool::connect(&db_url).await.expect("Gagal terhubung ke database session");

    // Jalankan migrasi secara manual: cargo rustbasic migrate
    // migrations::run_migrations_any(&pool).await;
}
