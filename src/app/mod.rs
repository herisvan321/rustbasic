/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Entry point untuk modul aplikasi.
 * --------------------------------------------------------- */

pub mod http;
pub mod models;
pub mod seeder;

// Re-export view helpers dari config untuk backward compatibility
pub use rustbasic_core::view::{render, view};
