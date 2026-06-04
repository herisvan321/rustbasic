# ⚙️ Panduan Topik Lanjutan (Validation, Session, & Logs)

## 📝 Kata Pengantar
Selamat datang di panduan **Topik Lanjutan RustBasic**. Dokumentasi ini dirancang khusus untuk memandu Anda menerapkan sistem validasi input data yang kuat di sisi server, mengelola siklus stateful menggunakan sesi terenkripsi, merekam jejak aktivitas aplikasi secara harian (rolling logs), dan menangani cache-busting aset web secara otomatis demi performa optimal di tingkat produksi.

Dengan menguasai topik-topik lanjutan ini, Anda dapat membangun aplikasi web enterprise yang aman, dapat ditelusuri (*traceable*), dan tangguh menghadapi trafik skala besar.

---

## 🛠️ Script Contoh

### A. Validasi Struct Input Data (`src/app/http/controllers/post_controller.rs`)
Menggunakan macro `Validate` dari crate `validator` untuk menyaring dan memvalidasi tipe data input dari request JSON secara deklaratif:
```rust
use rustbasic_core::serde::Deserialize;
use rustbasic_core::validator::Validate;

#[derive(Validate, Deserialize)]
pub struct CreatePostRequest {
    #[validate(length(min = 5, message = "Judul minimal harus berisi 5 karakter"))]
    pub title: String,
    
    #[validate(email(message = "Format alamat email salah"))]
    pub email: String,

    #[validate(range(min = 1, max = 100, message = "Jumlah harus di antara 1 sampai 100"))]
    pub quantity: i32,
}
```

### B. Membaca & Menulis Sesi HTTP Terenkripsi
Sesi di RustBasic secara otomatis dienkripsi menggunakan algoritma enkripsi aman AES-GCM berdasarkan kunci rahasia `APP_KEY` yang tersimpan di file `.env`.
```rust
use rustbasic_core::requests::Request;

// 1. Menyimpan data kompleks ke Sesi
req.session.set("user_role", "admin".to_string());
req.session.set("login_attempts", 3);

// 2. Mengambil data dari Sesi (type safe)
let role: Option<String> = req.session.get("user_role");
let attempts: Option<i32> = req.session.get("login_attempts");
```

### C. Inisialisasi & Penggunaan Tracing Log
Inisialisasi log di `main.rs` dan cara merekam jejak aktivitas di backend menggunakan library `tracing`:
```rust
// 1. Inisialisasi logger di main.rs
let _guard = rustbasic_core::logger::init(); // Mengembalikan guard untuk flush log

// 2. Merekam log di controller atau modul model
use rustbasic_core::tracing::{info, warn, error, debug};

pub fn process_order(order_id: i32, amount: f64) {
    debug!("Mulai proses order dengan ID: {}", order_id);
    
    if amount <= 0.0 {
        warn!("Mendeteksi nilai order mencurigakan (nol/negatif): {}", amount);
    }

    if let Err(e) = save_order_to_db(order_id) {
        error!("Gagal menyimpan transaksi order {} ke database: {}", order_id, e);
    } else {
        info!("Transaksi order {} senilai Rp{} berhasil diproses.", order_id, amount);
    }
}
```

---

## 🚀 Pembahasan Mendalam Sistem Keamanan & Log

### 1. Validasi Input Data & Pemetaan ke Frontend SPA
Sistem validasi menggunakan framework `validator` melakukan pencocokan rules sebelum logika database dijalankan. Jika validasi gagal di controller, handler secara otomatis mengembalikan response redirect 303 kembali ke halaman asal beserta data *error mapping* yang tersimpan di session flash.
Di halaman React SPA, data errors ini secara otomatis dipetakan ke dalam objek `errors` milik hook `useForm`:
```javascript
const { errors } = useForm();
// errors.title -> "Judul minimal harus berisi 5 karakter"
// errors.email -> "Format alamat email salah"
```

### 2. Enkripsi Sesi (Encrypted Sessions)
Sesi pengguna diidentifikasi menggunakan `session_id` unik yang dikirimkan melalui cookie browser yang ditandai dengan flag `HttpOnly` dan `Secure`. 
- **Enkripsi Kunci**: Seluruh payload session dienkripsi di sisi server.
- **Keamanan APP_KEY**: Keamanan enkripsi ini sepenuhnya bertumpu pada `APP_KEY` di file `.env`. Jika file `.env` tidak memiliki `APP_KEY`, server akan menolak berjalan demi mencegah eksploitasi data sesi. Anda dapat men-generate key baru via CLI:
  ```bash
  rustbasic key:generate
  ```

### 3. Log Harian Otomatis (Daily Rolling Logs)
Sistem logger RustBasic secara dinamis mengarahkan output log:
- **Terminal (Stdout)**: Format log berwarna (*colored logs*) yang interaktif untuk mempercepat debugging lokal.
- **File Disk (`storage/logs/`)**: Menyimpan log ke berkas fisik dengan rotasi harian otomatis. Log hari ini akan disimpan di `app.log`. Pada tengah malam, log tersebut akan diarsipkan menjadi `app.log.YYYY-MM-DD` dan file `app.log` baru akan dibuat. Ini mencegah file log tunggal membengkak dan menghabiskan ruang penyimpanan server.

### 4. Cache-Busting Aset Produksi
Ketika Anda membangun aplikasi untuk produksi (`rustbasic build`), compiler menambahkan hash konten unik di nama berkas (misal: `main-D3e4f2.js`). 
Ketika browser klien mengakses aplikasi web Anda:
- Server membaca hash nama berkas terbaru dari `manifest.json`.
- Browser dipaksa mengunduh berkas JS/CSS terbaru dari server alih-alih menggunakan cache lama di browser lokal mereka. Ini menjamin pengguna akhir selalu mendapatkan pembaruan antarmuka (UI) terbaru tanpa perlu menekan Ctrl+F5 secara manual.

---

## 🔄 Perbandingan Pemakaian (Tingkat Log/Log Levels)

Berikut adalah perbandingan tingkatan log yang digunakan dalam proses tracing aplikasi:

| Tingkat Log | Kapan Harus Digunakan | Contoh Kasus Penggunaan |
| :--- | :--- | :--- |
| **TRACE** | Informasi sangat detail tingkat rendah. | Perekaman aliran bytes data masuk/keluar di jaringan. |
| **DEBUG** | Detail alur teknis untuk developer. | Mencetak isi parameter internal variabel. |
| **INFO** | Catatan peristiwa penting yang sukses. | Server berhasil berjalan, user login, pembayaran terkonfirmasi. |
| **WARN** | Peringatan masalah potensial. | Percobaan login gagal berulang kali, performa database menurun. |
| **ERROR** | Gangguan fatal yang menghentikan proses. | Koneksi database terputus, API gateway down, gagal menulis file. |

---

## 📊 Tabel Ringkasan Proteksi Lanjutan

Berikut adalah ringkasan mekanisme pengamanan dan optimasi tingkat lanjut pada RustBasic:

| Nama Sistem | Cara Kerja Sistem | Deskripsi Manfaat |
| :--- | :--- | :--- |
| **Daily Rolling Log** | Logs otomatis diarsipkan harian di folder `storage/logs/`. | Mencegah file log membengkak menyumbat kapasitas disk. |
| **Validation Filter** | Menghentikan request di server jika data tidak sesuai aturan. | Menghindari data sampah/injeksi berbahaya masuk ke DB. |
| **Session Encryption** | Payload Sesi dienkripsi kuat menggunakan AES-GCM berbasis `APP_KEY`.| Menjamin data sesi pengguna aman dari manipulasi cookie. |
| **Cache-Busting Aset** | Menambahkan hash konten unik ke nama file JS/CSS terkompilasi. | Memastikan browser user selalu mengunduh visual teranyar. |

---

## 🏁 Penutup
Dengan menerapkan validasi input yang ketat, logging harian terstruktur, dan sesi terenkripsi, aplikasi Anda siap menghadapi beban trafik produksi yang tinggi secara aman, andal, dan mudah diawasi (*observable*).
