# 🦀 RustBasic Starter Kit

**RustBasic** adalah framework monolith modern untuk bahasa pemrograman Rust, yang dirancang untuk kecepatan pengembangan maksimal namun dengan performa dan keamanan Rust.

## 🚀 Fitur Unggulan
- ⚡ **Axum Powered**: Backend super cepat dan efisien.
- 🗄️ **Sea-ORM**: Manajemen database async yang mudah dan aman.
- 🎨 **Minijinja**: Engine template HTML standar yang familiar.
- 🛡️ **Built-in CLI**: Generator kode (Controller, Model, Auth) otomatis.
- 🔄 **Live Reload**: Refresh browser otomatis saat ada perubahan kode/template.

## 🚀 Smart Installer (Rekomendasi)

Instal CLI RustBasic secara otomatis dengan satu perintah. Script ini akan menangani konfigurasi path dan dependensi sistem secara cerdas.

### 🍎 macOS & 🐧 Linux
```bash
bash -c "$(curl -fsSL https://raw.githubusercontent.com/herisvan321/rustbasic-cli-install/main/rustbasic.sh)"
```

### 🪟 Windows (PowerShell)
```powershell
powershell -ExecutionPolicy Bypass -Command "iwr -useb https://raw.githubusercontent.com/herisvan321/rustbasic-cli-install/main/rustbasic.ps1 | iex"
```

> [!TIP]
> Script installer juga dapat digunakan untuk **Reinstall** (update) atau **Uninstall** RustBasic secara bersih dari sistem Anda.

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
Ditenagai oleh [rustbasic-core](https://github.com/herisvan321/rustbasic-core) & [rustbasic-cli-install](https://github.com/herisvan321/rustbasic-cli-install).
Dipersembahkan oleh Tim RustBasic.
