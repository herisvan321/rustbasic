# Database: Eloquent (Sea-ORM Entity)

RustBasic menggunakan **Sea-ORM** sebagai ORM utama, yang memberikan pengalaman yang sangat intuitif dan kuat.

## Mendefinisikan Model
Model disimpan di `src/app/models/`. Buat otomatis via CLI:
```bash
rustbasic make:model User
```

## Operasi Dasar (CRUD)
```rust
use crate::app::models::users;
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

// Ambil semua
let users = users::Entity::find().all(&db).await?;

// Cari berdasarkan ID
let user = users::Entity::find_by_id(1).one(&db).await?;

// Simpan data (Insert)
let new_user = users::ActiveModel {
    name: Set("example".to_owned()),
    email: Set("example@example.com".to_owned()),
    ..Default::default()
};
new_user.insert(&db).await?;
```

---

# Database: Migrations

Migrasi menggunakan **Sea-ORM Migration** (berbasis Rust) dan disimpan di `database/migrations/`.

## Membuat Migrasi
```bash
rustbasic make:model Name -m
# atau
rustbasic make:migration Name
```

## Menjalankan Migrasi
```bash
rustbasic migrate
```

## Refresh Migrasi
Gunakan perintah ini untuk membatalkan semua migrasi dan menjalankannya kembali (meriset database):
```bash
rustbasic migrate:refresh
```

## Rollback Migrasi
Gunakan perintah ini untuk membatalkan satu langkah migrasi terakhir:
```bash
rustbasic migrate:back
# atau
rustbasic migrate:rollback
```

## Contoh File Migrasi (Rust)
```rust
async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.create_table(
        Table::create()
            .table(Users::Table)
            .col(ColumnDef::new(Users::Id).integer().not_null().auto_increment().primary_key())
            .col(ColumnDef::new(Users::Name).string().not_null())
            .to_owned()
    ).await
}
```

---

# Database: Query Builder

Selain Entity, Anda dapat menggunakan Query Builder Sea-ORM untuk query yang lebih kompleks.

```rust
use sea_orm::{QuerySelect, Condition};

let results = users::Entity::find()
    .select_only()
    .column(users::Column::Name)
    .filter(users::Column::Email.contains("gmail"))
    .into_json()
    .all(&db)
    .await?;
```

---

# Database: Seeders

Seeder digunakan untuk mengisi database dengan data awal (seperti admin user, data kategori, dll) secara otomatis.

## Membuat Seeder
Gunakan CLI untuk membuat file seeder baru:
```bash
rustbasic make:seeder Name
```
File akan dibuat di `database/seeders/`. Secara default, RustBasic menyertakan **`DatabaseSeeder`** sebagai contoh utama.

## Menjalankan Seeder
Jalankan perintah berikut untuk mengeksekusi seluruh seeder yang terdaftar:
```bash
rustbasic db:seed
```

## Cara Registrasi Seeder
Setelah membuat file seeder, Anda harus mendaftarkannya di **`src/app/seeder.rs`** agar bisa dijalankan oleh framework. CLI `make:seeder` akan mencoba mendaftarkannya secara otomatis.

## Contoh Struktur Seeder
Setiap seeder harus mengimplementasikan `SeederTrait`:
```rust
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait};
use rustbasic_core::seeder::SeederTrait;

pub struct DatabaseSeeder;

#[async_trait::async_trait]
impl SeederTrait for DatabaseSeeder {
    async fn run(&self, db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
        // Logika pengisian data Anda di sini
        Ok(())
    }
}
```
