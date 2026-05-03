# Middleware

Middleware adalah lapisan yang memproses request sebelum sampai ke controller. Disimpan di `src/app/http/middleware/`.

## Menggunakan Middleware
Daftarkan middleware di `src/routes/mod.rs` atau `web.rs`:
```rust
Router::new()
    .route("/dashboard", get(handler))
    .layer(axum::middleware::from_fn(auth_middleware))
```

## Membuat Middleware
Gunakan CLI untuk membuat file middleware baru secara otomatis:
```bash
cargo rustbasic make:middleware Name
```

---

# Rate Limiting & Security

Aplikasi menyertakan perlindungan otomatis terhadap DDoS dan Brute Force menggunakan `tower-governor`.

## Konfigurasi
Atur batas request di file `.env`:
```env
APP_LIMIT_REQUEST=20
```
Ini akan membatasi setiap IP maksimal 20 request per detik.

## Tampilan Error
Jika batas terlampaui, aplikasi akan otomatis menampilkan halaman error **429 Too Many Requests** dengan sisa waktu tunggu yang dinamis.

---

# CSRF Protection

RustBasic secara otomatis menyertakan perlindungan CSRF untuk request yang mengubah data (POST, PUT, DELETE).

## Cara Kerja
1. Middleware CSRF akan mengecek token di cookie dan form.
2. Jika tidak cocok, request akan ditolak (403 Forbidden).

## Penggunaan di Template
Sertakan token di setiap form:
```html
<form method="POST">
    <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
    <!-- input lainnya -->
</form>
```

---

# HTTP Requests

Objek `Request` di RustBasic membungkus fungsionalitas umum seperti Session dan Auth.

## Mengambil Data Form
```rust
pub async fn store(Form(input): Form<MyStruct>) -> impl IntoResponse {
    // ...
}
```

## Akses Session dari Request
```rust
let user_id = req.session.get::<i32>("user_id");
```

---

# HTTP Responses

RustBasic mendukung berbagai tipe response:

## Render View (HTML)
```rust
view(&req, "file.html", context! { ... })
```

## JSON Response
```rust
axum::Json(serde_json::json!({ "status": "ok" }))
```

## Redirect
```rust
axum::response::Redirect::to("/login")
```
