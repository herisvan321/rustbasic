/* ---------------------------------------------------------
 * 📑 LABEL: LOGGING MIDDLEWARE (middleware/logging.rs)
 * Mencatat setiap request yang masuk beserta IP pengunjung.
 * Juga mencatat IP ke dalam tracker sesi untuk keamanan database.
 * --------------------------------------------------------- */

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use std::net::SocketAddr;
use colored::*;
use axum_session::Session;
use crate::config::session_manager::{RustBasicSessionStore, IP_TRACKER};

pub async fn logging_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    session: Session<RustBasicSessionStore>,
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let ip = addr.ip().to_string();

    // 1. Simpan IP ke tracker agar bisa diambil oleh DatabasePool saat menyimpan sesi
    IP_TRACKER.insert(session.get_session_id().to_string(), ip.clone());

    // 2. Log ke Terminal (dengan warna)
    let method_str = method.as_str();
    let method_colored = match method_str {
        "GET" => method_str.green(),
        "POST" => method_str.blue(),
        "PUT" => method_str.yellow(),
        "DELETE" => method_str.red(),
        _ => method_str.white(),
    };

    println!(
        "{} {} {:<6} {} from {}",
        chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string().dimmed(),
        "HTTP".magenta().bold(),
        method_colored.bold(),
        path.cyan(),
        ip.yellow()
    );

    // 3. Log ke File (Tanpa warna via tracing)
    tracing::info!(method = %method, path = %path, ip = %ip, "Request");

    next.run(req).await
}
