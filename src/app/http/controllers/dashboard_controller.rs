/* ---------------------------------------------------------
 * 📑 LABEL: DASHBOARD CONTROLLER (dashboard_controller.rs)
 * Menampilkan halaman dashboard untuk user yang sudah login.
 * --------------------------------------------------------- */

use crate::app::view;
use crate::app::http::requests::Request;
use axum::response::IntoResponse;
use minijinja::context;
use sqlx::sqlite::SqlitePool;
use crate::config::Config;

pub struct DashboardController;

impl DashboardController {
    pub async fn index(req: Request) -> impl IntoResponse {
        // 1. Ambil user_id dari session
        let user_id = req.session.get::<i64>("user_id").unwrap();

        // 2. Koneksi ke Database
        let cfg = Config::load();
        let db_url = format!("sqlite://database/{}.sqlite", cfg.db_database);
        let pool = SqlitePool::connect(&db_url).await.unwrap();

        // 3. Ambil data user
        let user: (String, String) = sqlx::query_as("SELECT name, email FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|_| ("User".to_string(), "".to_string()));

        // 4. Ambil statistik sederhana (total user)
        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        // 5. Render tampilan dashboard
        view(&req, "dashboard.html", context! {
            title => "Dashboard Utama",
            user_name => user.0,
            user_email => user.1,
            total_users => total_users
        })
    }
}
