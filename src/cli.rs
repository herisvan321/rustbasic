use rustbasic::app::seeder;
use rustbasic::migrations::Migrator;
use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    rustbasic_core::cli::run_cli(
        || Box::pin(async {
            let cfg = rustbasic_core::Config::load();
            let db = rustbasic_core::database::connect(&cfg).await;
            Migrator::up(&db, None).await.expect("Gagal menjalankan migrasi");
        }),
        || Box::pin(async {
            let cfg = rustbasic_core::Config::load();
            let db = rustbasic_core::database::connect(&cfg).await;
            seeder::run(&db).await;
        })
    ).await;
}
