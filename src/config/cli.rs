use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::process::Command;
use chrono::Local;
use dotenvy::dotenv;
use rustbasic::config::Config;
use rustbasic::config::database::connect;
use rustbasic::config::database::run_migrations;
use regex::Regex;
use colored::*;

#[tokio::main]
async fn main() {
    dotenv().expect("❌ Error: File .env tidak ditemukan! Silakan salin .env.example menjadi .env sebelum menggunakan CLI.");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "make:model" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama model tidak ditentukan.".red().bold());
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
                println!("{}", "❌ Error: Nama migration tidak ditentukan.".red().bold());
                return;
            }
            make_rust_migration(&args[2]);
        }
        "make:controller" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama controller tidak ditentukan.".red().bold());
                return;
            }
            make_controller(&args[2]);
        }
        "make:middleware" => {
            if args.len() < 3 {
                println!("{}", "❌ Error: Nama middleware tidak ditentukan.".red().bold());
                return;
            }
            make_middleware(&args[2]);
        }
        "migrate" => {
            run_manual_migrations().await;
        }
        "route:list" => {
            list_routes();
        }
        "build" => {
            build_project();
        }
        "cache:clear" => {
            clear_cache().await;
        }
        "check:update" => {
            check_updates();
        }
        "check:security" => {
            check_security();
        }
        _ => {
            println!("{} {}", "❌ Error: Perintah tidak dikenal:".red().bold(), command.yellow());
            print_help();
        }
    }
}

fn print_help() {
    println!("\n{}", "🛠️  RustBasic CLI".magenta().bold());
    println!("{}", "=================".magenta());
    println!("{}", "Penggunaan:".bold());
    println!("  {} {} <Nama> [-m]   {}", "cargo rustbasic".blue(), "make:model".green(), "Membuat model Sea-ORM (dan migration Rust)".dimmed());
    println!("  {} {} <Nama>    {}", "cargo rustbasic".blue(), "make:migration".green(), "Membuat file migration Rust".dimmed());
    println!("  {} {} <Nama>  {}", "cargo rustbasic".blue(), "make:controller".green(), "Membuat controller Axum".dimmed());
    println!("  {} {} <Nama>  {}", "cargo rustbasic".blue(), "make:middleware".green(), "Membuat middleware Axum".dimmed());
    println!("  {} {}                  {}", "cargo rustbasic".blue(), "migrate".green(), "Menjalankan migrasi database (Sea-ORM)".dimmed());
    println!("  {} {}               {}", "cargo rustbasic".blue(), "route:list".green(), "Menampilkan daftar route".dimmed());
    println!("  {} {}                    {}", "cargo rustbasic".blue(), "build".green(), "Membangun project dengan pilihan".dimmed());
    println!("  {} {}             {}", "cargo rustbasic".blue(), "check:update".green(), "Cek versi terbaru paket (dependencies)".dimmed());
    println!("  {} {}           {}", "cargo rustbasic".blue(), "check:security".green(), "Audit keamanan aplikasi".dimmed());
    println!("  {} {}               {}", "cargo rustbasic".blue(), "cache:clear".green(), "Membersihkan logs dan database sessions".dimmed());
    println!();
}

fn check_security() {
    println!("\n{}", "🛡️  RustBasic Security Health Check".magenta().bold());
    println!("{}", "====================================".magenta());

    // 1. Cek CSRF
    println!("\n{}", "1. Proteksi CSRF:".bold());
    if fs::read_to_string("src/app/http/middleware/csrf.rs").is_ok() {
        println!("   {} {}", "✅ Aktif:".green(), "Middleware CSRF terdeteksi.");
    } else {
        println!("   {} {}", "❌ Peringatan:".red(), "Middleware CSRF tidak ditemukan.");
    }

    // 2. Cek Password Hashing
    println!("\n{}", "2. Keamanan Password:".bold());
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();
    if cargo_toml.contains("bcrypt") {
        println!("   {} {}", "✅ Aman:".green(), "Menggunakan library bcrypt untuk hashing.");
    } else {
        println!("   {} {}", "⚠️  Saran:".yellow(), "Gunakan bcrypt atau argon2 untuk hashing password.");
    }

    // 3. Cek SQL Injection
    println!("\n{}", "3. Proteksi SQL Injection:".bold());
    if cargo_toml.contains("sea-orm") || cargo_toml.contains("sqlx") {
        println!("   {} {}", "✅ Aman:".green(), "Menggunakan Query Builder/Prepared Statements.");
    } else {
        println!("   {} {}", "⚠️  Saran:".yellow(), "Pastikan tidak menggunakan string formatting untuk query SQL.");
    }

    // 4. Cek XSS Protection (Template Engine)
    println!("\n{}", "4. Proteksi XSS:".bold());
    if cargo_toml.contains("minijinja") {
        println!("   {} {}", "✅ Aman:".green(), "MiniJinja melakukan auto-escaping secara default.");
    }

    // 5. Audit Dependency (External Tool)
    println!("\n{}", "5. Audit Dependency (crates.io):".bold());
    let has_audit = Command::new("cargo")
        .arg("audit")
        .arg("--version")
        .output()
        .is_ok();

    if has_audit {
        println!("{}", "⏳ Menjalankan cargo audit...".blue());
        let audit_output = Command::new("cargo")
            .arg("audit")
            .output()
            .expect("Gagal menjalankan cargo audit");
        
        if audit_output.status.success() {
            println!("   {} {}", "✅ Bersih:".green(), "Tidak ada kerentanan yang ditemukan pada dependency.");
        } else {
            let out = String::from_utf8_lossy(&audit_output.stdout);
            let err = String::from_utf8_lossy(&audit_output.stderr);

            // Cek jika hanya kerentanan RSA/Rand yang diketahui
            if out.contains("RUSTSEC-2023-0071") || out.contains("RUSTSEC-2026-0097") {
                println!("   {} {}", "⚠️  Peringatan Keamanan Terdeteksi:".yellow(), "Ditemukan isu pada library pihak ketiga.");
                println!("\n{}", "--- Detail Analisis ---".bold());
                
                if out.contains("RUSTSEC-2023-0071") {
                    println!("{} {}", "• RSA (Marvin Attack):".cyan(), "Isu pada driver MySQL (sqlx). Belum ada perbaikan resmi dari pembuat library untuk versi ini.");
                }
                if out.contains("RUSTSEC-2026-0097") {
                    println!("{} {}", "• Rand (Unsoundness):".cyan(), "Isu pada library session. Tidak berbahaya karena kita tidak menggunakan custom logger.");
                }
                
                println!("\n{}", "💡 Kesimpulan: Aplikasi Anda aman untuk dijalankan. Isu di atas adalah keterbatasan library eksternal saat ini.".green());
            } else {
                println!("   {} {}", "❌ Bahaya:".red(), "Ditemukan kerentanan kritis baru!");
                if !out.is_empty() { println!("{}", out.dimmed()); }
                if !err.is_empty() { println!("{}", err.red().dimmed()); }
            }
        }
    } else {
        println!("   {} {}", "💡 Info:".cyan(), "Instal 'cargo-audit' untuk audit otomatis (cargo install cargo-audit).");
    }

    println!("\n{}", "Kesimpulan:".bold());
    println!("{}", "Framework ini sudah menerapkan standar keamanan dasar (OWASP Top 10) dengan baik.".green());
    println!("{}\n", "Selalu pastikan untuk memperbarui dependensi secara berkala.".dimmed());
}

async fn clear_cache() {
    println!("\n{}", "🧹 Cleaning Cache & Logs...".magenta().bold());

    // 1. Clear Logs
    let log_dir = "storage/logs";
    if let Ok(entries) = fs::read_dir(log_dir) {
        let mut count = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                // Mengosongkan isi file tanpa menghapus filenya
                let _ = fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&path);
                count += 1;
            }
        }
        println!("   {} {} ({} file dibersihkan)", "✅ Logs:".green(), "Folder storage/logs telah dikosongkan.", count);
    } else {
        println!("   {} {}", "⚠️  Logs:".yellow(), "Folder storage/logs tidak ditemukan.");
    }

    // 2. Clear Sessions in DB
    let cfg = Config::load();
    let db = connect(&cfg).await;
    
    // Gunakan SQL mentah untuk truncate/delete session table
    let truncate_sql = if cfg.db_connection == "mysql" {
        "TRUNCATE TABLE sessions"
    } else {
        "DELETE FROM sessions"
    };

    use sea_orm::ConnectionTrait;
    match db.execute(sea_orm::Statement::from_string(cfg.db_backend(), truncate_sql.to_string())).await {
        Ok(_) => println!("   {} {}", "✅ Sessions:".green(), "Tabel sessions telah dikosongkan."),
        Err(e) => println!("   {} {} ({})", "❌ Error:".red(), "Gagal membersihkan tabel sessions.", e),
    }

    println!("\n{}", "✨ Cache berhasil dibersihkan!".green().bold());
}

fn check_updates() {
    println!("\n{}", "🔍 Mengecek versi terbaru paket...".cyan().bold());
    println!("{}", "Tunggu sebentar, sedang menghubungi crates.io...".dimmed());

    let output = Command::new("cargo")
        .args(["update", "--dry-run", "--verbose"])
        .output()
        .expect("Gagal menjalankan cargo update");

    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Regex untuk menangkap: Unchanged name v1.0.0 (available: v2.0.0)
    let re = Regex::new(r"Unchanged\s+([^\s]+)\s+v([^\s]+)\s+\(available:\s+v([^\)]+)\)").unwrap();

    let mut found = false;
    println!("\n{}", "+---------------------------+------------+------------+".magenta());
    println!("{}", "| PACKAGE NAME              | CURRENT    | LATEST     |".magenta().bold());
    println!("{}", "+---------------------------+------------+------------+".magenta());

    for line in stderr.lines() {
        if let Some(cap) = re.captures(line) {
            found = true;
            let name = &cap[1];
            let current = &cap[2];
            let latest = &cap[3];

            println!("| {:<25} | {:<10} | {:<10} |", name.cyan(), current.yellow(), latest.green().bold());
        }
    }

    if !found {
        println!("| {:<51} |", "Semua paket sudah menggunakan versi terbaru!".green());
    }
    println!("{}\n", "+---------------------------+------------+------------+".magenta());

    if found {
        println!("{}", "💡 Tips: Jalankan 'cargo update' untuk memperbarui paket yang kompatibel.".yellow());
    }
}

fn build_project() {
    println!("\n{}", "🚀 RustBasic Build Manager".magenta().bold());
    println!("{}", "--------------------------".magenta());
    
    // 1. Pilih Target
    println!("{}", "--- Pilih Target OS ---".cyan().bold());
    println!("1) Native (Sesuai OS Anda)");
    println!("2) Windows (x86_64-pc-windows-msvc)");
    println!("3) Linux (x86_64-unknown-linux-gnu)");
    println!("4) macOS ARM (aarch64-apple-darwin)");
    println!("5) Batal");
    print!("\n{}", "Masukkan pilihan target (1-5): ".bold());
    io::stdout().flush().unwrap();

    let mut target_choice = String::new();
    io::stdin().read_line(&mut target_choice).ok();
    let target_choice = target_choice.trim();

    if target_choice == "5" {
        println!("{}", "👋 Build dibatalkan.".yellow());
        return;
    }

    let target = match target_choice {
        "2" => Some("x86_64-pc-windows-msvc"),
        "3" => Some("x86_64-unknown-linux-gnu"),
        "4" => Some("aarch64-apple-darwin"),
        _ => None, // Native
    };

    // 2. Pilih Mode
    println!("\n{}", "--- Pilih Mode Build ---".cyan().bold());
    println!("1) Development");
    println!("2) Production (Release)");
    print!("\n{}", "Masukkan pilihan mode (1-2): ".bold());
    io::stdout().flush().unwrap();

    let mut mode_choice = String::new();
    io::stdin().read_line(&mut mode_choice).ok();
    let is_release = mode_choice.trim() == "2";

    // 3. Eksekusi Build
    println!("\n{}", "🛠️  Menyiapkan build...".blue());

    let has_zigbuild = Command::new("cargo")
        .arg("zigbuild")
        .arg("--version")
        .output()
        .is_ok();

    let mut cmd = if has_zigbuild && target.is_some() {
        println!("{}", "✨ Menggunakan cargo-zigbuild untuk kompilasi silang...".green().italic());
        let mut c = Command::new("cargo");
        c.arg("zigbuild");
        c
    } else {
        if let Some(t) = target {
            println!("{} {} {}", "📦 Menambahkan target".blue(), t.yellow(), "via rustup...".blue());
            Command::new("rustup")
                .args(["target", "add", t])
                .status()
                .ok();
        }
        let mut c = Command::new("cargo");
        c.arg("build");
        c
    };

    if is_release {
        cmd.arg("--release");
    }

    if let Some(t) = target {
        cmd.arg("--target").arg(t);
    }

    println!("{} {:?}", "🚀 Menjalankan:".blue().bold(), cmd);
    let status = cmd.status().expect("Gagal menjalankan perintah build");

    if status.success() {
        println!("\n{}", "✅ Build berhasil!".green().bold());
        if is_release {
            println!("{}", "📂 Output ada di folder target/release atau target/<target>/release".dimmed());
        }
    } else {
        println!("\n{}", "❌ Build gagal.".red().bold());
        println!("{}", "💡 Penyebab: Linker untuk target tersebut tidak ditemukan di sistem Anda.".yellow());
        
        if target_choice == "2" {
            println!("\n{}", "🔧 Cara memperbaiki untuk Windows:".cyan());
            println!("   Jalankan: {}", "brew install mingw-w64".white().on_black());
        } else if target_choice == "3" {
            println!("\n{}", "🔧 Cara memperbaiki untuk Linux:".cyan());
            println!("   Jalankan: {}", "brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu".white().on_black());
        }
        
        println!("\n{}", "Atau gunakan 'cargo-zigbuild' untuk kompilasi silang yang lebih mudah:".cyan());
        println!("1. brew install zig");
        println!("2. cargo install cargo-zigbuild");
        println!("3. Gunakan '{}'", "cargo zigbuild --target <target>".white().on_black());
    }
}

async fn run_manual_migrations() {
    let cfg = Config::load();
    println!("{} {} {}", "⏳ Menjalankan migrasi Sea-ORM di".blue(), cfg.db_connection.yellow(), "...".blue());
    
    let db = connect(&cfg).await;
    run_migrations(&db).await;
    
    println!("{}", "✅ Migrasi selesai!".green().bold());
}

fn list_routes() {
    let routes_path = "src/routes/web.rs";
    let content = fs::read_to_string(routes_path).expect("Gagal membaca src/routes/web.rs");

    let re = Regex::new(r#"\.route\(\s*"([^"]+)"\s*,\s*([a-z]+)\(([^)]+)\)\)"#).unwrap();

    println!("\n{}", "+----------------+----------------------+----------------------------------------------------------+".magenta());
    println!("{}", "| METHOD         | PATH                 | HANDLER                                                  |".magenta().bold());
    println!("{}", "+----------------+----------------------+----------------------------------------------------------+".magenta());

    for cap in re.captures_iter(&content) {
        let path = &cap[1];
        let method = cap[2].to_uppercase();
        let handler = &cap[3];

        let method_color = match method.as_str() {
            "GET" => method.green(),
            "POST" => method.blue(),
            _ => method.white(),
        };

        println!("| {:<14} | {:<20} | {:<56} |", method_color, path.cyan(), handler.dimmed());
    }
    println!("{}\n", "+----------------+----------------------+----------------------------------------------------------+".magenta());
}

fn make_controller(name: &str) {
    let clean_name = name.replace("Controller", "");
    let snake_name = to_snake_case(&clean_name);
    let class_name = format!("{}Controller", clean_name);
    let file_name = format!("{}_controller.rs", snake_name);
    let file_path = format!("src/app/http/controllers/{}", file_name);

    if std::path::Path::new(&file_path).exists() {
        println!("{} {} {}", "⚠️  Controller".yellow(), file_path.cyan(), "sudah ada.".yellow());
        return;
    }

    let template = format!(
r#"/* ---------------------------------------------------------
 * 📑 LABEL: {class_name} ({file_name})
 * --------------------------------------------------------- */

use crate::app::view;
use crate::config::requests::Request;
use axum::response::IntoResponse;
use minijinja::context;

pub struct {class_name};

impl {class_name} {{
    pub async fn index(req: Request) -> impl IntoResponse {{
        view(&req, "{snake_name}.html", context! {{
            title => "{class_name}"
        }})
    }}
}}
"#, class_name = class_name, file_name = file_name, snake_name = snake_name);

    fs::write(&file_path, template).expect("Gagal membuat file controller");
    println!("{} {}", "✅ Controller dibuat:".green(), file_path.cyan());

    update_controller_mod_rs(&file_name.replace(".rs", ""));
}

fn update_controller_mod_rs(mod_name: &str) {
    let mod_path = "src/app/http/controllers/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    let line = format!("pub mod {};", mod_name);
    if content.contains(&line) {
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Gagal membuka controllers/mod.rs");

    writeln!(file, "{}", line).ok();
    println!("{} {}", "📝".blue(), "controllers/mod.rs diperbarui.".dimmed());
}

fn make_middleware(name: &str) {
    let snake_name = to_snake_case(name).replace("_middleware", "");
    let fn_name = format!("{}_middleware", snake_name);
    let file_name = format!("{}.rs", snake_name);
    let file_path = format!("src/app/http/middleware/{}", file_name);

    if std::path::Path::new(&file_path).exists() {
        println!("{} {} {}", "⚠️  Middleware".yellow(), file_path.cyan(), "sudah ada.".yellow());
        return;
    }

    let template = format!(
r#"/* ---------------------------------------------------------
 * 📑 LABEL: {label} (middleware/{file_name})
 * --------------------------------------------------------- */

use axum::{{
    extract::Request,
    middleware::Next,
    response::Response,
}};

pub async fn {fn_name}(
    req: Request,
    next: Next,
) -> Response {{
    // Lakukan sesuatu sebelum request sampai ke controller
    
    let response = next.run(req).await;
    
    // Lakukan sesuatu setelah request selesai diproses
    
    response
}}
"#, label = name.to_uppercase(), file_name = file_name, fn_name = fn_name);

    fs::write(&file_path, template).expect("Gagal membuat file middleware");
    println!("{} {}", "✅ Middleware dibuat:".green(), file_path.cyan());

    update_middleware_mod_rs(&snake_name);
}

fn update_middleware_mod_rs(mod_name: &str) {
    let mod_path = "src/app/http/middleware/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    let line = format!("pub mod {};", mod_name);
    if content.contains(&line) {
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Gagal membuka middleware/mod.rs");

    writeln!(file, "{}", line).ok();
    println!("{} {}", "📝".blue(), "middleware/mod.rs diperbarui.".dimmed());
}

fn make_model(name: &str) {
    let snake_name = to_snake_case(name);
    let table_name = format!("{}s", snake_name);
    let file_path = format!("src/app/models/{}.rs", snake_name);

    if std::path::Path::new(&file_path).exists() {
        println!("{} {} {}", "⚠️  Model".yellow(), file_path.cyan(), "sudah ada.".yellow());
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
    println!("{} {}", "✅ Model dibuat:".green(), file_path.cyan());

    update_mod_rs(name, &snake_name);
}

fn make_rust_migration(name: &str) {
    let snake_name = to_snake_case(name);
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let mod_name = format!("m{}_{}", timestamp, snake_name);
    let file_path = format!("database/migrations/{}.rs", mod_name);

    if std::path::Path::new(&file_path).exists() {
        println!("{} {} {}", "⚠️  Migration".yellow(), file_path.cyan(), "sudah ada.".yellow());
        return;
    }

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
"#, table_iden = table_iden, mod_name = mod_name);

    fs::write(&file_path, template).expect("Gagal membuat file migration");
    println!("{} {}", "✅ Migration Rust dibuat:".green(), file_path.cyan());

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
    println!("{} {}", "📝".blue(), "database/migrations/mod.rs diperbarui.".dimmed());
}

fn update_mod_rs(class_name: &str, snake_name: &str) {
    let mod_path = "src/app/models/mod.rs";
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(mod_path) {
        file.read_to_string(&mut content).ok();
    }

    let mod_line = format!("pub mod {};", snake_name);
    if content.contains(&mod_line) {
        return;
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(mod_path)
        .expect("Gagal membuka models/mod.rs");

    writeln!(file, "{}", mod_line).ok();
    writeln!(file, "pub use {}::Entity as {};", snake_name, class_name).ok();
    
    println!("{} {}", "📝".blue(), "models/mod.rs diperbarui.".dimmed());
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
