use crate::migrations::Migrator;
use sea_orm_migration::prelude::*;
use sea_orm::DatabaseConnection;

/// Fungsi untuk menjalankan migrasi menggunakan Sea-ORM Migrator
pub async fn run_migrations(db: &DatabaseConnection) {
    Migrator::up(db, None).await.expect("Gagal menjalankan migrasi Sea-ORM");
}
