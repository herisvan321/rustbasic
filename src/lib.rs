pub use rustbasic_core::*;
pub mod app;
pub mod routes;
pub mod config;

#[path = "../database/migrations/mod.rs"]
pub mod migrations;

#[path = "../database/seeders/mod.rs"]
pub mod seeders;





// Native (Mobile & Desktop) Entry Point
#[cfg(any(target_os = "android", target_os = "ios"))]
rustbasic_native::setup_native!();
