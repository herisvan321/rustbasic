use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260501_000001_create_sessions_table::Migration),
            Box::new(m20260501_000002_create_users_table::Migration),
        ]
    }
}

pub mod m20260501_000001_create_sessions_table;
pub mod m20260501_000002_create_users_table;
