use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait, ColumnTrait, QueryFilter};
use rustbasic_core::seeder::SeederTrait;
use crate::app::models::users;
use bcrypt::{hash, DEFAULT_COST};
use colored::Colorize;

pub struct DatabaseSeeder;

#[async_trait::async_trait]
impl SeederTrait for DatabaseSeeder {
    async fn run(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        println!("   {} Sedang memproses DatabaseSeeder...", "⏳".blue());
        
        // 1. Cek apakah user admin sudah ada
        let admin_exists = users::Entity::find()
            .filter(users::Column::Email.eq("admin@rustbasic.com"))
            .one(db)
            .await?
            .is_some();

        if !admin_exists {
            let hashed_password = hash("password123", DEFAULT_COST).unwrap();
            
            let admin = users::ActiveModel {
                name: Set("Administrator".to_owned()),
                email: Set("admin@rustbasic.com".to_owned()),
                password: Set(hashed_password),
                ..Default::default()
            };

            admin.insert(db).await?;
            println!("   {} User admin default berhasil dibuat (admin@rustbasic.com / password123)", "✅".green());
        } else {
            println!("   {} User admin sudah ada, melewati...", "⏩".yellow());
        }

        Ok(())
    }
}
