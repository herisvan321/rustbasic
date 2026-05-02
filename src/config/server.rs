use axum::{Router, middleware};
use tower_http::services::ServeDir;
use axum_session::{SessionLayer, SessionStore};
use crate::{routes, app, database};
use crate::config::Config;
use std::net::SocketAddr;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::process::Command;

#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: Arc<Config>,
}

pub async fn start_server(
    cfg: Config, 
    session_store: SessionStore<database::session_manager::RustBasicSessionStore>,
    static_files: ServeDir,
    db: DatabaseConnection
) {
    // 0. Kill port jika sedang digunakan (Force Restart)
    kill_port_if_in_use(cfg.app_port);

    // 1. Inisialisasi State
    let state = AppState {
        db,
        config: Arc::new(cfg.clone()),
    };

    // 2. Bangun Router
    let app = Router::new()
        .merge(routes::web::router())
        .nest_service("/public", static_files)
        .layer(middleware::from_fn(app::http::middleware::csrf::csrf_middleware))
        .layer(middleware::from_fn(app::http::middleware::security_headers::security_headers_middleware))
        .layer(SessionLayer::new(session_store))
        .fallback(app::http::controllers::error_controller::ErrorController::not_found)
        .with_state(state);

    // 3. Tentukan Alamat
    let addr_str = format!("{}:{}", cfg.app_host, cfg.app_port);
    let addr: SocketAddr = addr_str.parse().expect("Alamat server tidak valid");
    
    tracing::info!("{} berjalan di: http://{}", cfg.app_name, addr);
    
    // 4. Jalankan Server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Membunuh proses yang menggunakan port tertentu agar tidak terjadi error "Address already in use"
fn kill_port_if_in_use(port: u16) {
    #[cfg(target_os = "macos")]
    {
        // Mencari PID yang menggunakan port tersebut
        let output = Command::new("lsof")
            .arg("-t")
            .arg(format!("-i:{}", port))
            .output();

        if let Ok(out) = output {
            let pid_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !pid_str.is_empty() {
                tracing::warn!("Port {} sedang digunakan oleh PID {}. Membunuh proses...", port, pid_str);
                
                // Membunuh proses tersebut
                for pid in pid_str.split('\n') {
                    let _ = Command::new("kill")
                        .arg("-9")
                        .arg(pid)
                        .output();
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let _ = Command::new("fuser")
            .arg("-k")
            .arg(format!("{}/tcp", port))
            .output();
    }
}
