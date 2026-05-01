# Dokumentasi Proyek RustMyAdmin (Axum SPA)

Aplikasi ini adalah sebuah *Single Page Application* (SPA) berbasis Rust yang menggunakan framework web Axum. Aplikasi ini dirancang menggunakan arsitektur folder ala Laravel untuk menjaga kode tetap rapi dan terstruktur, dipadukan dengan **Minijinja** sebagai templating engine (mirip Blade), serta **HTMX** dan **Alpine.js** untuk interaktivitas *frontend*.

---

## 📂 Struktur Direktori

Aplikasi ini mengadopsi struktur mirip framework PHP Laravel atau MVC modern:

```text
rustmyadmin/
├── Cargo.toml            # Konfigurasi dependensi proyek Rust
├── public/               # File statis yang dapat diakses publik (CSS, JS, Gambar)
│   └── css/
│       └── style.css     # Vanilla CSS untuk tampilan premium aplikasi
├── resources/
│   └── views/            # Template HTML (dirender oleh Minijinja)
│       ├── layouts/
│       │   └── app.html  # Layout utama (base layout) berisi CDN HTMX & Alpine.js
│       └── welcome.html  # Tampilan Welcome page
└── src/
    ├── main.rs           # Entry point utama aplikasi dan konfigurasi server Axum
    ├── routes/           # Pengaturan rute aplikasi (Routing)
    │   ├── mod.rs
    │   └── web.rs        # Registrasi endpoint ke controller
    └── app/              # Logika inti aplikasi (Application Layer)
        ├── mod.rs        # Inisialisasi Minijinja environment & fungsi render
        └── http/
            ├── mod.rs
            └── controllers/
                ├── mod.rs
                └── welcome_controller.rs  # Logika untuk menangani request Welcome Page
```

---

## ⚙️ Penggunaan & Teknologi

Aplikasi ini dibangun di atas tumpukan teknologi berikut:

- **Backend (Rust & Axum):** Menangani routing dan rendering sisi server secara *async*. Sangat cepat, aman, dan ringan.
- **Minijinja:** Digunakan untuk me-render HTML dari folder `resources/views`. Mirip seperti *Blade* di Laravel, fitur utamanya adalah pewarisan template (`{% extends "layouts/app.html" %}`).
- **HTMX:** Memungkinkan navigasi bergaya *SPA (Single Page Application)* tanpa perlu merender ulang seluruh halaman. Hal ini dilakukan dengan menambahkan parameter `hx-boost="true"` pada tag `<body>` di layout.
- **Alpine.js:** Framework JavaScript mini untuk menangani interaksi UI instan di sisi klien (seperti *dropdown*, *modal*, efek klik, atau animasi).
- **Vanilla CSS:** CSS murni digunakan tanpa framework eksternal tambahan (seperti Tailwind) agar tampilan estetik (*Glassmorphism*, gradasi warna premium) dapat dioptimalkan tanpa menambah *bloat*.

---

## 🔄 Alur dari Aplikasi

Berikut adalah tahapan siklus hidup *(lifecycle)* bagaimana aplikasi menangani permintaan (request) masuk dari pengguna, misalnya ketika mengakses halaman utama (`/`):

1. **User Request:** Pengguna membuka URL `http://localhost:3000/` di peramban web (browser).
2. **Entry Point (`src/main.rs`):** Server Axum menerima request dan mencari pengaturan rute yang sesuai dengan URL tersebut.
3. **Router (`routes/web.rs`):** Request untuk rute `/` dilempar ke controller yang tepat, dalam hal ini adalah fungsi `index` milik `welcome_controller`.
4. **Controller (`welcome_controller.rs`):** Fungsi `index()` dijalankan. Di sini kita menyiapkan data/konteks (misal: mengirim teks variabel `title`) lalu memanggil fungsi `render()`.
5. **View Rendering (`app/mod.rs` & `resources/views/`):** Fungsi `render()` menggunakan engine **Minijinja** untuk membaca template `welcome.html`. Minijinja lalu menggabungkan file tersebut dengan *layout* dasar dari `layouts/app.html`.
6. **Response ke User:** Server mengembalikan kode HTML yang sudah jadi ke browser. Bersamaan dengan itu, browser juga akan meminta file statis CSS yang terhubung langsung melalui `Router::fallback_service` yang menunjuk ke direktori `public/`.
7. **Interaksi Frontend:** Setelah halaman selesai dimuat, HTMX dan Alpine.js langsung bekerja. Jika Anda berpindah rute (melalui link) atau mengklik tombol, interaksi akan langsung ditangani secara mulus layaknya SPA murni.

---

## 🚀 Cara Menjalankan

Aplikasi ini menggunakan sistem Cargo milik ekosistem Rust. Ikuti panduan ini:

1. Buka terminal Anda.
2. Arahkan direktori aktif ke dalam folder utama proyek ini:
   ```bash
   cd "rustmyadmin"
   ```
   *(Pastikan terminal Anda sudah berada di `/Users/herisvanhendra/Desktop/Desktop new/project/belajar rust/rustmyadmin`)*
3. Ketik perintah berikut untuk langsung mengompilasi sekaligus menjalankan server:
   ```bash
   cargo run
   ```
4. Tunggu beberapa detik hingga proses selesai. Anda akan melihat log pesan:
   `INFO rustmyadmin: listening on 0.0.0.0:3000`
5. Buka Web Browser kesayangan Anda dan pergi ke alamat:
   **[http://localhost:3000](http://localhost:3000)**

> **Tips Pengembangan:**
> - Jika Anda mengubah kode Rust (berakhiran `.rs`), hentikan aplikasi (tekan `Ctrl + C` di terminal) dan jalankan `cargo run` kembali.
> - Jika Anda hanya mengubah isi HTML (`resources/views/`) atau CSS (`public/css/`), Anda **tidak perlu** *restart* server. Cukup *refresh* (*reload*) halaman di browser Anda.
