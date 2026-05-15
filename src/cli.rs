use rustbasic_core::Config;
use rustbasic_core::sea_orm_migration::MigratorTrait;

pub async fn handle(cfg: &Config, args: &[String]) -> bool {
    if args.len() < 2 {
        return false;
    }

    let command = args[1].as_str();
    if !command.starts_with("migrate") && command != "db:seed" {
        return false;
    }

    let db = rustbasic_core::database::connect(cfg).await;

    match command {
        "migrate" => {
            let _ = crate::migrations::Migrator::up(&db, None).await;
        }
        "migrate:refresh" => {
            let _ = crate::migrations::Migrator::fresh(&db).await;
        }
        "migrate:back" | "migrate:rollback" => {
            let _ = crate::migrations::Migrator::down(&db, Some(1)).await;
        }
        "db:seed" => {
            crate::app::seeder::run(&db).await;
        }
        _ => return false,
    }

    true
}
