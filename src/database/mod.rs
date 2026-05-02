/* ---------------------------------------------------------
 * 📑 LABEL: DATABASE MANAGER (database/mod.rs)
 * File ini mengelola modul-modul terkait database.
 * --------------------------------------------------------- */

pub mod connection;
pub mod migrations;
pub mod session;
pub mod session_manager;

// Re-export untuk akses mudah
pub use connection::connect;
pub use migrations::run_migrations;
pub use session::init_sessions;
