pub use rustbasic_core::*;
pub mod app;
pub mod routes;

#[path = "../database/migrations/mod.rs"]
pub mod migrations;

#[path = "../database/seeders/mod.rs"]
pub mod seeders;
