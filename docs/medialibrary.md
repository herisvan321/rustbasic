# 📂 Manajemen Media Library (rustbasic-medialibrary)

Paket `rustbasic-medialibrary` menyediakan pengelola aset multimedia (*media library manager*) yang tangguh. Mampu menangani upload berkas, pemrosesan gambar otomatis, pembuatan *thumbnail*, dan mendukung integrasi penyimpanan lokal (*disk*) maupun layanan awan S3/R2.

---

## 💾 1. Backend Penyimpanan (Storage Backend)

Tentukan tempat berkas fisik Anda akan disimpan. Paket ini menyediakan dua backend default:

### A. Local Storage Backend (Penyimpanan Disk Server)
Menyimpan berkas langsung pada folder lokal di dalam server web.

```rust
use rustbasic_medialibrary::LocalStorageBackend;
use std::path::PathBuf;

let local_backend = LocalStorageBackend::new("public/uploads");
```

### B. Cloud S3 Storage Backend (Mock/Real Cloud Integration)
Menghubungkan aplikasi ke layanan Object Storage seperti AWS S3, Cloudflare R2, MinIO, atau DigitalOcean Spaces.

```rust
use rustbasic_medialibrary::S3StorageBackend;

let s3_backend = S3StorageBackend::new("nama-bucket-saya");
```

---

## ⚙️ 2. Konfigurasi `MediaLibrary` Manager

Inisialisasi manajer media library dengan backend aktif dan tentukan validasi batasan keamanan (MIME type dan ukuran file).

```rust
use rustbasic_medialibrary::MediaLibrary;
use std::sync::Arc;

let library = MediaLibrary::new(Arc::new(local_backend))
    .max_size(10 * 1024 * 1024)  // Batasi upload maksimal 10 Megabytes
    .allow_mime("image/png")     // Hanya izinkan format PNG
    .allow_mime("image/jpeg")    // Dan JPEG
    .allow_mime("application/pdf"); // Serta berkas PDF
```

---

## 📤 3. Mengunggah Berkas Media (`upload`)

Gunakan fungsi `upload` untuk memproses data byte mentah. Fungsi ini akan menghasilkan nama berkas acak yang di-hash dengan SHA-256 agar tidak menimpa file lain dengan nama sama (*no duplicate collisions*).

```rust
let filename = "pasfoto.jpg";
let mime_type = "image/jpeg";
let file_data: &[u8] = ...; // Byte file yang diterima dari HTTP Request

match library.upload(filename, mime_type, file_data) {
    Ok(media) => {
        println!("Berkas Berhasil Diunggah!");
        println!("Nama Baru di Disk: {}", media.filename); // Cth: c82da940...jpg
        println!("MIME Type: {}", media.mime_type);
        println!("Ukuran: {} bytes", media.size);
        println!("Kategori: {}", media.category);         // Cth: "image", "document"
        println!("URL Publik: {}", media.url);             // Cth: "/uploads/c82da940...jpg"
    }
    Err(err) => eprintln!("Upload Gagal: {}", err),
}
```

---

## 🗑️ 4. Menghapus Berkas Media (`delete`)

Menghapus file fisik dari media storage backend aktif.

```rust
let file_to_delete: MediaFile = ...; // Metadata file yang diperoleh saat upload/DB

match library.delete(&file_to_delete) {
    Ok(_) => println!("Berkas berhasil dihapus secara fisik."),
    Err(e) => eprintln!("Gagal menghapus berkas: {}", e),
}
```

---

## 🖼️ 5. Pemrosesan Gambar (Thumbnails & WebP)

Layanan ini terintegrasi langsung dengan konversi otomatis untuk memotong atau menduplikasi file gambar menjadi aset WebP hemat memori.

### A. Membuat Crop Thumbnail Persegi (`create_thumbnail`)
Secara otomatis memotong tengah gambar asli dan mengubah ukurannya menjadi dimensi persegi sesuai target (misal: untuk avatar pengguna 150x150).

```rust
// Membuat thumbnail 150x150 piksel dari gambar asli yang diunggah
match library.create_thumbnail(&original_media, 150, 150) {
    Ok(thumb) => {
        println!("Thumbnail URL: {}", thumb.url); // Cth: "/uploads/xxxx_thumb_150x150.png"
    }
    Err(e) => eprintln!("Gagal membuat thumbnail: {}", e),
}
```

### B. Transcoding ke WebP (`convert_to_webp`)
Mengonversi gambar bertipe PNG/JPEG di server menjadi format WebP modern ultra-ringan secara asinkron.

```rust
match library.convert_to_webp(&original_media) {
    Ok(webp_file) => {
        println!("WebP URL: {}", webp_file.url); // Cth: "/uploads/xxxx.webp"
    }
    Err(e) => eprintln!("Gagal transcode gambar ke WebP: {}", e),
}
```

---

## 🏷️ 6. Klasifikasi Kategori Otomatis

Manajer media library secara otomatis mengklasifikasikan kategori file berdasarkan MIME type mereka saat diunggah:
*   `image`: Jika MIME dimulai dengan `image/` (PNG, JPEG, GIF, WebP).
*   `video`: Jika MIME dimulai dengan `video/` (MP4, WEBM).
*   `audio`: Jika MIME dimulai dengan `audio/` (MP3, WAV).
*   `document`: PDF, TXT, DOCX, XLSX, PPTX.
*   `archive`: ZIP, TAR, RAR, 7Z.
*   `other`: Jenis file lainnya.
