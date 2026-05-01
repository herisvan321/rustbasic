/* ---------------------------------------------------------
 * 📑 LABEL: CUSTOM SESSION MANAGER (database/session_manager.rs)
 * File ini mengimplementasikan penyimpanan session ala Laravel.
 * --------------------------------------------------------- */

use axum_session::{DatabasePool, DatabaseError};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

#[derive(Clone, Debug)]
pub struct LaravelSessionStore {
    pub pool: Pool<Sqlite>,
}

impl LaravelSessionStore {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DatabasePool for LaravelSessionStore {
    // 1. Inisialisasi tabel
    async fn initiate(&self, _table_name: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    // 2. Hapus Satu Session
    async fn delete_one_by_id(&self, id: &str, table_name: &str) -> Result<(), DatabaseError> {
        let query = format!("DELETE FROM {} WHERE id = ?", table_name);
        sqlx::query(&query)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    // 3. Muat Session
    async fn load(&self, id: &str, table_name: &str) -> Result<Option<String>, DatabaseError> {
        let query = format!("SELECT payload FROM {} WHERE id = ? AND last_activity > ?", table_name);
        let now = chrono::Utc::now().timestamp();
        
        let row: Option<(String,)> = sqlx::query_as(&query)
            .bind(id)
            .bind(now)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;

        Ok(row.map(|r| r.0))
    }

    // 4. Simpan Session
    async fn store(&self, id: &str, session: &str, expires: i64, table_name: &str) -> Result<(), DatabaseError> {
        let query = format!(
            "INSERT INTO {} (id, payload, last_activity) VALUES (?, ?, ?) 
             ON CONFLICT(id) DO UPDATE SET payload = excluded.payload, last_activity = excluded.last_activity",
            table_name
        );

        sqlx::query(&query)
            .bind(id)
            .bind(session)
            .bind(expires)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericInsertError(e.to_string()))?;
        Ok(())
    }

    // 5. Bersihkan Session Kadaluarsa
    async fn delete_by_expiry(&self, table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let now = chrono::Utc::now().timestamp();
        
        // Ambil ID yang akan dihapus (opsional, tapi trait minta Vec<String>)
        let select_query = format!("SELECT id FROM {} WHERE last_activity < ?", table_name);
        let ids: Vec<String> = sqlx::query_as::<_, (String,)>(&select_query)
            .bind(now)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?
            .into_iter()
            .map(|r| r.0)
            .collect();

        let delete_query = format!("DELETE FROM {} WHERE last_activity < ?", table_name);
        sqlx::query(&delete_query)
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;

        Ok(ids)
    }

    // 6. Hitung jumlah session
    async fn count(&self, table_name: &str) -> Result<i64, DatabaseError> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        let count: (i64,) = sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        Ok(count.0)
    }

    // 7. Cek keberadaan session
    async fn exists(&self, id: &str, table_name: &str) -> Result<bool, DatabaseError> {
        let query = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE id = ?)", table_name);
        let exists: (bool,) = sqlx::query_as(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?;
        Ok(exists.0)
    }

    // 8. Hapus semua session
    async fn delete_all(&self, table_name: &str) -> Result<(), DatabaseError> {
        let query = format!("DELETE FROM {}", table_name);
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericDeleteError(e.to_string()))?;
        Ok(())
    }

    // 9. Ambil semua ID
    async fn get_ids(&self, table_name: &str) -> Result<Vec<String>, DatabaseError> {
        let query = format!("SELECT id FROM {}", table_name);
        let ids: Vec<String> = sqlx::query_as::<_, (String,)>(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DatabaseError::GenericSelectError(e.to_string()))?
            .into_iter()
            .map(|r| r.0)
            .collect();
        Ok(ids)
    }

    // 10. Apakah database menangani expiry secara otomatis?
    fn auto_handles_expiry(&self) -> bool {
        false
    }
}
