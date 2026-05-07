/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Entry point untuk modul aplikasi.
 * --------------------------------------------------------- */

pub mod http;
pub mod models;

// Re-export view helpers dari config untuk backward compatibility
pub use crate::config::view::{render, view};
