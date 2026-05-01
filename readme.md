# 🚀 RustBasic (Axum SPA)

Aplikasi manajemen database SQLite yang ringan, cepat, dan modern. Proyek ini dirancang agar sangat mudah dipelajari dengan kode yang bersih dan terstruktur.

---

## 💎 Fitur Unggulan
- **⚡ Super Cepat**: Menggunakan Rust & Axum untuk performa maksimal.
- **🎨 Desain Premium**: Tampilan *Glassmorphism* modern dengan Vanilla CSS.
- **🔄 SPA Experience**: Navigasi mulus tanpa reload halaman berkat **HTMX**.
- **🪄 Interaktif**: Efek UI instan menggunakan **Alpine.js**.
- **📂 Arsitektur Bersih**: Struktur folder ala Laravel yang rapi dan logis.

---

## 📂 Struktur Folder
Berikut adalah susunan file terbaru dalam proyek ini:

### 📑 Label: Struktur Direktori Proyek
```text
rustbasic/
├── public/               # File statis (CSS, JS, Gambar)
│   └── css/
│       └── style.css     # File CSS Utama
├── resources/
│   └── views/            # Template HTML (Minijinja)
│       └── layouts/      # Kerangka utama (Base Layout)
├── src/
│   ├── main.rs           # Jantung aplikasi (Entry Point)
│   ├── routes/           # Pengatur jalan/URL (Routing)
│   └── app/              # Logika bisnis & Controller
└── Cargo.toml            # Daftar dependensi & konfigurasi
```

---

## 📝 Sistem Pelabelan Kode
Untuk mempermudah Anda belajar, setiap file penting telah ditandai dengan label khusus:
- **`📑 LABEL: ...`**: Menandakan fungsi utama dari file tersebut.
- **Komentar Bahasa Indonesia**: Penjelasan sederhana di setiap baris kode yang kompleks.

---

## 🚀 Cara Menjalankan

### 📑 Label: Perintah Masuk ke Folder
```bash
cd "rustbasic"
```

### 📑 Label: Perintah Menjalankan Aplikasi
```bash
cargo run
```

Setelah muncul pesan `Server berjalan di: http://0.0.0.0:3000`, buka:
👉 **[http://localhost:3000](http://localhost:3000)**

---

## 💡 Tips Pengembangan
- **Auto-Reload?** Saat ini Anda perlu menjalankan ulang `cargo run` jika mengubah kode Rust (`.rs`).
- **Update Tampilan?** Cukup ubah file di `resources/views` atau `public/css` dan *refresh* browser (tidak perlu restart server).

---

## 🛠️ Troubleshooting
- **Port 3000 sibuk?** Jika muncul error `Address already in use`, pastikan tidak ada aplikasi lain yang menggunakan port 3000, atau hentikan server sebelumnya dengan `Ctrl + C`.

---
*Dibuat dengan ❤️ untuk komunitas Rust Indonesia.*
