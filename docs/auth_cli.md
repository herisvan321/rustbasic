# 🔐 Panduan Autentikasi Breeze

## 📝 Kata Pengantar
Selamat datang di panduan **Breeze Authentication Scaffolding**. Dokumentasi ini dirancang khusus untuk memandu Anda memasang dan mengelola sistem login, registrasi, lupa password, reset kata sandi, dan halaman dashboard premium secara instan. Breeze mengotomatisasi penyusunan berkas backend (Rust) dan frontend (React) sehingga Anda dapat memiliki sistem autentikasi yang aman dan siap pakai dalam hitungan detik.

---

## 🛠️ Script Contoh

### A. Penambahan Paket Breeze ke Bagian dependencies (`Cargo.toml`)
```toml
[dependencies]
rustbasic-core = "0.1"
rustbasic-breeze = "0.0"
```

### B. Hashing Sandi Pengguna Menggunakan Bcrypt
```rust
use rustbasic_core::bcrypt::{hash, DEFAULT_COST};

// Hashing password dengan salt cost aman (12) sebelum disimpan di database
let password_hash = hash("password_rahasia_saya", DEFAULT_COST).unwrap();
```

### C. Proteksi Rute Melalui Auth Middleware (`src/routes/web.rs`)
```rust
use rustbasic_core::{Router, get, from_fn, AppState};
use crate::app::http::controllers::dashboard_controller;
use crate::app::http::middleware::auth::auth_middleware;

pub fn router() -> Router<AppState> {
    Router::new()
        // Rute dashboard hanya bisa diakses oleh user yang telah login
        .route("/dashboard", get(dashboard_controller::index))
        .layer(from_fn(auth_middleware))
}
```

---

## 🔄 Perbandingan Pemakaian (Autentikasi Manual vs Scaffolding Breeze)

Berikut adalah perbandingan pemakaian antara menulis sistem keamanan autentikasi secara manual dan menggunakan scaffolding otomatis Breeze:

| Parameter Proses | Membangun Sistem Manual | Menggunakan Scaffolding Breeze |
| :--- | :--- | :--- |
| **Waktu Pembangunan** | Membutuhkan waktu berhari-hari untuk kode & UI. | Instan (terbakar otomatis saat aplikasi dijalankan). |
| **Struktur File** | Rawan tidak konsisten dan sulit dirawat. | Sangat rapi mengikuti konvensi standard proyek. |
| **Fitur Lupa Password** | Harus membuat UUID token & integrasi SMTP sendiri. | Sudah siap pakai beserta template email HTML bawaan. |
| **Halaman Visual** | Harus mendesain form login/register sendiri. | Disediakan halaman login split-screen premium & modern. |

---

## 📊 Tabel Ringkasan Berkas Tergenerasi

Berikut adalah berkas penting yang didelegasikan secara otomatis oleh Breeze ke folder proyek Anda:

| Berkas Terbuat | Lokasi Penyimpanan File | Deskripsi Peran Berkas |
| :--- | :--- | :--- |
| **Controller Auth** | `src/app/http/controllers/auth/auth_controller.rs` | Mengolah request masuk login, registrasi, logout, & reset sandi. |
| **Middleware Auth** | `src/app/http/middleware/auth.rs` | Penjaga pintu rute yang membelokkan user tak dikenal ke `/login`. |
| **Rute Tamu** | `src/routes/auth.rs` | Rute web terpisah untuk penanganan login, daftar, & lupa kata sandi. |
| **Rute Dashboard** | `src/routes/dashboard.rs` | Rute web terpisah terproteksi untuk dashboard utama. |
| **Halaman React** | `src/resources/js/Pages/Auth/` | Kumpulan file UI React (.jsx) halaman login, register, & reset sandi. |
| **Template Email** | `src/resources/views/emails/` | Template email HTML untuk ucapan selamat datang & reset link. |

---

## 🗑️ Otomatisasi Uninstall & Clean-Up Sistem

Apabila Anda memutuskan untuk menghapus module autentikasi Breeze, Anda dapat menjalankan perintah uninstall berikut:
```bash
cargo run --package rustbasic-cli -- uninstall rustbasic-breeze
```
Perintah ini akan secara otomatis melakukan pembersihan total:
* Menghapus seluruh file routing (`src/routes/auth.rs`, `src/routes/dashboard.rs`), controllers, models, middleware, views, dan email templates yang digenerate.
* Mengupdate file `src/routes/web.rs` dan file mod deklarasi lainnya untuk melepas routing secara bersih tanpa meninggalkan *warning unused imports*.
* Menghapus riwayat pencatatan migrasi tabel Breeze dari database secara otomatis.

---

## 🏁 Penutup
Dengan memanfaatkan paket autentikasi Breeze, Anda mendapatkan fondasi keamanan sistem akun yang teruji secara industri, terlindung dari eksploitasi celah CSRF, serta siap melayani pengguna dengan tampilan visual yang sangat premium.
