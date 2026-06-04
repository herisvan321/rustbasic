# 🛣️ Panduan Perutean Inline (Closure Routing)

## 📝 Kata Pengantar
Selamat datang di panduan **Perutean Inline RustBasic**. Dokumentasi ini dirancang khusus untuk memandu Anda memahami cara menulis rute web, memproses data request, dan langsung mengembalikan respon visual secara instan menggunakan closure asinkron secara langsung, tanpa harus memisahkan kode ke dalam berkas controller tersendiri.

Konsep ini sangat ideal untuk prototyping cepat, halaman web statis sederhana (seperti halaman Syarat & Ketentuan, FAQ, Kebijakan Privasi), pengujian endpoint API kilat, atau redirect rute legacy.

---

## 🛠️ Script Contoh

### A. Perutean Teks Sederhana Inline (`src/routes/web.rs`)
Mengembalikan teks murni secara instan menggunakan closure asinkron tanpa parameter:
```rust
use rustbasic_core::{Router, get, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        // Mengembalikan teks murni secara langsung
        .route("/quick-test", get(|| async { 
            "Ini rute inline instan!" 
        }))
}
```

### B. Mengambil URL Parameter & Query Parameter
Menangkap parameter dinamis dari URL (misal: `/user/:id`) dan query string (misal: `?status=active`):
```rust
use rustbasic_core::{Router, get, AppState, Request};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/:id", get(|req: Request| async move {
            // 1. Ekstrak parameter dinamis dari URL path
            let user_id = req.param("id").unwrap_or("0");
            
            // 2. Ekstrak query parameter dari URL string
            let filter = req.query("status").unwrap_or("semua");
            
            format!("Mencari user ID: {} dengan filter status: {}", user_id, filter)
        }))
}
```

### C. Merender Komponen React SPA Secara Inline
Menggunakan jembatan `inertia` langsung di dalam rute inline untuk menyajikan komponen React:
```rust
use rustbasic_core::{Router, get, AppState, Request, serde_json::json};
use crate::app::inertia;

pub fn router() -> Router<AppState> {
    Router::new()
        // Langsung menyajikan komponen Welcome.jsx dengan props tersemat
        .route("/faq", get(|req: Request| async move {
            inertia(&req, "Welcome", json!({
                "title": "FAQ Bantuan Aplikasi",
                "is_logged_in": false,
                "auth_installed": true
            }))
        }))
}
```

---

## 🚀 Konsep & Arsitektur Closure Handler

Di dalam framework RustBasic, rute inline memanfaatkan sistem **Trait Handler** asinkron. RustBasic secara cerdas mendeteksi tipe closure berdasarkan parameter input yang Anda berikan (*Arity*). Ada tiga tanda tanda (*signature*) utama yang didukung:

### 1. Arity 0: Tanpa Parameter (`Fn() -> Fut`)
Cocok untuk respon statis sederhana yang tidak memerlukan informasi dari request.
```rust
|| async { "Respon statis" }
```

### 2. Arity 1: Dengan Parameter Request (`Fn(Request) -> Fut`)
Digunakan jika Anda membutuhkan data masukan, sesi (session), atau cookie dari klien.
```rust
|req: Request| async move {
    let host = &req.path;
    format!("Path saat ini: {}", host)
}
```

### 3. Arity 2: Dengan State & Request (`Fn(State<AppState>, Request) -> Fut`)
Digunakan apabila rute inline Anda memerlukan akses ke database pool, cache, atau konfigurasi aplikasi global.
```rust
use rustbasic_core::{State, AppState};

|State(state), req: Request| async move {
    let app_name = &state.config.app_name;
    format!("Nama aplikasi di konfigurasi: {}", app_name)
}
```

> [!WARNING]
> **Kepemilikan Data (`async move`)**
> Selalu sertakan keyword `move` sebelum blok `async` pada closure yang menerima parameter. Ini memastikan compiler Rust memindahkan kepemilikan variabel (seperti `req` atau `state`) ke dalam masa hidup masa depan (*future*) asinkron dengan aman.

---

## 📊 Tabel Ringkasan Pintasan Perutean Builder

Struktur `Router` menyediakan beberapa fungsi pembantu instan agar penulisan rute inline lebih ringkas tanpa menulis closure manual:

| Metode Helper | Deskripsi Fungsi | Contoh Penulisan Kode |
| :--- | :--- | :--- |
| **`.get_json(path, data)`** | Membuat rute GET yang langsung mengembalikan payload data JSON. | `.get_json("/status", json!({"status": "healthy"}))` |
| **`.get_redirect(path, to)`** | Membuat rute GET redirect status 303 ke URL lain. | `.get_redirect("/beranda", "/")` |
| **`.get_view(path, tpl, ctx)`**| Membuat rute GET untuk merender HTML template Minijinja. | `.get_view("/welcome", "app.rb.html", json!({}))` |

---

## 🔄 Perbandingan Pemakaian (Rute Inline vs Rute Controller)

Berikut adalah panduan pengambilan keputusan untuk memilih metode penulisan rute yang tepat:

| Kriteria Keputusan | Perutean Inline (Closure) | Perutean Controller (Class Method) |
| :--- | :--- | :--- |
| **Sintaksis** | `.route("/url", get(|| async { ... }))` | `.route("/url", get(welcome_controller::index))` |
| **Lokasi Logika** | Ditulis menyatu di berkas `src/routes/web.rs` atau `api.rs`. | Ditulis secara terpisah di berkas `src/app/http/controllers/`. |
| **Tingkat Kompleksitas** | Sangat rendah (prototyping, static content, testing). | Menengah hingga tinggi (transaksi database, pengolahan form). |
| **Keterbacaan Kode** | Rawan berantakan jika logika melebihi 15 baris kode. | Sangat rapi karena logika dipisah rapi secara modular. |
| **Penggunaan Kembali (Reusable)**| Logika terkunci hanya untuk rute tersebut. | Method controller bisa dipanggil di test atau di rute lain. |

---

## 🏁 Penutup
Perutean inline memberikan kemudahan luar biasa bagi pengembang untuk membuat halaman statis atau purwarupa (mockup) dengan cepat, menghemat penulisan berkas controller untuk tugas-tugas yang sederhana. Namun, demi menjaga kemudahan pemeliharaan (*maintainability*), selalu pindahkan logika Anda ke berkas Controller ketika fungsionalitasnya mulai bertambah kompleks.
