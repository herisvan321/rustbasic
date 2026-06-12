# 🗄️ Panduan Database, Migrasi, & Seeder

## 📝 Kata Pengantar

Selamat datang di panduan **Manajemen Database RustBasic**. Dokumentasi ini dirancang khusus untuk memandu Anda mengelola interaksi database relasional, skema tabel, dan pengisian data dummy secara aman, andal, dan modular. Anda akan mempelajari cara menggunakan macro `model!` untuk memetakan objek data, menulis file migrasi skema blueprint, hingga memicu eksekusi pengisian data seeder.

---

## ⚙️ Pemilihan Driver Database (Cargo Features)

RustBasic menggunakan sistem **Cargo features** untuk memilih driver database secara eksplisit. Ini memastikan Anda hanya mengkompilasi driver yang benar-benar dibutuhkan, sehingga waktu kompilasi tetap singkat.

### Tabel Driver yang Tersedia

| Feature | Aktif Default | Deskripsi |
| :--- | :---: | :--- |
| `sqlite` | ✅ Ya | Driver SQLite — cocok untuk development & aplikasi file-based. Tidak perlu server database. |
| `sqlite-bundled` | ❌ Tidak | SQLite bundled tanpa ketergantungan `libsqlite3` di sistem operasi host. |
| `mysql` | ❌ Tidak | Driver MySQL/MariaDB dengan koneksi asinkron + TLS. Aktifkan jika `DB_CONNECTION=mysql`. |

### Cara Mengaktifkan Driver

Edit file `Cargo.toml` di root project Anda:

```toml
[dependencies]

# Pilihan 1: SQLite saja (default — tidak perlu tambahan)
rustbasic-core = { path = "../rustbasic-core" }

# Pilihan 2: MySQL / MariaDB
rustbasic-core = { path = "../rustbasic-core", features = ["mysql"] }

# Pilihan 3: SQLite bundled (tanpa libsqlite3 di sistem)
rustbasic-core = { path = "../rustbasic-core", features = ["sqlite-bundled"] }
```

> ⚠️ **Penting:** Jika file `.env` berisi `DB_CONNECTION=mysql`, fitur `mysql` **wajib diaktifkan**. Jika tidak, server akan panic saat startup dengan pesan:
> ```
> DB_CONNECTION=mysql terdeteksi, tapi fitur mysql belum aktif.
> Tambahkan features = ["mysql"] pada rustbasic-core di Cargo.toml project Anda
> ```

### Konfigurasi `.env` per Driver

**SQLite:**
```env
DB_CONNECTION=sqlite
DB_DATABASE=rustbasic
```

**MySQL / MariaDB:**
```env
DB_CONNECTION=mysql
DB_HOST=127.0.0.1
DB_PORT=3306
DB_DATABASE=nama_database
DB_USERNAME=root
DB_PASSWORD=rahasia
```

---



## 🛠️ Script Contoh

### A. Definisi Model dengan Macro (`src/app/models/product.rs`)

```rust
use rustbasic_core::model;
use rustbasic_core::chrono::NaiveDateTime;

model! {
    table: "products",
    fillable: [name, price],
    Model {
        pub id: i32,
        pub name: String,
        pub price: f64,
        pub created_at: NaiveDateTime,
        pub updated_at: NaiveDateTime,
    }
}
```

### B. Membuat Data Baru (Insert) di Controller

```rust
use crate::app::models::Product;
use rustbasic_core::serde_json::json;

pub async fn store_product(db: &sqlx::AnyPool) {
    let _ = Product::create(db, json!({
        "name": "Keyboard Mechanical Premium",
        "price": 1250000.0
    })).await;
}
```

### C. File Migrasi Skema Blueprint (`database/migrations/m2026_create_products.rs`)

```rust
use rustbasic_core::{Schema, SchemaManager, MigrationTrait, DbErr};
use rustbasic_core::async_trait;

pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    fn name(&self) -> &str { "m2026_create_products" }

    async fn up<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::create(manager, "products", |table| {
            table.id();
            table.string("name").not_null();
            table.double("price").not_null();
            table.timestamps();
        }).await
    }

    async fn down<'a>(&self, manager: &'a SchemaManager<'a>) -> Result<(), DbErr> {
        Schema::drop(manager, "products").await
    }
}
```

> [!NOTE]
> **Kompatibilitas Multi-Engine (SQLite & MySQL)**
> Blueprint builder secara otomatis menerjemahkan tipe kolom pada runtime:
> * Kolom `text()` diterjemahkan menjadi `TEXT` di SQLite dan `VARCHAR(4000)` di MySQL.
> * Kolom `long_text()` diterjemahkan menjadi `TEXT` di SQLite dan `VARCHAR(16383)` di MySQL.
> Penyesuaian ini menjamin kestabilan driver SQLx Any untuk mencegah pembacaan data teks sebagai tipe binary/BLOB yang memicu error type mismatch.

---

## 🚀 Active Record ORM & Query Builder

RustBasic menyediakan ORM (Active Record) dan Query Builder bawaan yang kuat dan berkinerja tinggi. Berikut adalah panduan dan contoh penggunaannya.

### A. Static ORM Methods (Active Record)

Jika Anda mendefinisikan model menggunakan macro `model!`, Model Anda secara otomatis mendapatkan static methods berikut:

#### 1. `Model::all(db) -> Result<Vec<Self>, sqlx::Error>`
Mengambil seluruh data dari tabel model:
```rust
let users = users::Model::all(&db).await.unwrap_or_default();
```

#### 2. `Model::first(db) -> Result<Option<Self>, sqlx::Error>`
Mengambil data pertama di dalam tabel:
```rust
if let Some(user) = users::Model::first(&db).await.unwrap() {
    println!("User pertama: {}", user.name);
}
```

#### 3. `Model::find(db, id) -> Result<Option<Self>, sqlx::Error>`
Mencari satu record berdasarkan numeric ID:
```rust
if let Some(user) = users::Model::find(&db, 1).await.unwrap() {
    println!("User ditemukan: {}", user.name);
}
```

#### 4. `Model::count(db) -> Result<i64, sqlx::Error>`
Mendapatkan total jumlah record di dalam tabel:
```rust
let total = users::Model::count(&db).await.unwrap_or(0);
```

#### 5. `Model::create(db, json_data) -> Result<Self, sqlx::Error>`
Menyimpan record baru dan mengembalikan instance model yang terisi data:
```rust
let new_user = users::Model::create(&db, json!({
    "name": "Budi",
    "email": "budi@example.com",
    "password": "hashed_password"
})).await.unwrap();
```

#### 6. `Model::destroy(db, id) -> Result<u64, sqlx::Error>`
Menghapus record berdasarkan ID dan mengembalikan jumlah baris terhapus:
```rust
let rows_deleted = users::Model::destroy(&db, 1).await.unwrap();
```

#### 7. `Model::query(db) -> QueryBuilder`
Memulai fluent Query Builder untuk melakukan kueri yang lebih kompleks secara langsung dari Model:
```rust
let users = users::Model::query(&db)
    .where_("status", "active")
    .order_by("created_at", "DESC")
    .limit(10)
    .get::<users::Model>()
    .await
    .unwrap();
```

---

### B. Fluent Query Builder (`DB::table`)

Untuk kueri dinamis atau tabel yang tidak didefinisikan sebagai model kustom, Anda dapat menggunakan pembangun kueri `DB::table`:

```rust
use rustbasic_core::database::DB;

// Mengambil banyak record
let users = DB::table(&db, "users")
    .where_op("id", ">", 1)
    .get::<users::Model>()
    .await
    .unwrap();

// Update Data
DB::table(&db, "users")
    .where_("id", 1)
    .update(json!({ "name": "Budi Baru" }))
    .await
    .unwrap();
```

---

### C. Advanced ORM Features (Relationships, Accessors, Mutators, & Resources)

Anda dapat mengimplementasikan fitur asinkron tingkat lanjut mirip ORM modern langsung pada struct Model Anda.

#### 1. Relationships (Hubungan Antar Tabel)
Mendefinisikan method relasi di model menggunakan Query Builder:
```rust
// users.rs
impl Model {
    pub async fn password_resets(&self, db: &sqlx::AnyPool) -> Result<Vec<password_resets::Model>, sqlx::Error> {
        password_resets::Model::query(db)
            .where_("email", &self.email)
            .get::<password_resets::Model>()
            .await
    }
}
```

#### 2. Accessors (Akses Format Data)
Membuat method getter untuk memformat data:
```rust
impl Model {
    pub fn name_and_email(&self) -> String {
        format!("{} ({})", self.name, self.email)
    }
}
```

#### 3. Mutators (Format Sebelum Simpan)
Membuat method untuk memodifikasi attribute model sebelum disimpan:
```rust
impl Model {
    pub fn set_password(&mut self, plain: &str) {
        self.password = rustbasic_core::bcrypt::hash(plain, 10).unwrap();
    }
}
```

#### 4. API Resources (Transformasi Respon JSON)
Memformat respon JSON publik untuk menyembunyikan field sensitif:
```rust
impl Model {
    pub fn to_resource(&self) -> serde_json::Value {
        json!({
            "id": self.id,
            "name": self.name,
            "email": self.email,
        })
    }
}
```

#### 5. Streaming Data & Fail-Safe Queries (Penyetaraan Eloquent Lanjutan)

Berikut adalah pemetaan implementasi untuk beberapa pola kueri lanjutan:

##### A. Chunking & Streaming (`chunk`, `lazy`, `cursor`)
Di Rust, pemrosesan data besar baris demi baris menggunakan **SQLx Stream** asinkron agar hemat penggunaan RAM:
```rust
use futures::StreamExt;

let mut rows = sqlx::query_as::<_, users::Model>("SELECT * FROM users")
    .fetch(&db);

while let Some(result) = rows.next().await {
    if let Ok(user) = result {
        println!("Memproses user: {}", user.name);
    }
}
```

##### B. Penemuan Data atau Gagal (`findOrFail`, `firstOrFail`)
Rust menggunakan pengembalian `Option` dan operator `?` untuk penanganan error/fail secara eksplisit:
```rust
// findOrFail: Mengembalikan error jika data tidak ditemukan (None)
let user = users::Model::find(&db, 1).await?
    .ok_or("User tidak ditemukan (404)")?;
```

##### C. Melacak Perubahan State (`isDirty`, `isClean`, `getChanges`)
Untuk melacak perubahan properti sebelum disimpan, lakukan penggandaan data (`.clone()`) dan bandingkan dengan data baru:
```rust
let mut user = users::Model::find(&db, 1).await?.unwrap();
let original = user.clone(); // Gandakan data awal

user.name = "Jack".to_string(); // Mutasi data

// isDirty()
let is_dirty = user != original; // true
let is_name_dirty = user.name != original.name; // true

// getPrevious()
let old_name = original.name; // "John"
```

##### D. Mass Assignment (`fillable`, `guarded`, `unguard`)

Di RustBasic, pembatasan atribut saat pengisian massal (*mass assignment*) dikonfigurasi langsung di dalam macro `model!`.

1. **Whitelisting (`fillable`)**
   Anda mendefinisikan field mana saja yang boleh diisi secara massal:
   ```rust
   model! {
       table: "flights",
       fillable: [name, options],
       Model {
           pub id: i32,
           pub name: String,
           pub options: Option<String>,
       }
   }
   ```

2. **JSON Nested Mass Assignment (`options->enabled`)**
   Sistem mengizinkan dot/arrow notation untuk melakukan manipulasi field JSON. Di RustBasic, ini dikelola secara asinkron menggunakan strongly typed structs lewat bantuan Serde. Keuntungannya adalah validasi struktur langsung terjadi di level kompilasi (*compile-time safety*) sehingga tidak rentan terhadap typo:
   ```rust
   #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
   pub struct FlightOptions {
       pub enabled: bool,
   }

   impl Model {
       // Accessor untuk deserialize data JSON text ke strongly-typed struct
       pub fn get_options(&self) -> Option<FlightOptions> {
           self.options.as_ref().and_then(|opt| serde_json::from_str(opt).ok())
       }

       // Mutator untuk serialize strongly-typed struct kembali ke string JSON
       pub fn set_options(&mut self, options: &FlightOptions) {
           self.options = Some(serde_json::to_string(options).unwrap());
       }
   }
   ```

3. **Unguarded Models (`#[Unguarded]` atau `unguard()`)**
   Apabila Anda tidak ingin menyaring kolom apapun saat eksekusi `create` (seluruh field bebas dimasukkan), Anda cukup menghilangkan deklarasi `fillable` dan `guarded` di dalam deklarasi macro `model!`:
   ```rust
   model! {
       table: "flights_unguarded",
       Model {
           pub id: i32,
           pub name: String,
           pub is_active: bool,
       }
   }
   ```

##### E. Soft Deletes (`SoftDeletes`)

Dalam framework, Anda dapat menggunakan fitur hapus lunak (*soft deletes*) untuk menyembunyikan record dari hasil query alih-alih menghapusnya secara permanen dari database. Di RustBasic, fitur ini diaktifkan dengan menambahkan konfigurasi `soft_deletes: true` di dalam macro `model!`.

1. **Definisi Model**
   Pastikan tabel Anda memiliki kolom `deleted_at` (tipe data `Option<NaiveDateTime>`) dan tambahkan opsi `soft_deletes: true`:
   ```rust
   model! {
       table: "flights",
       soft_deletes: true,
       Model {
           pub id: i32,
           pub name: String,
           pub deleted_at: Option<NaiveDateTime>,
       }
   }
   ```

2. **Operasi Soft Delete**
   Memanggil `Model::destroy` pada model yang menggunakan soft delete akan memperbarui kolom `deleted_at` ke waktu saat ini alih-alih menjalankan perintah SQL `DELETE`:
   ```rust
   // Melakukan soft delete
   flights::Model::destroy(&db, 1).await?;
   ```

3. **Query Mengabaikan Data Terhapus**
   Secara default, query standar seperti `all()`, `find()`, `first()`, dan `count()` akan otomatis memfilter data yang telah di-soft-delete (`deleted_at IS NULL`):
   ```rust
   // Hanya mengembalikan flight aktif (deleted_at IS NULL)
   let active_flights = flights::Model::all(&db).await?;
   ```

4. **Kueri Termasuk Data Terhapus (`query_with_trashed`)**
    Jika Anda ingin menyertakan data yang telah di-soft-delete dalam pencarian:
   ```rust
   let all_flights = flights::Model::query_with_trashed(&db)
       .get::<flights::Model>()
       .await?;
   ```

5. **Kueri Hanya Data Terhapus (`query_only_trashed`)**
    Jika Anda hanya ingin mengambil data yang telah di-soft-delete:
   ```rust
   let deleted_flights = flights::Model::query_only_trashed(&db)
       .get::<flights::Model>()
       .await?;
   ```

6. **Mengembalikan Data Terhapus (`restore`)**
    Untuk membatalkan soft delete dan mengembalikan record menjadi aktif kembali:
   ```rust
   flights::Model::restore(&db, 1).await?;
   ```

7. **Penghapusan Permanen (`force_destroy`)**
    Untuk menghapus record secara permanen dari database:
   ```rust
   flights::Model::force_destroy(&db, 1).await?;
   ```

##### F. Query Scopes (Local & Global Scopes)

Query Scopes memungkinkan Anda untuk mendefinisikan batasan kueri yang dapat digunakan kembali (*reusable*) untuk model tertentu. RustBasic mendukung Local Scopes (dipicu secara eksplisit) dan Global Scopes (diterapkan secara otomatis pada semua kueri).

1. **Local Scopes (Batasan Kueri Reusable)**
   Untuk mendefinisikan local scopes, tambahkan blok `scopes` di dalam macro `model!`. Blok ini akan menghasilkan sebuah extension trait bernama `ModelScopes` untuk `QueryBuilder` Anda serta fungsi statis pada model:
   ```rust
   model! {
       table: "flights",
       scopes: {
           // Scope tanpa argumen tambahan
           active() => |q| q.where_("active", 1),
           // Scope dengan argumen tambahan
           destination(dest: &str) => |q, dest| q.where_("destination", dest),
       },
       Model {
           pub id: i32,
           pub name: String,
           pub active: i32,
           pub destination: Option<String>,
       }
   }
   ```

2. **Memanggil Local Scopes secara Chainable**
   Untuk menggunakan local scope secara berantai pada `QueryBuilder`, import trait `ModelScopes` dari modul model Anda:
   ```rust
   // Import trait scope untuk mengaktifkan chainable methods
   use crate::app::models::flights::ModelScopes;

   let flights = flights::Model::query(&db)
       .active()
       .destination("London")
       .get::<flights::Model>()
       .await?;
   ```

3. **Memanggil Local Scopes secara Statis**
   Anda juga dapat memulai kueri langsung menggunakan method statis yang digenerate pada model:
   ```rust
   let flights = flights::Model::active(&db)
       .get::<flights::Model>()
       .await?;
   ```

4. **Global Scopes (Batasan Kueri Otomatis)**
   Global Scopes diterapkan secara otomatis pada seluruh kueri model. Tambahkan blok `global_scopes` di dalam macro `model!`:
   ```rust
   model! {
       table: "flights",
       global_scopes: {
           active_only => |q| q.where_("active", 1),
       },
       Model { ... }
   }
   ```
   Sekarang, semua pemanggilan kueri standar seperti `all()`, `find()`, `first()`, dan `count()` akan menyertakan kondisi `active = 1` secara otomatis.

5. **Melewati Global Scopes (`query_without_global_scopes`)**
   Jika Anda perlu melakukan kueri tanpa menyertakan global scopes (termasuk soft deletes), gunakan method helper `query_without_global_scopes`:
   ```rust
   let all_flights = flights::Model::query_without_global_scopes(&db)
       .get::<flights::Model>()
       .await?;
   ```

##### G. Model Events, Observers, & Service Layer

Untuk memisahkan logika bisnis kompleks dari controller (Service Layer) dan memicu aksi di berbagai siklus hidup model secara terstruktur (Model Events & Observers), RustBasic menyediakan generator CLI khusus.

1. **Membuat Service (`make:service`)**
   Service Layer digunakan untuk menampung logika bisnis terpusat yang membutuhkan akses database atau dependensi lainnya:
   ```bash
   rustbasic make:service UserService
   ```
   Ini akan menghasilkan `src/app/services/user_service.rs` dan mendaftarkan modulnya. Anda dapat menyuntikkan `AnyPool` (Database Pool) dan menuliskan fungsi logika bisnis:
   ```rust
   use rustbasic_core::sqlx::AnyPool;

   pub struct UserService {
       db: AnyPool,
   }

   impl UserService {
       pub fn new(db: AnyPool) -> Self {
           Self { db }
       }

       pub async fn register_user(&self, email: &str) -> Result<(), sqlx::Error> {
           // Jalankan logika pendaftaran & database transaksi di sini
           Ok(())
       }
   }
   ```

2. **Membuat Observer (`make:observer`)**
   Observer digunakan untuk mendengarkan perubahan state pada model tertentu secara terstruktur:
   ```bash
   rustbasic make:observer UserObserver --model=User
   ```
   Ini akan menghasilkan `src/app/observers/user_observer.rs` yang mengimplementasikan trait `UserObserver` dengan fungsi hook lengkap (`creating`, `created`, `updating`, `updated`, `deleting`, `deleted`).
   
   *Catatan: CLI memiliki fitur otomatis pendeteksian nama file model plural (seperti `users.rs` vs `user.rs`) untuk mengimpor path struct Model yang tepat.*

3. **Memicu Observer secara Eksplisit**
   Demi transparansi alur program (*explicit code execution*), event dipicu secara transparan sebelum/sesudah operasi database, baik di tingkat model wrapper maupun di Service Layer:
   ```rust
    UserObserverImpl::creating(&mut user_data);
    let user = User::create(&self.db, user_data).await?;
    UserObserverImpl::created(&user);
    ```

##### H. Eager Loading (dengan `with`)

Untuk menghindari masalah kueri **N+1**, RustBasic menyediakan fitur *eager loading* relasi yang terintegrasi langsung ke dalam macro `model!`. Fitur ini memungkinkan Anda memuat relasi (seperti `belongs_to` dan `has_many`) secara massal dengan hanya **2 kueri database** (1 untuk data utama, 1 untuk data relasi menggunakan `where_in`).

1. **Mendefinisikan Relasi di Model**
   Tambahkan blok konfigurasi `relations` di dalam makro `model!`:

   ```rust
   // src/app/models/books.rs
   model! {
       table: "books",
       relations: {
           author ( belongs_to, foreign_key: author_id, local_key: id ) => crate::app::models::authors::Model,
       },
       Model {
           pub id: i32,
           pub author_id: i32,
           pub title: String,
           pub author: Option<crate::app::models::authors::Model>,
       }
   }
   ```

   ```rust
   // src/app/models/authors.rs
   model! {
       table: "authors",
       relations: {
           books ( has_many, foreign_key: author_id, local_key: id ) => crate::app::models::books::Model,
       },
       Model {
           pub id: i32,
           pub name: String,
           pub books: Option<Vec<crate::app::models::books::Model>>,
       }
   }
   ```

2. **Melakukan Eager Loading**
   Gunakan method static `with` pada model untuk memuat relasi yang diinginkan:

   ```rust
   // Eager loading "belongs_to" (Memuat Buku beserta Penulisnya)
   let books = Book::with(&db, &["author"])
       .order_by("id", "asc")
       .get()
       .await?;

   for book in books {
       if let Some(author) = book.author {
           println!("Buku: {}, Penulis: {}", book.title, author.name);
       }
   }

   // Eager loading "has_many" (Memuat Penulis beserta semua Bukunya)
   let authors = Author::with(&db, &["books"])
       .get()
       .await?;

   for author in authors {
       if let Some(books) = author.books {
           println!("Penulis: {} menulis {} buku", author.name, books.len());
       }
   }
   ```

---

## 🔄 Perbandingan Pemakaian (Raw SQL vs ORM Model)

Berikut adalah perbandingan pemakaian antara penulisan query database SQL mentah dan penggunaan ORM Model bawaan RustBasic:

| Parameter Evaluasi         | Raw SQL Manual Query                                | ORM Model (`model!`)                                   |
| :------------------------- | :-------------------------------------------------- | :----------------------------------------------------- |
| **Sintaksis**              | `"INSERT INTO products (name) VALUES (?)"`          | `Product::create(&db, json!({"name": ".."}));`         |
| **Keamanan SQL Injection** | Rawan jika parameter tidak dibinding secara manual. | Terproteksi secara otomatis di tingkat driver model.   |
| **Validasi Tipe Data**     | Baru terdeteksi saat query dijalankan di database.  | Dicek langsung oleh compiler Rust saat proses build.   |
| **Maintanability**         | Sulit diperbarui jika ada perubahan nama kolom.     | Sangat mudah karena dikelola terpusat di berkas model. |

---

## 📊 Tabel Ringkasan Perintah Database CLI

Berikut adalah daftar perintah CLI penting untuk manajemen siklus database aplikasi:

| Perintah Terminal                | Kegunaan Utama                              | Hasil Eksekusi                                              |
| :------------------------------- | :------------------------------------------ | :---------------------------------------------------------- |
| **`rustbasic migrate`**          | Menjalankan seluruh berkas migrasi baru.    | Struktur tabel database ter-update ke versi terbaru.        |
| **`rustbasic migrate:refresh`**  | Menghapus semua tabel lalu membangun ulang. | Database bersih dan skema tabel dibangun dari awal.         |
| **`rustbasic migrate:rollback`** | Membatalkan satu langkah migrasi terakhir.  | Skema tabel dikembalikan ke versi sebelum migrasi terakhir. |
| **`rustbasic db:seed`**          | Menjalankan seluruh pengisian data seeder.  | Tabel terisi data dummy awal secara otomatis.               |

---

## 🏁 Penutup

Sistem database terpadu pada RustBasic mempermudah Anda melakukan manipulasi data (CRUD) secara aman serta menjaga konsistensi struktur tabel antar server (lokal, staging, dan produksi) menggunakan skema migrasi terpusat.
