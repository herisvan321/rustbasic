pub use rustbasic_core::*;
pub mod app;
pub mod routes;

#[path = "../database/migrations/mod.rs"]
pub mod migrations;

#[path = "../database/seeders/mod.rs"]
pub mod seeders;

// Bootstrapping configurations directly in lib.rs
rustbasic_core::bootstrap_config!();

// Native (Mobile & Desktop) Entry Point
#[cfg(any(target_os = "android", target_os = "ios"))]
rustbasic_native::setup_native!();
