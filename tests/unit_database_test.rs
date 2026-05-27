/* ---------------------------------------------------------
 * 🧪 UNIT TEST: Database (tests/unit_database_test.rs)
 * --------------------------------------------------------- */

use rustbasic_core::{Config, MigratorTrait};
use rustbasic_core::database::DB;
use rustbasic_core::serde_json::json;
use rustbasic::app::models::users;

#[tokio::test]
async fn test_database_insert_and_query() {
    // 1. Muat konfigurasi
    let _ = rustbasic_core::dotenvy::dotenv();
    let cfg = Config::load();
    
    // 2. Hubungkan ke database
    let db = rustbasic_core::database::connect(&cfg).await;
    
    // 2.5 Jalankan migrasi secara programatis untuk membuat tabel jika belum ada
    let _ = rustbasic::migrations::Migrator::up(&db, None).await;
    
    // 3. Bersihkan data user test sebelumnya (jika ada) untuk isolasi
    let _ = DB::table(&db, "users")
        .where_("email", "john_test@example.com")
        .delete()
        .await;

    // 4. Arrange: Siapkan data user baru
    let user_data = json!({
        "name": "John Test",
        "email": "john_test@example.com",
        "password": "hashedpassword123"
    });

    // 5. Act: Insert user baru menggunakan model::create
    let created_user = users::Model::create(&db, user_data).await
        .expect("Gagal membuat user baru di database");

    // 6. Assert: Periksa nilai kembalian model::create
    assert_eq!(created_user.name, "John Test");
    assert_eq!(created_user.email, "john_test@example.com");

    // 7. Act: Query kembali dari database untuk memverifikasi persistensi data
    let retrieved_user = DB::table(&db, "users")
        .where_("email", "john_test@example.com")
        .first::<users::Model>()
        .await
        .expect("Query gagal")
        .expect("User tidak ditemukan di database");

    // 6.5. Verify new Eloquent ORM helper methods
    // Test count()
    let count = users::Model::count(&db).await.expect("Failed to count users");
    assert!(count >= 1);

    // Test first()
    let first_user = users::Model::first(&db).await.expect("Failed to get first user")
        .expect("No users found");
    assert!(!first_user.email.is_empty());

    // Test all()
    let all_users = users::Model::all(&db).await.expect("Failed to get all users");
    assert!(!all_users.is_empty());

    // Test find() using the ID retrieved from database
    let found_user = users::Model::find(&db, retrieved_user.id).await.expect("Failed to find user")
        .expect("User not found by ID");
    assert_eq!(found_user.email, "john_test@example.com");

    // Test query()
    let queried_user = users::Model::query(&db)
        .where_("email", "john_test@example.com")
        .first::<users::Model>()
        .await
        .expect("Query failed")
        .expect("User not found by query");
    assert_eq!(queried_user.name, "John Test");

    // Test Accessor (name_and_email)
    assert_eq!(retrieved_user.name_and_email(), "John Test (john_test@example.com)");

    // Test Mutator (set_password)
    let mut temp_user = retrieved_user.clone();
    temp_user.set_password("newpassword123");
    assert!(rustbasic_core::bcrypt::verify("newpassword123", &temp_user.password).unwrap());

    // Test API Resource (to_resource)
    let resource = retrieved_user.to_resource();
    assert_eq!(resource["name"], "John Test");
    assert_eq!(resource["email"], "john_test@example.com");
    assert!(resource.get("password").is_none()); // Password should be hidden in resource

    // 8. Assert: Pastikan data yang diambil dari DB sama dengan yang di-insert
    assert_eq!(retrieved_user.name, "John Test");
    assert_eq!(retrieved_user.email, "john_test@example.com");

    // 9. Clean up menggunakan users::Model::destroy setelah test selesai agar tidak mengotori DB
    let _ = users::Model::destroy(&db, retrieved_user.id).await;
}

