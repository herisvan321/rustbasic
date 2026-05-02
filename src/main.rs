use tower_http::services::ServeDir;
use dotenvy::dotenv;
use rustbasic::config;
use rustbasic::config::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env & Inisialisasi Logger (Terminal + File)
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menjalankan server.");
    let _guard = config::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 3. Setup Database & Sea-ORM
    config::database::setup_database(&cfg).await;
    let db = config::database::connect(&cfg).await;
    config::database::run_migrations(&db).await; // <-- Jalankan migrasi otomatis
    
    // 4. Inisialisasi Session Store
    config::session::init_sessions(&cfg).await;
    let session_store = config::session::setup_session(&cfg).await;

    // 5. Setup Statics & Jalankan Server
    let static_files = ServeDir::new("public");
    config::server::start_server(cfg, session_store, static_files, db).await;
}
