-- 📑 LABEL: LARAVEL SESSION SCHEMA
-- Tabel ini mengikuti struktur standar Laravel untuk session.

CREATE TABLE IF NOT EXISTS sessions (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NULL,
    ip_address VARCHAR(45) NULL,
    user_agent TEXT NULL,
    payload TEXT NOT NULL,
    last_activity INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS sessions_last_activity_index ON sessions(last_activity);
