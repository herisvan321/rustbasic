# 🦀 RustBasic Starter Kit

**RustBasic** adalah framework Full-stack modern untuk bahasa pemrograman Rust, yang dirancang untuk kecepatan pengembangan maksimal namun dengan performa dan keamanan Rust. Dilengkapi antarmuka bergaya premium yang memadukan tema modern Y2K cerah dengan utilitas penuh **Bootstrap 5.3.8** dan **HTMX**.

## 🚀 Fitur Unggulan
- ⚡ **Axum Powered**: Backend super cepat dan efisien berbasis **Axum 0.8+**.
- 🗄️ **Sea-ORM**: Manajemen database *async* yang mudah, aman, dan berkinerja tinggi.
- 🎨 **Minijinja & Interseptor Global**: Engine template HTML standar dengan injeksi otomatis variabel global (seperti `user_roles`, `user_permissions`, dan `csrf_token`) di setiap *view* tanpa mengotori pengontrol.
- 📦 **Bootstrap 5.3.8 Ready**: Pustaka gaya, *grid*, dan komponen interaktif Bootstrap siap pakai secara murni melalui penyajian berkas statis lokal (`public/`).
- 🛡️ **Built-in CLI**: Generator kode (Controller, Model, Auth) otomatis.
- 🔄 **Live Reload**: Refresh browser otomatis saat ada perubahan kode/template.

## ️ Instalasi Manual (Via Cargo)
```bash
cargo install rustbasic-cli
```
*Catatan: Pastikan direktori bin cargo (`~/.cargo/bin`) sudah masuk ke dalam PATH sistem Anda.*

---

## 🛠️ Penggunaan CLI (`rustbasic`)

RustBasic menyediakan CLI yang powerful untuk mempercepat workflow pengembangan Anda:

### Manajemen Project
```bash
rustbasic new nama_app        # Membuat project baru
rustbasic serve               # Menjalankan server (Auto-Reload/Hot-Reload)
rustbasic key:generate        # Generate APP_KEY baru di .env
```

### Scaffolding (Generator Kode)
```bash
rustbasic make:controller BlogController
rustbasic make:model Post -m             # -m untuk otomatis membuat migration
rustbasic make:auth                      # Membuat sistem Login/Register instan
```

### Database & Migrasi
```bash
rustbasic make:migration create_users    # Membuat file migrasi baru (Create)
rustbasic make:migration:add bio users  # Membuat migrasi tambah kolom (Add)
rustbasic migrate                        # Jalankan migrasi
rustbasic migrate:refresh                # Reset dan jalankan ulang migrasi
rustbasic db:seed                        # Jalankan seeder database
```

---

## 🌐 API Documentation
RustBasic hadir dengan struktur rute API yang terpisah dan sudah mendukung **CORS**. Anda dapat mengelola rute API di `src/routes/api.rs`.

Endpoint bawaan:
- `GET /api/health`: Mengecek status kesehatan server.
- `GET /api/version`: Menampilkan informasi versi framework.

> [!NOTE]
> Rute API secara otomatis melewati proteksi CSRF, sehingga cocok untuk dipanggil oleh aplikasi Mobile atau Frontend (React/Vue/Next.js).

---

## ⚙️ Environment (.env)
Pastikan Anda telah mengonfigurasi file `.env` sebelum menjalankan aplikasi:
- `APP_URL`: Alamat dasar aplikasi (Default: `http://localhost:4000`).
- `DATABASE_URL`: Koneksi database (SQLite/MySQL/PostgreSQL).
- `APP_KEY`: Kunci enkripsi aplikasi (Gunakan `rustbasic key:generate` jika kosong).

---

## 📝 Panduan Pengembangan
1. **Model**: Terletak di `src/app/models/`
2. **Controller**: Terletak di `src/app/http/controllers/`
3. **View (Template)**: Terletak di `src/resources/views/` (Format `.rb.html`)
4. **Routes**:
   - `src/routes/web.rs`: Untuk rute web (dengan CSRF).
   - `src/routes/api.rs`: Untuk rute API (dengan CORS).
5. **Aset Statis**: Berkas statis publik diletakkan di folder `public/`.
   - Secara *default*, file statis tidak diekspos secara otomatis ke root URL untuk mencegah kebocoran file penting dan melindungi Custom Error 404 milik framework.
   - Gunakan `.nest_service()` di `src/main.rs` untuk mendaftarkan folder yang ingin diakses ke publik. Contoh:
     ```rust
     .nest_service("/css", ServeDir::new("public/css"))
     .nest_service("/js", ServeDir::new("public/js"))
     ```
   - Dengan konfigurasi di atas, file `public/css/style.css` dapat diakses di HTML via `<link href="/css/style.css">`.
6. **CLI Handler**: Logika perintah CLI delegasi berada di `src/cli.rs`.

---

## 🤝 Kontribusi
Framework ini bersifat open source. Silakan kirimkan Pull Request atau laporkan Issue di repositori GitHub kami.

**Selamat membangun aplikasi hebat dengan RustBasic! 🚀**
Ditenagai oleh [rustbasic-core](https://github.com/herisvan321/rustbasic-core) & [rustbasic-cli-install](https://github.com/herisvan321/rustbasic-cli-install).
Dipersembahkan oleh Tim RustBasic.
