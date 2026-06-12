# 🌐 Internasionalisasi & Penerjemah (rustbasic-translatable)

Paket `rustbasic-translatable` menyediakan sistem lokalisasi multi-bahasa (i18n) berkinerja tinggi untuk framework RustBasic. Memungkinkan aplikasi Anda menyajikan teks dalam berbagai bahasa berdasarkan preferensi deteksi otomatis per pengguna.

---

## 📁 1. Struktur Folder Bahasa (JSON)

Seluruh teks terjemahan disimpan di dalam berkas JSON di bawah satu direktori bahasa (misal: `/lang`). Struktur JSON dapat berupa satu tingkat (*flat*) maupun bersarang (*nested*).

### Contoh File `lang/id.json`:
```json
{
    "welcome": "Selamat datang kembali!",
    "greet": "Halo, {name}!",
    "auth": {
        "failed": "Kombinasi email dan kata sandi salah.",
        "throttle": "Terlalu banyak percobaan masuk. Silakan coba lagi dalam {seconds} detik."
    }
}
```

### Contoh File `lang/en.json`:
```json
{
    "welcome": "Welcome back!",
    "greet": "Hello, {name}!",
    "auth": {
        "failed": "These credentials do not match our records."
    }
}
```

---

## ⚙️ 2. Inisialisasi Translator saat Aplikasi Boot

Saat server web dijalankan, Anda perlu mendaftarkan dan memuat folder bahasa tersebut ke dalam memori aplikasi:

```rust
use rustbasic_translatable::TRANSLATOR;

fn main() {
    // Memuat seluruh file JSON dari folder /lang
    TRANSLATOR.init("lang").expect("Gagal memuat file bahasa");

    // Menentukan bahasa default global (Fallback language)
    TRANSLATOR.set_default_locale("id");
}
```

---

## 🗣️ 3. Menggunakan Fungsi Penerjemah di Kode Rust

Gunakan static global `TRANSLATOR` untuk menerjemahkan kunci teks. Kunci bersarang diakses menggunakan notasi titik (`.`).

### A. Menerjemahkan Teks Dasar (`trans`)
```rust
use rustbasic_translatable::TRANSLATOR;

// Mengambil terjemahan untuk bahasa Indonesia
let msg = TRANSLATOR.trans("welcome", "id"); 
// Output: "Selamat datang kembali!"

// Mengambil terjemahan dengan notasi titik (dot notation)
let err_msg = TRANSLATOR.trans("auth.failed", "en");
// Output: "These credentials do not match our records."

// Jika bahasa target tidak memiliki kunci tersebut, otomatis fallback ke bahasa default ("id")
let err_msg_fallback = TRANSLATOR.trans("auth.throttle", "en");
// Output: "Terlalu banyak percobaan masuk. Silakan coba lagi dalam {seconds} detik."
```

### B. Mengisi Parameter Placeholder Dinamis (`trans_with`)
```rust
let welcome_user = TRANSLATOR.trans_with(
    "greet", 
    "id", 
    &[("name", "Hendra")]
);
// Output: "Halo, Hendra!"

let rate_limit = TRANSLATOR.trans_with(
    "auth.throttle", 
    "id", 
    &[("seconds", "30")]
);
// Output: "Terlalu banyak percobaan masuk. Silakan coba lagi dalam 30 detik."
```

---

## 🛡️ 4. Deteksi Bahasa Otomatis via Middleware

Paket ini menyertakan `translatable_middleware` yang secara dinamis memeriksa preferensi bahasa klien pada setiap request HTTP secara berurutan:
1.  **Query String**: Parameter URL `?lang=en` atau `?locale=en`.
2.  **Session**: Nilai sesi aktif dengan kunci `"locale"`.
3.  **Cookies**: Nilai cookie dengan nama `lang` atau `locale`.
4.  **Header HTTP**: Menganalisis header `Accept-Language` yang dikirim oleh web browser (misal: `id-ID,id;q=0.9,en-US;q=0.8`).
5.  **Fallback**: Kembali ke konfigurasi default translator global.

### Mendaftarkan Middleware ke Router Global:
```rust
use rustbasic_core::Router;
use rustbasic_translatable::translatable_middleware;

pub fn app_router() -> Router {
    Router::new()
        // Semua rute di bawah router ini akan mendeteksi bahasa secara otomatis
        .middleware(translatable_middleware)
}
```

---

## 🔑 5. Mendapatkan Bahasa Aktif di Request Scope

Di dalam handler/controller, Anda dapat memanggil `get_locale()` untuk mendeteksi bahasa apa yang saat ini sedang aktif secara thread-safe untuk request saat ini:

```rust
use rustbasic_core::{Response, ResponseHelper};
use rustbasic_translatable::{TRANSLATOR, get_locale};

pub async fn home_controller() -> Response {
    let locale = get_locale(); // Mendapatkan kode bahasa aktif (cth: "en")
    
    let welcome_text = TRANSLATOR.trans("welcome", &locale);

    ResponseHelper::html(&format!("<h1>{}</h1>", welcome_text))
}
```
