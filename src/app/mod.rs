/* ---------------------------------------------------------
 * 📑 LABEL: APPLICATION CORE (app/mod.rs)
 * Entry point untuk modul aplikasi.
 * --------------------------------------------------------- */

pub mod http;
pub mod models;

// Inline module for Inertia integration to avoid extra files
pub mod inertia {
    pub use rustbasic_core::inertia::{inertia, get_vite_assets};
}
pub use inertia::inertia;

// Inline module for database seeders registration
pub mod seeder {
    use rustbasic_core::sql::AnyPool;
    use rustbasic_core::seeder::{SeederTrait, run_seeders};
    use crate::seeders;

    pub struct SeederApp;

    #[rustbasic_core::async_trait]
    impl SeederTrait for SeederApp {
        async fn run<'a>(&'a self, db: &'a AnyPool) -> Result<(), rustbasic_core::sql::Error> {
            let seeders: Vec<Box<dyn SeederTrait + Send + Sync>> = vec![
                Box::new(seeders::database_seeder::DatabaseSeeder),
            ];
            run_seeders(db, seeders).await;
            Ok(())
        }
    }

    pub async fn run(db: &AnyPool) {
        let runner = SeederApp;
        let _ = runner.run(db).await;
    }
}

// Re-export view helpers dari config untuk backward compatibility
pub use rustbasic_core::view::{render, view};
