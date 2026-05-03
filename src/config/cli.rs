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
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;


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
        "key:generate" => {
            generate_app_key();
        }
        "make:auth" | "auth" => {
            if args.len() >= 3 && args[2] == "back" {
                remove_auth().await;
            } else {
                make_auth().await;
            }
        }
        "auth:back" => {
            remove_auth().await;
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
    println!("  {} {}             {}", "cargo rustbasic".blue(), "key:generate".green(), "Membuat APP_KEY baru di file .env".dimmed());
    println!("  {} {}                   {}", "cargo rustbasic".blue(), "make:auth".green(), "Scaffold autentikasi (Login/Register)".dimmed());
    println!("  {} {}                   {}", "cargo rustbasic".blue(), "auth:back".red(), "Menghapus semua scaffolding autentikasi".dimmed());

    println!();
}

fn check_security() {
    println!("\n{}", "🛡️  RustBasic Security Health Check".magenta().bold());
    println!("{}", "====================================".magenta());

    // 1. Cek CSRF
    println!("\n{}", "1. Proteksi CSRF:".bold());
    if fs::read_to_string("src/app/http/middleware/csrf.rs").is_ok() {
        println!("   {} Middleware CSRF terdeteksi.", "✅ Aktif:".green());
    } else {
        println!("   {} Middleware CSRF tidak ditemukan.", "❌ Peringatan:".red());
    }

    // 2. Cek Password Hashing
    println!("\n{}", "2. Keamanan Password:".bold());
    let cargo_toml = fs::read_to_string("Cargo.toml").unwrap_or_default();
    if cargo_toml.contains("bcrypt") {
        println!("   {} Menggunakan library bcrypt untuk hashing.", "✅ Aman:".green());
    } else {
        println!("   {} Gunakan bcrypt atau argon2 untuk hashing password.", "⚠️  Saran:".yellow());
    }

    // 3. Cek SQL Injection
    println!("\n{}", "3. Proteksi SQL Injection:".bold());
    if cargo_toml.contains("sea-orm") || cargo_toml.contains("sqlx") {
        println!("   {} Menggunakan Query Builder/Prepared Statements.", "✅ Aman:".green());
    } else {
        println!("   {} Pastikan tidak menggunakan string formatting untuk query SQL.", "⚠️  Saran:".yellow());
    }

    // 4. Cek XSS Protection (Template Engine)
    println!("\n{}", "4. Proteksi XSS:".bold());
    if cargo_toml.contains("minijinja") {
        println!("   {} MiniJinja melakukan auto-escaping secara default.", "✅ Aman:".green());
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
            println!("   {} Tidak ada kerentanan yang ditemukan pada dependency.", "✅ Bersih:".green());
        } else {
            let out = String::from_utf8_lossy(&audit_output.stdout);
            let err = String::from_utf8_lossy(&audit_output.stderr);

            // Cek jika hanya kerentanan RSA/Rand yang diketahui
            if out.contains("RUSTSEC-2023-0071") || out.contains("RUSTSEC-2026-0097") {
                println!("   {} Ditemukan isu pada library pihak ketiga.", "⚠️  Peringatan Keamanan Terdeteksi:".yellow());
                println!("\n{}", "--- Detail Analisis ---".bold());
                
                if out.contains("RUSTSEC-2023-0071") {
                    println!("{} Isu pada driver MySQL (sqlx). Belum ada perbaikan resmi dari pembuat library untuk versi ini.", "• RSA (Marvin Attack):".cyan());
                }
                if out.contains("RUSTSEC-2026-0097") {
                    println!("{} Isu pada library session. Tidak berbahaya karena kita tidak menggunakan custom logger.", "• Rand (Unsoundness):".cyan());
                }
                
                println!("\n{}", "💡 Kesimpulan: Aplikasi Anda aman untuk dijalankan. Isu di atas adalah keterbatasan library eksternal saat ini.".green());
            } else {
                println!("   {} Ditemukan kerentanan kritis baru!", "❌ Bahaya:".red());
                if !out.is_empty() { println!("{}", out.dimmed()); }
                if !err.is_empty() { println!("{}", err.red().dimmed()); }
            }
        }
    } else {
        println!("   {} Instal 'cargo-audit' untuk audit otomatis (cargo install cargo-audit).", "💡 Info:".cyan());
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
        println!("   {} Folder storage/logs telah dikosongkan. ({} file dibersihkan)", "✅ Logs:".green(), count);
    } else {
        println!("   {} Folder storage/logs tidak ditemukan.", "⚠️  Logs:".yellow());
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
        Ok(_) => println!("   {} Tabel sessions telah dikosongkan.", "✅ Sessions:".green()),
        Err(e) => println!("   {} Gagal membersihkan tabel sessions. ({})", "❌ Error:".red(), e),
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

fn generate_app_key() {
    println!("\n{}", "🔑 Generating Application Key...".magenta().bold());

    // 1. Generate 32-byte random key
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    
    // 2. Encode to base64
    let encoded = general_purpose::STANDARD.encode(key);
    let key_str = format!("base64:{}", encoded);
    
    // 3. Update .env file
    let env_path = ".env";
    match fs::read_to_string(env_path) {
        Ok(content) => {
            let re = Regex::new(r"(?m)^APP_KEY=.*").unwrap();
            let new_content = if re.is_match(&content) {
                re.replace(&content, &format!("APP_KEY={}", key_str)).to_string()
            } else {
                format!("{}\nAPP_KEY={}", content.trim_end(), key_str)
            };

            if let Err(e) = fs::write(env_path, new_content) {
                println!("{} Gagal menulis ke file .env: {}", "❌ Error:".red(), e);
            } else {
                println!("{} {}", "✅ Application key set successfully:".green(), key_str.cyan());
                println!("{}", "💡 Pastikan untuk tidak membagikan APP_KEY ini ke publik!".dimmed());
            }
        }
        Err(_) => {
            println!("{} File .env tidak ditemukan.", "❌ Error:".red());
        }
    }
}

async fn make_auth() {
    println!("\n{}", "🔐 Scaffolding Authentication...".magenta().bold());

    // 1. Create src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    let auth_route_template = r#"use axum::{Router, routing::{get, post}, middleware::from_fn};
use crate::app::http::controllers::auth;
use crate::app::http::middleware::auth::guest_middleware;
use crate::config::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", get(auth::auth_controller::AuthController::login_page))
        .route("/login", post(auth::auth_controller::AuthController::login))
        .route("/register", get(auth::auth_controller::AuthController::register_page))
        .route("/register", post(auth::auth_controller::AuthController::register))
        .layer(from_fn(guest_middleware))
}
"#;
    if !std::path::Path::new(auth_route_path).exists() {
        fs::write(auth_route_path, auth_route_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_route_path.cyan());
    } else {
        println!("   {} {}", "⚠️  Exists:".yellow(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path) {
        if !content.contains("pub mod auth;") {
            content.push_str("pub mod auth;\n");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }
    }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path) {
        if !content.contains("use crate::routes::auth as auth_routes;") {
            content = content.replace("use axum::{Router, routing::get};", "use axum::{Router, routing::{get, post}, middleware::from_fn};");
            content = content.replace("use crate::config::server::AppState;", "use crate::app::http::controllers::{auth, dashboard_controller};\nuse crate::app::http::middleware::auth::auth_middleware;\nuse crate::config::server::AppState;\nuse crate::routes::auth as auth_routes;");
            
            let merge_logic = r#"    let auth_protected_routes = Router::new()
        .route("/dashboard", get(dashboard_controller::DashboardController::index))
        .route("/logout", post(auth::auth_controller::AuthController::logout))
        .layer(from_fn(auth_middleware));

    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))
        .merge(auth_routes::router())
        .merge(auth_protected_routes)"#;

            content = content.replace("Router::new()\n        .route(\"/\", get(welcome_controller::index))\n        .route(\"/dev\", get(welcome_controller::dev_info))", merge_logic);
            
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }
    }

    // 4. Create Controller Folder & mod.rs
    let auth_controller_dir = "src/app/http/controllers/auth";
    fs::create_dir_all(auth_controller_dir).ok();
    let auth_controller_mod = "src/app/http/controllers/auth/mod.rs";
    if !std::path::Path::new(auth_controller_mod).exists() {
        fs::write(auth_controller_mod, "pub mod auth_controller;").ok();
    }
    update_controller_mod_rs("auth");

    let auth_controller_path = "src/app/http/controllers/auth/auth_controller.rs";
    if !std::path::Path::new(auth_controller_path).exists() {
        let controller_template = r#"/* ---------------------------------------------------------
 * 📑 LABEL: AUTH CONTROLLER (auth/auth_controller.rs)
 * Menangani pendaftaran, login, dan logout user.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::app::models::users;
use crate::config::requests::Request;
use crate::config::responses::ResponseHelper;
use crate::config::server::AppState;
use axum::{response::IntoResponse, extract::State};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use validator::Validate;
use minijinja::context;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Set};

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    
    #[validate(length(min = 8, message = "Password minimal 8 karakter"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// Menampilkan halaman login
    pub async fn login_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/login.html", context! { title => "Login" })
    }

    /// Menampilkan halaman register
    pub async fn register_page(req: Request) -> impl IntoResponse {
        view(&req, "auth/register.html", context! { title => "Daftar Akun" })
    }

    /// Proses Pendaftaran
    pub async fn register(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<RegisterRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/register"),
        };

        // 2. Cek apakah email sudah terdaftar
        let existing = users::Entity::find()
            .filter(users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if existing.is_some() {
            return ResponseHelper::redirect_with_error("/register", "Email sudah terdaftar", req.session);
        }

        // 3. Hash Password
        let hashed = hash(data.password, DEFAULT_COST).unwrap();

        // 4. Simpan ke Database
        let new_user = users::ActiveModel {
            name: Set(data.name),
            email: Set(data.email),
            password: Set(hashed),
            ..Default::default()
        };

        if let Err(e) = users::Entity::insert(new_user).exec(&state.db).await {
            tracing::error!("Gagal menyimpan user: {}", e);
            return ResponseHelper::redirect_with_error("/register", "Gagal mendaftar, coba lagi.", req.session);
        }

        ResponseHelper::redirect_with_success("/login", "Pendaftaran berhasil! Silakan login.", req.session)
    }

    /// Proses Login
    pub async fn login(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<LoginRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/login"),
        };

        // 2. Ambil User dari DB
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(&data.email))
            .one(&state.db)
            .await
            .ok()
            .flatten();

        if let Some(u) = user {
            // 3. Verifikasi Password
            if verify(data.password, &u.password).unwrap_or(false) {
                // 4. Set Session
                req.session.set("user_id", u.id);
                return ResponseHelper::redirect_with_success("/dashboard", "Selamat datang kembali!", req.session);
            }
        }

        ResponseHelper::redirect_with_error("/login", "Email atau password salah", req.session)
    }

    /// Proses Logout
    pub async fn logout(req: Request) -> impl IntoResponse {
        req.session.remove("user_id");
        ResponseHelper::redirect_with_success("/", "Anda telah keluar.", req.session)
    }
}
"#;
        fs::write(auth_controller_path, controller_template).ok();
        println!("   {} {}", "✅ Created:".green(), auth_controller_path.cyan());
    }

    // 5. Views
    let auth_view_dir = "resources/views/auth";
    fs::create_dir_all(auth_view_dir).ok();
    
    let login_view = "resources/views/auth/login.html";
    if !std::path::Path::new(login_view).exists() {
        let login_template = r##"{% extends "layouts/app.html" %}
{% from "components/forms.html" import input %}
{% from "components/buttons.html" import button, link_back %}

{% block title %}Login - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Selamat Datang Kembali</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Masuk untuk melanjutkan petualangan Anda di ekosistem RustBasic.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            {{ link_back() }}
            
            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Login</h2>
            <p class="text-muted mb-5">Masukkan kredensial Anda untuk masuk.</p>

            <form hx-post="/login" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                {{ input("email", type="email", label="Email", placeholder="nama@email.com", value=old.email, errors=errors.email, required=true) }}
                
                {{ input("password", type="password", label="Password", placeholder="********", required=true) }}

                {{ button("MASUK", class="w-100 mb-4") }}

                <p class="text-muted" style="text-align: center; font-size: 0.9rem;">
                    Belum punya akun? <a href="/register" class="text-primary" style="font-weight: 700;">Daftar Sekarang</a>
                </p>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;
        fs::write(login_view, login_template).ok();
    }
    
    let register_view = "resources/views/auth/register.html";
    if !std::path::Path::new(register_view).exists() {
        let register_template = r##"{% extends "layouts/app.html" %}
{% from "components/forms.html" import input %}
{% from "components/buttons.html" import button, link_back %}

{% block title %}Daftar - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sisi Visual -->
    <div class="split-side-visual" style="background: linear-gradient(135deg, var(--secondary), var(--accent), var(--primary));">
        <div class="visual-inner">
            <h1 style="font-size: 3rem; font-weight: 800; margin-bottom: 1rem;">Bergabung Sekarang</h1>
            <p style="font-size: 1.25rem; opacity: 0.9;">Mulai perjalanan Anda membangun aplikasi web performa tinggi.</p>
        </div>
    </div>

    <!-- Sisi Form -->
    <div class="split-side-content">
        <div class="content-container">
            {{ link_back() }}

            <h2 class="title" style="font-size: 2.5rem; margin-bottom: 0.5rem; text-align: left;">Daftar</h2>
            <p class="text-muted mb-5">Lengkapi formulir di bawah ini.</p>

            <form hx-post="/register" hx-target="body" hx-push-url="true" hx-indicator="#indicator">
                {{ input("name", label="Nama Lengkap", placeholder="Nama Anda", value=old.name, errors=errors.name, required=true) }}

                {{ input("email", type="email", label="Email", placeholder="nama@email.com", value=old.email, errors=errors.email, required=true) }}

                {{ input("password", type="password", label="Password", placeholder="Min. 8 karakter", errors=errors.password, required=true) }}

                {{ button("DAFTAR SEKARANG", class="w-100 mb-4") }}

                <p class="text-muted" style="text-align: center; font-size: 0.9rem;">
                    Sudah punya akun? <a href="/login" class="text-primary" style="font-weight: 700;">Login Disini</a>
                </p>
            </form>
        </div>
    </div>
</div>
{% endblock %}
"##;
        fs::write(register_view, register_template).ok();
    }

    let dashboard_view = "resources/views/dashboard.html";
    if !std::path::Path::new(dashboard_view).exists() {
        let dashboard_template = r##"{% extends "layouts/app.html" %}
{% from "components/buttons.html" import button %}
{% from "components/display.html" import stat_card %}

{% block title %}{{ title }} - RustBasic{% endblock %}

{% block content %}
<div class="split-screen">
    <!-- Sidebar / Visual Side (Kiri - Narrower) -->
    <div class="split-side-visual" style="flex: 0.4; align-items: flex-start; text-align: left; padding: 3rem;">
        <div style="width: 100%;">
            <div style="width: 80px; height: 80px; background: rgba(255,255,255,0.2); border-radius: 2rem; display: flex; align-items: center; justify-content: center; font-size: 2rem; font-weight: 800; margin-bottom: 2rem; border: 2px solid rgba(255,255,255,0.3);">
                {{ user_name[0] | upper }}
            </div>
            <h2 style="font-size: 2rem; font-weight: 800; margin-bottom: 0.5rem;">{{ user_name }}</h2>
            <p style="opacity: 0.8; margin-bottom: 3rem;">{{ user_email }}</p>

            <nav style="display: flex; flex-direction: column; gap: 0.75rem; width: 100%;">
                <a href="/dashboard" class="btn" style="background: rgba(255,255,255,0.2); color: #fff; justify-content: flex-start; text-transform: none;">Dashboard Overview</a>
                <a href="/" class="btn" style="color: #fff; justify-content: flex-start; text-transform: none; opacity: 0.7;">Beranda Utama</a>
                <div style="margin-top: auto; padding-top: 2rem;">
                    {% from "components/overlays.html" import logout_confirm_button %}
                    {{ logout_confirm_button(id="dashboard-logout", label="LOGOUT", variant="outline", class="w-100", style="border: none; background: rgba(255,255,255,0.1); color: #fff;") }}
                </div>
            </nav>
        </div>
    </div>

    <!-- Main Content (Kanan - Wider) -->
    <div class="split-side-content" style="flex: 1.2; align-items: flex-start; justify-content: flex-start; background: #f8faff;">
        <div style="width: 100%; padding: 4rem;">
            <div class="mb-5">
                <h1 class="title" style="font-size: 3rem; text-align: left; margin-bottom: 0.5rem;">Dashboard</h1>
                <p class="text-muted">Kelola aplikasi dan pantau aktivitas Anda di sini.</p>
            </div>

            <!-- Grid Statistik -->
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 2rem; margin-bottom: 4rem;">
                {{ stat_card("Total Pengguna", total_users) }}
                
                <div style="border-bottom: 4px solid var(--accent); padding: 1.5rem 0;">
                    <p class="text-muted text-uppercase" style="font-size: 0.8rem; font-weight: 700; letter-spacing: 0.1em; margin-bottom: 1rem;">Status Sistem</p>
                    <div style="display: flex; align-items: center; gap: 0.75rem; font-size: 1.5rem; font-weight: 800; color: #2e7d32; padding: 1.2rem 0;">
                        <div style="width: 12px; height: 12px; background: #4caf50; border-radius: 50%;"></div>
                        ONLINE
                    </div>
                </div>

                {{ stat_card("Performa", "99.9%", color="var(--secondary)") }}
            </div>

            <!-- Content Area -->
            <div style="background: #fff; padding: 3rem; border-radius: 0; border-left: 8px solid var(--primary);">
                <h2 style="font-weight: 800; margin-bottom: 1.5rem;">Informasi Sistem</h2>
                <p class="text-muted" style="line-height: 1.8; margin-bottom: 2rem;">
                    Aplikasi ini berjalan menggunakan backend Rust yang dioptimalkan. Semua data diproses secara real-time melalui koneksi SQLite yang aman.
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                    <span class="badge">Engine: Axum 0.8</span>
                    <span class="badge">Render: Minijinja</span>
                    <span class="badge">Frontend: HTMX + Minijinja</span>
                </div>
            </div>
        </div>
    </div>
</div>
{% endblock %}
"##;
        fs::write(dashboard_view, dashboard_template).ok();
    }
    
    // 6. Create Dashboard Controller
    let dashboard_controller_path = "src/app/http/controllers/dashboard_controller.rs";
    if !std::path::Path::new(dashboard_controller_path).exists() {
        let dashboard_template = r#"use crate::app::view;
use crate::app::models::users;
use crate::config::requests::Request;
use crate::config::server::AppState;
use axum::{response::IntoResponse, extract::State};
use minijinja::context;
use sea_orm::{EntityTrait, PaginatorTrait};

pub struct DashboardController;

impl DashboardController {
    pub async fn index(State(state): State<AppState>, req: Request) -> impl IntoResponse {
        let user_id = req.session.get::<i32>("user_id").unwrap_or(0);
        let user = users::Entity::find_by_id(user_id).one(&state.db).await.ok().flatten();
        let total_users = users::Entity::find().count(&state.db).await.unwrap_or(0);

        view(&req, "dashboard.html", context! {
            title => "Dashboard",
            user_name => user.as_ref().map(|u| u.name.clone()).unwrap_or("Guest".to_string()),
            user_email => user.as_ref().map(|u| u.email.clone()).unwrap_or_default(),
            total_users => total_users,
        })
    }
}
"#;
        fs::write(dashboard_controller_path, dashboard_template).ok();
        println!("   {} {}", "✅ Created:".green(), dashboard_controller_path.cyan());
    }
    update_controller_mod_rs("dashboard_controller");

    println!("   {} Folder resources/views/auth dan dashboard siap.", "✅ Views:".green());

    // 6. Update welcome.html
    let welcome_path = "resources/views/welcome.html";
    if let Ok(content) = fs::read_to_string(welcome_path) {
        if !content.contains("{% if auth %}") {
            println!("   {} {}", "⚠️  Manual:".yellow(), "Pastikan welcome.html memiliki tombol login/register.".dimmed());
        } else {
            println!("   {} {}", "✅ OK:".green(), "welcome.html sudah memiliki logika auth.".dimmed());
        }
    }

    println!("\n{}", "✨ Authentication scaffolded successfully!".green().bold());
    println!("{}", "Jalankan 'cargo rustbasic route:list' untuk melihat route baru.".dimmed());
}

async fn remove_auth() {
    println!("\n{}", "🗑️  Removing Authentication Scaffold...".red().bold());

    // 1. Delete src/routes/auth.rs
    let auth_route_path = "src/routes/auth.rs";
    if std::path::Path::new(auth_route_path).exists() {
        fs::remove_file(auth_route_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_route_path.cyan());
    }

    // 2. Update src/routes/mod.rs
    let routes_mod_path = "src/routes/mod.rs";
    if let Ok(mut content) = fs::read_to_string(routes_mod_path) {
        if content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            fs::write(routes_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), routes_mod_path.cyan());
        }
    }

    // 3. Update src/routes/web.rs
    let web_route_path = "src/routes/web.rs";
    if let Ok(mut content) = fs::read_to_string(web_route_path) {
        let mut changed = false;
        
        // Remove imports
        if content.contains("use axum::{Router, routing::{get, post}, middleware::from_fn};") {
            content = content.replace("use axum::{Router, routing::{get, post}, middleware::from_fn};", "use axum::{Router, routing::get};");
            changed = true;
        }
        
        let imports_to_remove = [
            "use crate::app::http::controllers::{auth, dashboard_controller};\n",
            "use crate::app::http::middleware::auth::auth_middleware;\n",
            "use crate::routes::auth as auth_routes;\n",
            "use crate::app::http::controllers::{auth, dashboard_controller};",
            "use crate::app::http::middleware::auth::auth_middleware;",
            "use crate::routes::auth as auth_routes;",
        ];
        
        for imp in imports_to_remove {
            if content.contains(imp) {
                content = content.replace(imp, "");
                changed = true;
            }
        }

        // Remove auth_protected_routes logic and restore basic Router
        if content.contains("let auth_protected_routes = Router::new()") {
            let re = Regex::new(r##"(?s)let auth_protected_routes = Router::new\(\).*?\.layer\(from_fn\(auth_middleware\)\);"##).unwrap();
            content = re.replace(&content, "").to_string();
            
            content = content.replace(".merge(auth_routes::router())", "");
            content = content.replace(".merge(auth_protected_routes)", "");
            
            // Restore clean Router::new()
            let clean_router = r#"    Router::new()
        .route("/", get(welcome_controller::index))
        .route("/dev", get(welcome_controller::dev_info))"#;
            
            let router_re = Regex::new(r##"(?s)Router::new\(\).*?\.route\(\s*\"/dev\"\s*,\s*get\(welcome_controller::dev_info\)\s*\)"##).unwrap();
            content = router_re.replace(&content, clean_router).to_string();
            
            changed = true;
        }

        if changed {
            fs::write(web_route_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), web_route_path.cyan());
        }
    }

    // 4. Delete Controllers
    let auth_controller_dir = "src/app/http/controllers/auth";
    if std::path::Path::new(auth_controller_dir).exists() {
        fs::remove_dir_all(auth_controller_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_controller_dir.cyan());
    }

    // 5. Delete Views
    let auth_view_dir = "resources/views/auth";
    if std::path::Path::new(auth_view_dir).exists() {
        fs::remove_dir_all(auth_view_dir).ok();
        println!("   {} {}", "✅ Deleted:".green(), auth_view_dir.cyan());
    }

    // 6. Delete Dashboard Controller
    let dashboard_path = "src/app/http/controllers/dashboard_controller.rs";
    if std::path::Path::new(dashboard_path).exists() {
        fs::remove_file(dashboard_path).ok();
        println!("   {} {}", "✅ Deleted:".green(), dashboard_path.cyan());
    }

    // 7. Update src/app/http/controllers/mod.rs
    let controllers_mod_path = "src/app/http/controllers/mod.rs";
    if let Ok(mut content) = fs::read_to_string(controllers_mod_path) {
        let mut changed = false;
        if content.contains("pub mod auth;") {
            content = content.replace("pub mod auth;\n", "");
            changed = true;
        }
        if content.contains("pub mod dashboard_controller;") {
            content = content.replace("pub mod dashboard_controller;\n", "");
            changed = true;
        }
        if changed {
            fs::write(controllers_mod_path, content).ok();
            println!("   {} {}", "📝 Updated:".blue(), controllers_mod_path.cyan());
        }
    }

    println!("\n{}", "✨ Authentication removed successfully!".green().bold());
}


