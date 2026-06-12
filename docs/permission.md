# 🔐 Manajemen Hak Akses & Peran (rustbasic-permission)

Paket `rustbasic-permission` menyediakan sistem otorisasi dinamis berbasis **Role-Based Access Control (RBAC)** untuk framework RustBasic. Memungkinkan Anda membatasi akses endpoint rute atau tampilan SPA berdasarkan Peran (*Roles*) dan Hak Izin (*Permissions*).

---

## 🗄️ 1. Skema Database (Inisialisasi Otomatis)

Saat fungsi inisialisasi dijalankan di *setup/boot* aplikasi Anda, 5 tabel relasi RBAC akan otomatis dibuat di database:

```rust
use rustbasic_permission::init_permission_tables;

// Menjalankan migrasi skema tabel RBAC jika belum ada
init_permission_tables(&db_pool).await.unwrap();
```

### Tabel yang Terbentuk:
1.  **`roles`**: Menyimpan data peran pengenal (cth: `"admin"`, `"editor"`, `"user"`).
2.  **`permissions`**: Menyimpan data hak akses (cth: `"post.create"`, `"user.delete"`).
3.  **`model_has_roles`**: Menghubungkan peran ke entitas model dengan referensi ID dinamis.
4.  **`model_has_permissions`**: Menghubungkan hak akses langsung ke model (*direct permissions*).
5.  **`role_has_permissions`**: Menghubungkan hak akses ke peran.

---

## 🛠️ 2. Penggunaan `PermissionService`

`PermissionService` memiliki serangkaian fungsi asinkron statis untuk mengelola hubungan otorisasi.

### A. Membuat Role & Permission Baru
```rust
use rustbasic_permission::PermissionService;

// Membuat Role baru (jika belum ada)
let admin_role = PermissionService::create_role(&db_pool, "admin", None).await.unwrap();

// Membuat Permission baru
let create_post_perm = PermissionService::create_permission(&db_pool, "post.create", None).await.unwrap();
```

### B. Menghubungkan Permission ke Role
Menetapkan hak akses tertentu ke dalam sebuah peran kelompok pengguna.
```rust
// Memberikan izin "post.create" ke peran "admin"
PermissionService::give_permission_to_role(&db_pool, "admin", "post.create").await.unwrap();

// Mencabut izin dari peran
PermissionService::revoke_permission_from_role(&db_pool, "admin", "post.create").await.unwrap();
```

### C. Menugaskan Role ke Pengguna (Model)
Peran dan izin dihubungkan menggunakan nama tipe model (*model type*) dan ID model untuk fleksibilitas entitas apa pun.
```rust
let model_type = "User";
let user_id = 1;

// Memberikan peran "admin" ke pengguna ID 1
PermissionService::assign_role(&db_pool, model_type, user_id, "admin").await.unwrap();

// Memeriksa apakah pengguna memiliki peran tersebut
let is_admin = PermissionService::has_role(&db_pool, model_type, user_id, "admin").await.unwrap();

// Mencabut peran "admin" dari pengguna
PermissionService::remove_role(&db_pool, model_type, user_id, "admin").await.unwrap();
```

### D. Memberikan Permission Langsung ke Pengguna
Anda juga dapat melewatkan peran dan memberikan hak izin khusus langsung ke target pengguna tunggal.
```rust
// Memberikan izin "post.create" langsung ke user ID 1
PermissionService::give_permission_to(&db_pool, "User", user_id, "post.create").await.unwrap();

// Mencabut izin langsung
PermissionService::revoke_permission_to(&db_pool, "User", user_id, "post.create").await.unwrap();
```

---

## 🛡️ 3. Memverifikasi Otorisasi di Controller

Gunakan fungsi `has_permission_to` untuk memeriksa apakah pengguna aktif berhak menjalankan aksi tertentu. Fungsi ini secara cerdas akan mengecek apakah ada kecocokan izin langsung (*direct*) atau izin warisan lewat Peran yang disandang pengguna.

```rust
use rustbasic_core::{Response, ResponseHelper};
use rustbasic_permission::PermissionService;

pub async fn delete_post_controller(db: AnyPool, current_user_id: i32) -> Response {
    // Memeriksa izin "post.delete"
    let allowed = PermissionService::has_permission_to(&db, "User", current_user_id, "post.delete")
        .await
        .unwrap_or(false);

    if !allowed {
        return ResponseHelper::error("Anda tidak memiliki akses untuk menghapus artikel.");
    }

    // Lanjutkan aksi penghapusan...
    ResponseHelper::json(json!({ "status": "success" }))
}
```

---

## 👑 4. Aturan Khusus Peran Super Admin

Paket ini memiliki kebijakan bawaan (*built-in bypass*) untuk pengguna dengan peran **`"admin"`**:
*   Jika metode `get_all_permissions_for` dipanggil untuk model yang memiliki peran `"admin"`, sistem akan **secara otomatis mengembalikan semua daftar permission** yang terdaftar di dalam database.
*   Hal ini mempermudah manajemen peran global, sehingga akun administrator utama otomatis mendapatkan seluruh kekuasaan rute tanpa perlu didaftarkan satu per satu secara manual.
