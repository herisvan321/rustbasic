mod app;
mod routes;
mod config;
mod database;

use std::net::SocketAddr;
use tower_http::services::ServeDir;
use axum::Router;
use dotenvy::dotenv;
use crate::config::Config;

// 🔑 Import untuk Session
use axum_session::{SessionConfig, SessionStore, SessionLayer};

#[tokio::main]
async fn main() {
    // 1. Muat file .env
    dotenv().ok();

    // 2. Inisialisasi Log (untuk melihat aktivitas server)
    tracing_subscriber::fmt::init();

    // 3. Muat Konfigurasi
    let cfg = Config::load();

    // 4. Hubungkan ke Database (Sea-ORM)
    let _db = database::connect(&cfg).await;
    tracing::info!("Database terhubung menggunakan driver: {}", cfg.db_connection);

    // 5. Inisialisasi Tabel Session (Laravel Style)
    database::init_sessions(&cfg).await;

    // 6. Setup Session dengan Enkripsi (APP_KEY)
    let key_bytes = if cfg.app_key.starts_with("base64:") {
        use base64::{Engine as _, engine::general_purpose};
        general_purpose::STANDARD.decode(&cfg.app_key[7..]).unwrap_or_else(|_| cfg.app_key.as_bytes().to_vec())
    } else {
        cfg.app_key.as_bytes().to_vec()
    };
    
    // cookie::Key butuh tepat 64 bytes. Kita gunakan Sha512 untuk derivasi kunci agar selalu 64 bytes.
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(&key_bytes);
    let final_key = hasher.finalize();
    let session_key = axum_session::Key::from(&final_key);

    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(session_key);

    let session_db_url = if cfg.session_driver == "file" {
        "sqlite://database/sessions.sqlite?mode=rwc".to_string()
    } else {
        format!("sqlite://database/{}.sqlite?mode=rwc", cfg.db_database)
    };

    let session_pool = sqlx::SqlitePool::connect(&session_db_url).await.unwrap();
    let session_store = SessionStore::<database::session_manager::LaravelSessionStore>::new(
        Some(database::session_manager::LaravelSessionStore::new(session_pool)), 
        session_config
    ).await.unwrap();

    tracing::info!("Session aktif menggunakan driver: {} (Encrypted)", cfg.session_driver);

    // 7. Setup Folder Publik (untuk CSS/JS/Gambar)
    let static_files = ServeDir::new("public");

    // 8. Gabungkan Rute dan Jalankan Server dengan Layer Keamanan
    let app = Router::new()
        .merge(routes::web::router())
        .layer(axum::middleware::from_fn(app::http::middleware::csrf::csrf_middleware))
        .layer(axum::middleware::from_fn(app::http::middleware::security_headers::security_headers_middleware))
        .layer(SessionLayer::new(session_store))
        .fallback_service(static_files);

    let addr_str = format!("{}:{}", cfg.app_host, cfg.app_port);
    let addr: SocketAddr = addr_str.parse().expect("Alamat server tidak valid");
    
    tracing::info!("{} berjalan di: http://{}", cfg.app_name, addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
