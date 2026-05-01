/* ---------------------------------------------------------
 * 📑 LABEL: AUTH CONTROLLER (auth/auth_controller.rs)
 * Menangani pendaftaran, login, dan logout user.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::app::http::requests::Request;
use crate::app::http::responses::ResponseHelper;
use axum::response::IntoResponse;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::Deserialize;
use validator::Validate;
use minijinja::context;
use sqlx::sqlite::SqlitePool;
use crate::config::Config;

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
    pub async fn register(req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<RegisterRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/register"),
        };

        // 2. Cek apakah email sudah terdaftar
        let cfg = Config::load();
        let db_url = format!("sqlite://database/{}.sqlite", cfg.db_database);
        let pool = SqlitePool::connect(&db_url).await.unwrap();

        let exists: Option<i64> = sqlx::query_scalar("SELECT id FROM users WHERE email = ?")
            .bind(&data.email)
            .fetch_optional(&pool)
            .await
            .unwrap();

        if exists.is_some() {
            return ResponseHelper::redirect_with_error("/register", "Email sudah terdaftar", req.session);
        }

        // 3. Hash Password
        let hashed = hash(data.password, DEFAULT_COST).unwrap();

        // 4. Simpan ke Database
        sqlx::query("INSERT INTO users (name, email, password) VALUES (?, ?, ?)")
            .bind(&data.name)
            .bind(&data.email)
            .bind(&hashed)
            .execute(&pool)
            .await
            .unwrap();

        ResponseHelper::redirect_with_success("/login", "Pendaftaran berhasil! Silakan login.", req.session)
    }

    /// Proses Login
    pub async fn login(req: Request) -> impl IntoResponse {
        // 1. Validasi Input
        let data = match req.validate::<LoginRequest>() {
            Ok(d) => d,
            Err(_) => return ResponseHelper::redirect("/login"),
        };

        // 2. Ambil User dari DB
        let cfg = Config::load();
        let db_url = format!("sqlite://database/{}.sqlite", cfg.db_database);
        let pool = SqlitePool::connect(&db_url).await.unwrap();

        let user: Option<(i64, String)> = sqlx::query_as("SELECT id, password FROM users WHERE email = ?")
            .bind(&data.email)
            .fetch_optional(&pool)
            .await
            .unwrap();

        if let Some((id, hashed)) = user {
            // 3. Verifikasi Password
            if verify(data.password, &hashed).unwrap() {
                // 4. Set Session
                req.session.set("user_id", id);
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
