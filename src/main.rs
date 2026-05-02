use tower_http::services::ServeDir;
use dotenvy::dotenv;
use rustbasic::{config, database};
use rustbasic::config::Config;

#[tokio::main]
async fn main() {
    // 1. Muat file .env & Inisialisasi Logger
    dotenv().ok();
    config::logger::init();

    // 2. Muat Konfigurasi
    let cfg = Config::load();

    // 3. Setup Database & Sea-ORM
    config::database::setup_database(&cfg).await;
    let db = database::connect(&cfg).await;
    
    // 4. Inisialisasi Session Store
    database::init_sessions(&cfg).await;
    let session_store = config::session::setup_session(&cfg).await;

    // 5. Setup Statics & Jalankan Server
    let static_files = ServeDir::new("public");
    config::server::start_server(cfg, session_store, static_files, db).await;
}
