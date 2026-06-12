# 🖼️ Paket Pengoptimal Gambar WebP (rustbasic-webp)

Paket `rustbasic-webp` menyediakan modul manipulasi gambar canggih berbasis format WebP untuk meningkatkan performa loading aset media aplikasi Anda. Mengurangi ukuran file gambar tanpa mengorbankan kualitas secara signifikan.

---

## 🔍 1. Membaca Informasi Gambar (`get_image_info`)

Anda dapat mengekstrak metadata dari data biner gambar (JPEG, PNG, GIF, BMP, dll.) seperti dimensi lebar, tinggi, format asli, dan ukuran file.

```rust
use rustbasic_webp::get_image_info;

let raw_image_bytes: &[u8] = ...; // Buffer gambar mentah dari upload/file

match get_image_info(raw_image_bytes) {
    Ok(info) => {
        println!("Lebar: {}px", info.width);
        println!("Tinggi: {}px", info.height);
        println!("Format Asli: {}", info.format);
        println!("Ukuran: {} bytes", info.size_bytes);
    }
    Err(err) => eprintln!("Format gambar tidak dikenali: {}", err),
}
```

---

## 🔄 2. Konversi Gambar ke WebP (`convert_to_webp`)

Fungsi utama untuk mengubah gambar apa pun menjadi WebP berkinerja tinggi dengan menentukan skala kualitas 1.0 (lossless/terbaik) hingga 100.0 (biasanya disarankan berkisar 75.0 - 85.0).

```rust
use rustbasic_webp::convert_to_webp;

let quality = 80.0;
match convert_to_webp(raw_image_bytes, quality) {
    Ok(webp_bytes) => {
        // webp_bytes kini berisi biner gambar WebP yang telah dikompres
        std::fs::write("output.webp", webp_bytes).unwrap();
    }
    Err(e) => eprintln!("Gagal mengonversi ke WebP: {}", e),
}
```

---

## 📏 3. Mengubah Ukuran Gambar (*Resizing*)

Paket ini menyediakan helper resizing berkecepatan tinggi dengan tetap menjaga rasio aspek asli (*aspect ratio*) menggunakan filter Lanczos3.

### A. Ubah Ukuran Dua Dimensi Maksimal (`convert_and_resize`)
Mengubah ukuran gambar agar tidak melebihi batasan lebar dan tinggi maksimal secara asimetris.

```rust
use rustbasic_webp::convert_and_resize;

let max_width = 800;
let max_height = 600;
let quality = 85.0;

let resized_bytes = convert_and_resize(raw_image_bytes, max_width, max_height, quality).unwrap();
```

### B. Ubah Lebar Maksimal (`convert_and_resize_width`)
Mengunci lebar maksimal, tinggi akan menyesuaikan secara otomatis mengikuti rasio aspek gambar asli.

```rust
use rustbasic_webp::convert_and_resize_width;

let max_width = 400; // Tinggi otomatis
let quality = 80.0;

let resized_bytes = convert_and_resize_width(raw_image_bytes, max_width, quality).unwrap();
```

### C. Ubah Tinggi Maksimal (`convert_and_resize_height`)
Mengunci tinggi maksimal, lebar akan menyesuaikan secara otomatis mengikuti rasio aspek gambar asli.

```rust
use rustbasic_webp::convert_and_resize_height;

let max_height = 300; // Lebar otomatis
let quality = 80.0;

let resized_bytes = convert_and_resize_height(raw_image_bytes, max_height, quality).unwrap();
```

---

## 📂 4. Konversi Berkas Lokal Langsung (`convert_file_to_webp`)

Helper utilitas untuk membaca berkas dari disk lokal, melakukan kompresi WebP, dan langsung menyimpannya kembali ke jalur berkas baru.

```rust
use rustbasic_webp::convert_file_to_webp;

let input_path = "storage/original/photo.jpg";
let output_path = "public/uploads/photo.webp";
let quality = 80.0;

match convert_file_to_webp(input_path, output_path, quality) {
    Ok(_) => println!("Berkas gambar berhasil dikonversi ke WebP di: {}", output_path),
    Err(e) => eprintln!("Gagal memproses berkas gambar: {}", e),
}
```
