# 🦀 RustBasic Starter Kit

**RustBasic** adalah framework monolith modern untuk bahasa pemrograman Rust, yang dirancang untuk kecepatan pengembangan maksimal (seperti Laravel) namun dengan performa dan keamanan Rust.

## 🚀 Fitur Unggulan
- ⚡ **Axum Powered**: Backend super cepat dan efisien.
- 🗄️ **Sea-ORM**: Manajemen database async yang mudah dan aman.
- 🎨 **Minijinja**: Engine template HTML standar yang familiar.
- 🛡️ **Built-in CLI**: Generator kode (Controller, Model, Auth) otomatis.
- 🔄 **Live Reload**: Refresh browser otomatis saat ada perubahan kode/template.

## 📦 Persiapan Awal
1. **Clone & Install**:
   ```bash
   git clone https://github.com/herisvan321/rustbasic-starter my-app
   cd my-app
   cp .env.example .env
   ```
2. **Setup Database**:
   Edit `.env` dan jalankan migrasi:
   ```bash
   cargo rustbasic migrate
   ```

## 🛠️ Penggunaan CLI (`cargo rustbasic`)
Framework ini dilengkapi dengan wrapper script `cargo rustbasic` yang memudahkan Anda:
- **Jalankan Server**: `cargo rustbasic serve`
- **Buat Controller**: `cargo rustbasic make:controller Nama`
- **Scaffold Auth**: `cargo rustbasic make:auth`
- **Cek Rute**: `cargo rustbasic route:list`

## 📚 Dokumentasi Lengkap
Lihat folder [`docs/`](docs/README.md) untuk panduan mendalam mengenai:
- [Dasar-Dasar & Routing](docs/basics.md)
- [Views & Templates](docs/views.md)
- [Referensi CLI Lengkap](docs/cli.md)

---

Ditenagai oleh [rustbasic-core](https://crates.io/crates/rustbasic-core).
Dipersembahkan oleh Tim RustBasic.
