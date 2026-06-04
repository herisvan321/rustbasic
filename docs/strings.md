# 🔤 Panduan Pengolahan String & Fluent Stringable

## 📝 Kata Pengantar
Selamat datang di panduan **Pengolahan String & Fluent Stringable**. Dokumentasi ini dirancang khusus untuk membantu Anda memahami manipulasi teks secara aman, efisien, dan modular di dalam ekosistem **RustBasic**. 

RustBasic menyediakan helper static `Str` dan builder `Stringable` yang dibungkus dengan API fluent mirip framework modern, memudahkan Anda mengolah string tanpa pusing memikirkan manajemen memori Rust yang ketat.

---

## 🚀 Memahami Tipe Data String di Rust

Sebelum menggunakan helper string, penting untuk memahami dua tipe data string utama di Rust:
1. **`String`**: Heap-allocated, dinamis, ukurannya bisa bertambah, dan memiliki kepemilikan data penuh (*owned*).
2. **`&str`**: Slicing string penunjuk ke segmen memori tertentu (*borrowed*). Biasanya bersifat read-only dan sangat efisien karena tidak melakukan alokasi memori baru.

Helper string di RustBasic dirancang agar Anda dapat menginput tipe `&str` yang hemat memori, dan secara otomatis memproses alokasi heap baru menjadi `String` secara aman di bawah kap.

> [!TIP]
> **Keamanan Karakter Multi-Byte (Unicode-Safety)**
> Pemotongan string biasa di Rust (slicing byte) dapat memicu kegagalan panik (*panic error*) jika memotong di tengah-tengah karakter multi-byte (seperti huruf non-latin atau Emoji). Seluruh helper string di RustBasic dirancang **Unicode-Safe** karena memproses iterasi berdasarkan karakter Unicode (*chars*), bukan byte mentah.

---

## 🛠️ Daftar Lengkap Method Static Helper (`Str`)

Untuk mulai menggunakannya, impor modul helper ini langsung di berkas controller atau logika program Anda:
```rust
use rustbasic_core::support::Str;
```

### 1. `Str::of(value: &str) -> Stringable`
Membuat objek fluent `Stringable` baru untuk memulai perantaian (chaining) operasi string:
```rust
let fluent = Str::of("Halo Dunia");
```

### 2. `Str::uuid() -> String`
Menghasilkan string UUID (version 4) acak yang valid dan aman:
```rust
let token = Str::uuid(); // Contoh: "d2d2a450-4d56-42d4-a8eb-ccdfa2ef0413"
```

### 3. `Str::random(length: usize) -> String`
Menghasilkan string acak alfanumerik sepanjang karakter yang diinginkan:
```rust
let key = Str::random(16); // Contoh: "aB8f7K2lPn5mQqZs"
```

### 4. `Str::slug(title: &str) -> String`
Mengubah teks menjadi URL-friendly slug (huruf kecil, dipisah oleh tanda hubung, menghapus karakter spesial):
```rust
let slug = Str::slug("Panduan Belajar RustBasic SPA 🚀"); // "panduan-belajar-rustbasic-spa"
```

### 5. `Str::after(subject: &str, search: &str) -> String`
Mengambil porsi string setelah kemunculan pertama karakter pencari:
```rust
let result = Str::after("user-profile-avatar", "profile-"); // "avatar"
```

### 6. `Str::before(subject: &str, search: &str) -> String`
Mengambil porsi string sebelum kemunculan pertama karakter pencari:
```rust
let result = Str::before("user-profile-avatar", "-profile"); // "user"
```

### 7. `Str::between(subject: &str, from: &str, to: &str) -> String`
Mengambil bagian string di antara dua karakter penanda:
```rust
let text = Str::between("hello [world] rust", "[", "]"); // "world"
```

### 8. `Str::contains(haystack: &str, needle: &str) -> bool`
Memeriksa apakah string mengandung kata pencari:
```rust
let has_rust = Str::contains("belajar rustbasic", "rust"); // true
```

### 9. `Str::starts_with(subject: &str, needle: &str) -> bool`
Memeriksa apakah string diawali dengan teks tertentu:
```rust
let match_start = Str::starts_with("prefix_data", "prefix"); // true
```

### 10. `Str::ends_with(subject: &str, needle: &str) -> bool`
Memeriksa apakah string diakhiri dengan teks tertentu:
```rust
let match_end = Str::ends_with("data_suffix", "suffix"); // true
```

### 11. `Str::is_uuid(value: &str) -> bool`
Memeriksa apakah string merupakan format UUID v4 yang valid:
```rust
let is_valid = Str::is_uuid("123e4567-e89b-12d3-a456-426614174000"); // true
```

### 12. `Str::lower(value: &str) -> String`
Mengubah string ke huruf kecil (Unicode-Safe):
```rust
let lower = Str::lower("RUSTBASIC 🚀"); // "rustbasic 🚀"
```

### 13. `Str::upper(value: &str) -> String`
Mengubah string ke huruf besar (Unicode-Safe):
```rust
let upper = Str::upper("rustbasic"); // "RUSTBASIC"
```

### 14. `Str::limit(value: &str, max: usize, end: &str) -> String`
Membatasi jumlah karakter string dan menambahkan akhiran pemotong:
```rust
let cut = Str::limit("Teks yang sangat panjang sekali", 15, "..."); // "Teks yang sanga..."
```

### 15. `Str::replace(search: &str, replace: &str, subject: &str) -> String`
Menggantikan semua kemunculan kata kunci target dengan kata kunci baru:
```rust
let replaced = Str::replace("pagi", "malam", "selamat pagi"); // "selamat malam"
```

---

## ⛓️ Fluent Stringable (Method Chaining)

Sistem `Stringable` memungkinkan Anda merantai beberapa operasi modifikasi string secara berturut-turut dengan sintaksis yang bersih secara fluent.

Gunakan method static `Str::of()` untuk membuat objek builder `Stringable` dan akhiri dengan `.get()` untuk mendapatkan nilai string akhirnya.

### Contoh Pemakaian:

```rust
use rustbasic_core::support::Str;

let result = Str::of(" Panduan  Belajar   Rust  ")
    .append(" Keren  Sekali ")
    .upper()
    .slug()
    .get();

// Hasil: "panduan-belajar-rust-keren-sekali"
```

### Method Chaining yang Didukung:
Setiap method ini mengembalikan instance `Stringable` kembali sehingga dapat langsung dirantai:

- **`.upper()`**: Mengubah teks ke huruf besar.
- **`.lower()`**: Mengubah teks ke huruf kecil.
- **`.append(val: &str)`**: Menambahkan string di akhir.
- **`.prepend(val: &str)`**: Menambahkan string di awal.
- **`.slug()`**: Mengubah teks menjadi slug URL (URL-friendly).
- **`.after(search: &str)`**: Mengambil teks setelah kemunculan pertama kata pencarian.
- **`.before(search: &str)`**: Mengambil teks sebelum kemunculan pertama kata pencarian.
- **`.between(from: &str, to: &str)`**: Mengambil teks di antara dua tanda batas.
- **`.limit(max: usize, end: &str)`**: Membatasi panjang teks dan menyematkan akhiran pemotong.
- **`.replace(search: &str, replace: &str)`**: Mengganti semua kata pencarian dengan kata baru.
- **`.get() -> String`**: Mengembalikan nilai string akhir (mengonsumsi builder).

---

## 🏁 Penutup
Dengan bantuan helper static `Str` dan kelas fluent `Stringable`, Anda mendapatkan kontrol manipulasi teks tingkat tinggi yang efisien, aman dari crash runtime akibat pemotongan byte multi-byte, dan ditulis secara deklaratif.
