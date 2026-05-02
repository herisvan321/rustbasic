use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;
use dotenvy::dotenv;
use rustbasic::config::Config;
use rustbasic::database::connect;
use rustbasic::database::run_migrations;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "make:model" => {
            if args.len() < 3 {
                println!("❌ Error: Nama model tidak ditentukan.");
                return;
            }
            let model_name = &args[2];
            let with_migration = args.contains(&"-m".to_string());
            
            make_model(model_name);
            if with_migration {
                make_rust_migration(model_name);
            }
        }
        "make:migration" => {
            if args.len() < 3 {
                println!("❌ Error: Nama migration tidak ditentukan.");
                return;
            }
            make_rust_migration(&args[2]);
        }
        "migrate" => {
            run_manual_migrations().await;
        }
        _ => {
            println!("❌ Error: Perintah tidak dikenal: {}", command);
            print_help();
        }
    }
}

fn print_help() {
    println!("🛠️  RustBasic CLI");
    println!("Penggunaan:");
    println!("  cargo rustbasic make:model <Nama> [-m]   Membuat model Sea-ORM (dan migration Rust)");
    println!("  cargo rustbasic make:migration <Nama>    Membuat file migration Rust");
    println!("  cargo rustbasic migrate                  Menjalankan migrasi database (Sea-ORM)");
}

async fn run_manual_migrations() {
    let cfg = Config::load();
    println!("⏳ Menjalankan migrasi Sea-ORM di {}...", cfg.db_connection);
    
    let db = connect(&cfg).await;
    run_migrations(&db).await;
    
    println!("✅ Migrasi selesai!");
}

fn make_model(name: &str) {
    let snake_name = to_snake_case(name);
    let table_name = format!("{}s", snake_name);
    let file_path = format!("src/app/models/{}.rs", snake_name);

    if std::path::Path::new(&file_path).exists() {
        println!("⚠️  Model {} sudah ada.", file_path);
        return;
    }

    let template = format!(
r#"use sea_orm::entity::prelude::*;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {{}}

impl ActiveModelBehavior for ActiveModel {{}}
"#, table_name);

    fs::write(&file_path, template).expect("Gagal membuat file model");
    println!("✅ Model dibuat: {}", file_path);

    update_mod_rs(name, &snake_name);
}

fn make_rust_migration(name: &str) {
    let snake_name = to_snake_case(name);
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let mod_name = format!("m{}_{}", timestamp, snake_name);
    let file_path = format!("database/migrations/{}.rs", mod_name);

    let struct_name = name.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let table_iden = format!("{}s", struct_name);

    let template = format!(
r#"use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum {table_iden} {{
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
}}

#[derive(Iden)]
pub struct Migration;

impl MigrationName for Migration {{
    fn name(&self) -> &str {{
        "{mod_name}"
    }}
}}

#[async_trait::async_trait]
impl MigrationTrait for Migration {{
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .create_table(
                Table::create()
                    .table({table_iden}::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new({table_iden}::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new({table_iden}::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new({table_iden}::UpdatedAt)
                            .date_time()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }}

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {{
        manager
            .drop_table(Table::drop().table({table_iden}::Table).to_owned())
            .await
    }}
}}
"#, table_iden = table_iden);

    fs::write(&file_path, template).expect("Gagal membuat file migration");
    println!("✅ Migration Rust dibuat: {}", file_path);

    update_migration_mod_rs(&mod_name);
}

fn update_migration_mod_rs(mod_name: &str) {
    let mod_path = "database/migrations/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    // Tambahkan mod declaration
    if !content.contains(&format!("pub mod {};", mod_name)) {
        content.push_str(&format!("pub mod {};\n", mod_name));
    }

    // Tambahkan ke list migrations
    let search_pattern = "fn migrations() -> Vec<Box<dyn MigrationTrait>> {";
    if let Some(_pos) = content.find(search_pattern) {
        let insert_pos = content.find("        ]").unwrap_or(content.len());
        content.insert_str(insert_pos, &format!("            Box::new({}::Migration),\n", mod_name));
    }

    fs::write(mod_path, content).expect("Gagal memperbarui database/migrations/mod.rs");
    println!("📝 database/migrations/mod.rs diperbarui.");
}

fn update_mod_rs(class_name: &str, snake_name: &str) {
    let mod_path = "src/app/models/mod.rs";
    let mut file = OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Gagal membuka models/mod.rs");

    writeln!(file, "pub mod {};", snake_name).ok();
    writeln!(file, "pub use {}::Entity as {};", snake_name, class_name).ok();
    
    println!("📝 models/mod.rs diperbarui.");
}

fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}
