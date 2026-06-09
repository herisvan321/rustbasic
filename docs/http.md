# 🌐 Panduan HTTP Stack, CSRF, & Middleware

## 📝 Kata Pengantar
Selamat datang di panduan **HTTP Stack, CSRF, & Middleware**. Dokumentasi ini dirancang khusus untuk memandu Anda memahami cara server memproses permintaan request, menyaring akses melalui tumpukan middleware, menangani keamanan CSRF secara otomatis, dan mengembalikan respon yang ramah terhadap protokol komunikasi React SPA.

Tumpukan HTTP engine RustBasic dibangun di atas performa tinggi yang disesuaikan secara khusus untuk menangani transisi asinkron Inertia.js secara mulus dan aman.

---

## 🛠️ Script Contoh

### A. Registrasi Middleware pada Rute (`src/routes/web.rs`)
Mengamankan sekelompok rute menggunakan middleware kustom dengan bantuan fungsi `from_fn` dari framework:
```rust
use rustbasic_core::{Router, get, from_fn, AppState};
use crate::app::http::controllers::admin_controller;
use crate::app::http::middleware::admin_auth::admin_auth_middleware;

pub fn router() -> Router<AppState> {
    Router::new()
        // Mengamankan rute dashboard menggunakan middleware kustom
        .route("/admin/dashboard", get(admin_controller::index))
        // Seluruh rute di atas baris .layer() akan disaring oleh middleware tersebut
        .layer(from_fn(admin_auth_middleware))
}
```

### B. Struktur Middleware Kustom (`src/app/http/middleware/auth.rs`)
Menulis fungsi middleware asinkron dengan parameter `Request` dan `Next` chain:
```rust
use rustbasic_core::{Request, Response, Next, IntoResponse, Redirect};

pub async fn check_auth_middleware(req: Request, next: Next) -> Response {
    // 1. Mengecek apakah user session ID ada di memori sesi
    if req.session.get::<i32>("user_id").is_none() {
        // Jika tidak ada, batalkan request dan redirect ke halaman login
        return Redirect::to("/login").into_response();
    }
    
    // 2. Teruskan request ke middleware berikutnya atau ke controller handler
    next.run(req).await
}
```

### C. Redirect 303 Khusus SPA di Controller
Melakukan redirect yang kompatibel dengan alur transaksi data Inertia React SPA:
```rust
use rustbasic_core::{IntoResponse, Redirect};

pub async fn store_data() -> impl IntoResponse {
    // Wajib menggunakan status 303 agar browser React SPA Inertia melakukan reload data dengan aman
    Redirect::to("/dashboard")
}
```

#### Redirect Menggunakan Rute Bernama (Named Routes)
Anda juga dapat melakukan pengalihan secara dinamis menggunakan rute bernama yang terdaftar di registry backend Rust:
```rust
pub async fn store_data_named() -> impl IntoResponse {
    // Ambil path rute bernama "dashboard" dari registry global
    let path = rustbasic_core::router::get_named_routes()
        .get("dashboard")
        .cloned()
        .unwrap_or_else(|| "/".to_string());
        
    Redirect::to(&path)
}
```

---

## 🛡️ Proteksi Keamanan CSRF (Cross-Site Request Forgery)

Secara default, RustBasic menyertakan perlindungan CSRF otomatis untuk mengamankan data aplikasi Anda dari serangan manipulasi sesi pihak ketiga.

### 1. Cara Kerja CSRF Middleware
Setiap kali request masuk, middleware CSRF memeriksa status sesi:
- **Penyediaan Token**: Jika sesi belum memiliki token CSRF, string alfanumerik acak sepanjang 40 karakter akan digenerate secara otomatis dan disimpan di session (`_token`). Token ini disematkan ke dalam tag `<meta name="csrf-token" content="...">` di template HTML utama (`app.rb.html`).
- **Pengecekan Mutasi**: Untuk semua request yang bersifat mutasi (seperti POST, PUT, PATCH, DELETE), middleware akan membandingkan token yang dikirimkan oleh klien di header `x-csrf-token` dengan token yang tersimpan di session server.
- **Respon Kegagalan**: Jika token tidak cocok atau tidak disertakan, server akan menolak request tersebut dan mengembalikan status HTTP **419 (Page Expired)**.

### 2. Integrasi Otomatis dengan Axios Frontend
Di dalam berkas bootstrap `main.tsx` frontend Anda, token CSRF dibaca dari meta tag dan secara otomatis disematkan sebagai header default untuk setiap request AJAX:
```javascript
const csrfToken = document.querySelector('meta[name="csrf-token"]')?.getAttribute('content');
if (csrfToken) {
  window.axios.defaults.headers.common['X-CSRF-TOKEN'] = csrfToken;
}
```

---

## 📡 HTTP Client (Fluent API Client)

RustBasic menyediakan wrapper HTTP Client fluent yang terintegrasi secara asinkron di atas library `reqwest` dan `serde`. Fitur ini memungkinkan Anda mengirim permintaan API eksternal (seperti integrasi Payment Gateway atau API pihak ketiga) dengan sintaksis berantai secara fluent.

### A. GET Request dengan Query Parameter & Token
Anda dapat mengirimkan request GET dengan bearer token, kustom header, dan query parameter secara terantai, kemudian melakukan deserialisasi respon JSON ke typed `struct` secara otomatis:

```rust
use rustbasic_core::{Http, serde::Deserialize};

#[derive(Deserialize, Debug)]
struct GithubUser {
    id: i64,
    login: String,
    name: Option<String>,
}

let response = Http::get("https://api.github.com/users/octocat")
    .with_token("secret_github_token")
    .header("User-Agent", "RustBasic-App")
    .query(serde_json::json!({ "page": 1 }))
    .send()
    .await?;

if response.is_success() {
    let user: GithubUser = response.json().await?;
    println!("User: {} ({:?})", user.login, user.name);
}
```

### B. POST Request dengan JSON Payload & Basic Auth
Untuk request POST yang mengirimkan data JSON dan menggunakan autentikasi Basic Auth:

```rust
use rustbasic_core::{Http, serde::Serialize};

#[derive(Serialize)]
struct CreateProduct {
    name: String,
    price: u64,
}

let payload = CreateProduct {
    name: "Laptop Asus ROG".to_string(),
    price: 15_000_000,
};

let response = Http::post("https://api.example.com/products")
    .basic_auth("username_api", Some("password_api"))
    .json(&payload)
    .send()
    .await?;

if response.status().as_u16() == 201 {
    let result: serde_json::Value = response.json_value().await?;
    println!("Produk berhasil dibuat: {:?}", result);
}
```

### C. Ringkasan Method HTTP Client

| Method Builder | Deskripsi | Kasus Penggunaan |
| :--- | :--- | :--- |
| **`Http::get(url)`** | Inisialisasi request GET. | Mengambil data dari API luar. |
| **`Http::post(url)`** | Inisialisasi request POST. | Mengirimkan data baru ke API luar. |
| **`Http::put(url)`** | Inisialisasi request PUT. | Memperbarui data secara penuh di API luar. |
| **`Http::delete(url)`** | Inisialisasi request DELETE. | Menghapus data di API luar. |
| **`with_token(token)`** | Menambahkan header `Authorization: Bearer <token>`. | Autentikasi OAuth / JWT. |
| **`basic_auth(user, pass)`** | Menambahkan header `Authorization: Basic <base64>`. | Autentikasi API key basic. |
| **`header(key, val)`** | Menambahkan kustom header HTTP. | Menyertakan format data khusus (misal `User-Agent`). |
| **`query(params)`** | Menambahkan query parameter pada URL. | Pagination, filter pencarian. |
| **`json(body)`** | Menyematkan payload data JSON. | Mengirimkan request body data model. |
| **`timeout(duration)`** | Membatasi durasi tunggu request. | Mencegah aplikasi terhenti jika server luar lambat. |

---

## 🔄 Perbandingan Pemakaian (Redirect 302 vs Redirect 303)

Berikut adalah perbandingan pemakaian kode status HTTP redirect untuk aplikasi Single Page Application (SPA):

| Parameter | Redirect 302 (Found) | Redirect 303 (See Other) |
| :--- | :--- | :--- |
| **Deskripsi** | Standar pengalihan HTTP tradisional. | Pengalihan khusus untuk mengubah metode HTTP request. |
| **Dampak Pada Inertia Client**| Klien Inertia mendeteksi kegagalan karena browser berupaya menggunakan metode request asal (misal POST/PUT) pada URL tujuan redirect. | Klien Inertia mendeteksi redirect dengan benar, membatalkan request lama, lalu memicu request GET bersih ke URL tujuan. |
| **Kasus Penggunaan** | Pengalihan rute statis biasa (bukan hasil pemrosesan form POST/PUT/DELETE). | **Wajib digunakan** setelah pemrosesan input formulir di controller yang mengubah data. |

---

## 📊 Tabel Ringkasan Respon Server

Berikut adalah jenis respon HTTP yang disediakan oleh backend RustBasic:

| Jenis Respon | Sintaks Kode | Deskripsi & Kegunaan |
| :--- | :--- | :--- |
| **Inertia Render** | `inertia(&req, "Welcome", props)` | Merender halaman React SPA dengan data props yang dinamis. |
| **Redirect SPA** | `Redirect::to("/url")` | Mengalihkan halaman browser SPA dengan status 303 secara otomatis. |
| **JSON API** | `Json(json!({ ... }))` | Mengembalikan respon JSON murni untuk integrasi API pihak ketiga. |
| **HTML Polos** | `Html(content)` | Menyajikan file teks HTML murni secara langsung ke browser. |

---

## 🏁 Penutup
Dengan memanfaatkan tumpukan middleware, perlindungan CSRF otomatis berbasis cookie, dan pemilihan respon HTTP yang tepat, aplikasi web Anda tidak hanya memiliki performa secepat kilat tetapi juga memiliki pertahanan keamanan yang sangat kokoh dari ancaman pembajakan sesi.
