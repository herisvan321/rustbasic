# 🛠️ Panduan Perintah CLI RustBasic

## 📝 Kata Pengantar
Selamat datang di panduan resmi **RustBasic CLI**. Dokumentasi ini dirancang khusus untuk memandu Anda menguasai baris perintah (Command Line) bawaan framework. 

Dengan menguasai perkakas CLI ini, Anda dapat mengotomatisasi pembuatan struktur folder, file kode boilerplate (generator), pengelolaan migrasi database, hingga audit rute secara instan di terminal Anda.

---

## 🛠️ Script Contoh

### A. Membuat Proyek Baru (Global CLI)
```bash
# Membuat kerangka proyek SPA baru
rustbasic new my-new-app
```

### B. Membuat Controller Baru di Terminal (Local CLI)
```bash
# Membuat file controller kosong di src/app/http/controllers/
rustbasic make:controller ArticleController
```

### C. Menampilkan Daftar Rute yang Aktif
```bash
# Mencetak daftar rute, HTTP method, dan middleware yang melindunginya
rustbasic route:list
```

### D. Memulai Proses Build Produksi
```bash
# Mengompilasi frontend React + Vite, mengoptimalkan biner Rust, dan menyusun file rilis di folder deploy/
rustbasic build
```

---

## 🔄 Perbedaan CLI Global vs CLI Lokal

Dalam ekosistem RustBasic, perintah CLI dibagi menjadi dua wilayah kerja demi fleksibilitas dan sinkronisasi versi library:

### 1. Perintah CLI Global (`rustbasic-cli`)
- **Metode Instalasi**: Diinstal sekali secara global di sistem operasi Anda menggunakan `cargo install rustbasic-cli`.
- **Tugas Utama**: Membuat proyek baru (`rustbasic new <nama>`). CLI global mengunduh boilerplate template resmi teranyar dari repositori Github RustBasic dan menyiapkannya di komputer lokal Anda.

### 2. Perintah CLI Lokal (`rustbasic <cmd>`)
- **Metode Eksekusi**: Dijalankan langsung dari direktori root proyek lokal Anda.
- **Tugas Utama**: Mengelola logika internal proyek seperti pembuatan model, migrasi database, seeding, routing audit, dan kompilasi produksi.
- **Delegasi Kerja**: Saat Anda menjalankan perintah di root proyek, CLI global secara otomatis mendeteksi keberadaan file biner lokal proyek dan mendelegasikan tugas ke modul internal `src/config/cli.rs` agar selalu selaras dengan versi dependensi lokal proyek Anda.

---

## 📊 Tabel Komparasi CLI Global vs CLI Lokal

| Parameter Evaluasi | Perintah CLI Global (`rustbasic-cli`) | Perintah CLI Lokal (`rustbasic <cmd>`) |
| :--- | :--- | :--- |
| **Metode Instalasi** | Terinstal secara global di OS (`cargo install`). | Dijalankan langsung di folder root proyek lokal Anda. |
| **Tugas Utama** | Membuat kerangka folder proyek baru (`new`). | Menjalankan migrasi, membuat model, controller, & seeder. |
| **Koneksi Database** | Tidak terhubung ke database mana pun. | Terkoneksi langsung ke database pool berdasarkan `.env` lokal. |
| **Sinkronisasi Versi** | Independen dari versi library proyek. | Otomatis selaras dengan versi dependensi lokal proyek Anda. |
| **Sumber Eksekusi** | File biner global system. | Didelegasikan ke file biner proyek (`src/config/cli.rs`). |

---

## 📚 Referensi Perintah CLI Lengkap

Berikut adalah daftar perintah baris teks lengkap yang didukung oleh RustBasic:

### 1. Perintah Generator Kode (`make:*`)
Digunakan untuk membuat file boilerplate komponen baru:

- **`make:controller <Name>`**:
  Membuat berkas controller baru di `src/app/http/controllers/<name>_controller.rs` dan mendaftarkan modulnya di `mod.rs`.
- **`make:model <Name> [-m]`**:
  Membuat berkas model di `src/app/models/<name>.rs`. Menyertakan flag `-m` atau `--migration` akan otomatis membuat file migrasi tabel untuk model tersebut di `database/migrations/`.
- **`make:middleware <Name>`**:
  Membuat berkas middleware baru di `src/app/http/middleware/<name>.rs`.
- **`make:observer <Name> --model=<ModelName>`**:
  Membuat berkas observer di `src/app/observers/<name>_observer.rs` untuk mendengarkan perubahan data model.
- **`make:service <Name>`**:
  Membuat berkas logika bisnis modular (Service Layer) di `src/app/services/<name>_service.rs`.
- **`make:seeder <Name>`**:
  Membuat berkas pengisi data awal baru di `database/seeders/<name>_seeder.rs`.
- **`make:migration <Name>`**:
  Membuat file migrasi blueprint database baru dengan nama acak berstempel waktu di `database/migrations/`.

### 2. Perintah Database (`migrate:*` & `db:*`)
Digunakan untuk mengelola skema dan data database:

- **`migrate`**:
  Menjalankan seluruh berkas migrasi baru yang belum dieksekusi di database.
- **`migrate:refresh`**:
  Melakukan rollback seluruh migrasi (drop table) dan menjalankan ulang seluruh migrasi dari awal (database reset).
- **`migrate:rollback`** atau **`migrate:back`**:
  Membatalkan satu langkah migrasi terakhir yang dijalankan.
- **`db:seed`**:
  Menjalankan fungsi `run` dari seluruh kelas seeder yang terdaftar untuk mengisi data dummy awal.

### 3. Perintah Utilitas Aplikasi
- **`serve`**:
  Menjalankan server pengembangan RustBasic beserta hot-reload frontend.
- **`key:generate`**:
  Menghasilkan kunci enkripsi acak base64 sepanjang 32 karakter dan menyimpannya secara otomatis di variabel `APP_KEY` pada berkas `.env`.
- **`storage:link`**:
  Membuat tautan pintasan (symbolic link) dari folder privat `storage/app/public` ke folder publik web `public/storage` agar dapat diakses dari browser klien.
- **`route:list`**:
  Menampilkan tabel visual seluruh rute, HTTP methods, alamat handler controller, dan tumpukan middleware yang memproteksinya.
- **`build`**:
  Mengompilasi program untuk rilis produksi (bundling Vite, cargo release build, stripping debug info) dan menyatukannya ke folder `deploy/`.

---

## 🏁 Penutup
Perkakas CLI pada RustBasic dirancang untuk menghilangkan kejenuhan menulis kode boilerplate secara manual. Dengan memanfaatkan pintasan generator ini, Anda dapat fokus sepenuhnya pada pembangunan logika bisnis utama aplikasi secara cepat dan konsisten.
