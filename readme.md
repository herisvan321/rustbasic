# 🦀 RustBasic Starter Kit

**RustBasic** adalah framework monolith modern untuk bahasa pemrograman Rust, yang dirancang untuk kecepatan pengembangan maksimal namun dengan performa dan keamanan Rust.

## 🚀 Fitur Unggulan
- ⚡ **Axum Powered**: Backend super cepat dan efisien.
- 🗄️ **Sea-ORM**: Manajemen database async yang mudah dan aman.
- 🎨 **Minijinja**: Engine template HTML standar yang familiar.
- 🛡️ **Built-in CLI**: Generator kode (Controller, Model, Auth) otomatis.
- 🔄 **Live Reload**: Refresh browser otomatis saat ada perubahan kode/template.

## 🚀 Instalasi Global (Semua Platform)

Instal CLI RustBasic secara global menggunakan Cargo. Pastikan Anda sudah menginstal Rust di sistem Anda.

### Semua Platform (Mac, Linux, & Windows):
Jalankan perintah berikut di terminal:
```bash
cargo install --git https://github.com/herisvan321/rustbasic
```

> [!NOTE]
> Perintah `rustbasic` akan **langsung aktif**. Jika belum muncul setelah instalasi, silakan restart terminal Anda atau jalankan `source ~/.zshrc` (Mac/Linux).

---

## 🛠️ Penggunaan CLI (`rustbasic`)

RustBasic menggunakan satu perintah utama `rustbasic` yang sangat cepat dan mudah digunakan:

### Membuat Project Baru
```bash
rustbasic new nama_project_anda
```

### Menjalankan Server (Auto-Reload)
```bash
rustbasic serve
```

### Scaffolding (Pembuatan Kode Otomatis)
```bash
rustbasic make:controller BlogController
rustbasic make:model Post -m             # -m untuk otomatis membuat migration
rustbasic make:auth                      # Membuat sistem Login/Register instan
```

### Database & Migrasi
```bash
rustbasic migrate                        # Jalankan migrasi
rustbasic migrate:refresh                # Reset dan jalankan ulang migrasi
rustbasic db:seed                        # Jalankan seeder database
```

---

## 📝 Panduan Pengembangan
1. **Model**: Terletak di `src/app/models/`
2. **Controller**: Terletak di `src/app/http/controllers/`
3. **View (Template)**: Terletak di `src/resources/views/` (Format `.rb.html`)
4. **Routes**: Konfigurasi route ada di `src/routes/web.rs`

---

## 🤝 Kontribusi
Framework ini bersifat open source. Silakan kirimkan Pull Request atau laporkan Issue di repositori GitHub kami.

**Selamat membangun aplikasi hebat dengan RustBasic! 🚀**
Ditenagai oleh [rustbasic-core](https://github.com/herisvan321/rustbasic-core).
Dipersembahkan oleh Tim RustBasic.
