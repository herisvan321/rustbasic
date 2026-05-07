# Validation

Validasi di RustBasic menggunakan crate `validator`. 

## Mendefinisikan Struct Validasi
```rust
use validator::Validate;

#[derive(Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,
    
    #[validate(email(message = "Format email salah"))]
    pub email: String,
}
```

## Menjalankan Validasi
```rust
if let Err(e) = input.validate() {
    // Tangani error validasi
}
```

---

# HTTP Session

Session dikelola secara otomatis menggunakan `axum_session`. 

## Menyimpan Data
```rust
req.session().set("key", "value");
```

## Mengambil Data
```rust
let value: Option<String> = req.session().get("key");
```

---

# Logging

RustBasic menggunakan `tracing` untuk logging.

## Penggunaan
```rust
use tracing::{info, warn, error};

info!("Halaman dashboard diakses");
error!("Gagal menyimpan data: {}", err);
```

---

# Error Handling

Error ditangani di `src/app/http/controllers/error_controller.rs` dan ditampilkan melalui view yang cantik.

## Custom 404
Secara default, route yang tidak ditemukan akan dilempar ke `ErrorController::not_found`.

---

# URL Generation

Saat ini URL dibentuk secara manual menggunakan helper di template:
```html
<a href="/dashboard">Dashboard</a>
```
Untuk asset statis:
```html
<link rel="stylesheet" href="/css/app.css">
```
