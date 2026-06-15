use rustbasic_core::{Config, MigratorTrait};

pub async fn handle(args: &[String], cfg: &Config) -> bool {
    if args.len() < 2 {
        return false;
    }

    let command = args[1].as_str();

    let is_migration_cmd = command.starts_with("migrate") || command == "db:seed";
    let is_storage_cmd = command == "storage:link";
    let is_server_cmd = command == "server" || command == "serve";
    let is_build_cmd = command == "build";

    if !is_migration_cmd && !is_storage_cmd && !is_server_cmd && !is_build_cmd {
        return false;
    }

    // Skip logger banner output for android logcat/command redirection unless running web server
    if command != "server" && command != "serve" {
        println!("🛠️  RustBasic Local CLI - Command: {}", command);
    }

    if is_build_cmd {
        handle_build(args).await;
        return true;
    }

    if is_storage_cmd {
        handle_storage_link();
        return true;
    }

    if is_server_cmd {
        let run_android = args.iter().any(|arg| arg == "--android");
        let run_desktop = args.iter().any(|arg| arg == "--desktop");
        
        if run_android {
            println!("🚀 Memulai RustBasic Android Wrapper...");
            let mut cmd = std::process::Command::new("bash")
                .arg("./native/run-android.sh")
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("Gagal menjalankan Android runner");
            let status = cmd.wait().expect("Error saat menunggu Android runner");
            std::process::exit(status.code().unwrap_or(0));
        } else if run_desktop {
            println!("🚀 Memulai RustBasic Desktop Wrapper...");
            let mut cmd = std::process::Command::new("bash")
                .arg("./native/run-desktop.sh")
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()
                .expect("Gagal menjalankan Desktop runner");
            let status = cmd.wait().expect("Error saat menunggu Desktop runner");
            std::process::exit(status.code().unwrap_or(0));
        } else {
            return false; // Fall through to standard web server
        }
    }

    // Hubungkan ke database
    let pool = rustbasic_core::database::connect(cfg).await;

    match command {
        "migrate" => {
            println!("🚀 Menjalankan migrasi database...");
            if let Err(e) = crate::migrations::Migrator::up(&pool, None).await {
                println!("❌ Gagal menjalankan migrasi: {}", e);
            } else {
                println!("✅ Migrasi selesai!");
            }
        }
        "migrate:refresh" => {
            println!("🔄 Mereset dan menjalankan ulang migrasi...");
            if let Err(e) = crate::migrations::Migrator::fresh(&pool).await {
                println!("❌ Gagal refresh migrasi: {}", e);
            } else {
                println!("✅ Database berhasil di-refresh!");
            }
        }
        "migrate:back" | "migrate:rollback" => {
            println!("⬅️  Rollback migrasi terakhir...");
            if let Err(e) = crate::migrations::Migrator::down(&pool, None).await {
                println!("❌ Gagal rollback: {}", e);
            } else {
                println!("✅ Rollback berhasil!");
            }
        }
        "db:seed" => {
            crate::app::seeder::run(&pool).await;
        }
        _ => return false,
    }

    true
}

fn handle_storage_link() {
    let target = crate::config::app::STORAGE_TARGET;
    let source = crate::config::app::STORAGE_SOURCE;

    if let Err(e) = std::fs::create_dir_all(source) {
        println!("❌ Gagal membuat direktori storage: {}", e);
        return;
    }

    let path = std::path::Path::new(target);
    if path.exists() || path.is_symlink() {
        println!("ℹ️  Link 'public/storage' sudah ada atau berupa file/folder lain.");
        return;
    }

    println!("🔗 Membuat symbolic link...");

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        if let Err(e) = symlink("../storage/app/public", target) {
            println!("❌ Gagal membuat symlink: {}", e);
        } else {
            println!("✅ Link storage berhasil dibuat! [public/storage -> storage/app/public]");
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::symlink_dir;
        if let Err(e) = symlink_dir("../storage/app/public", target) {
            println!("❌ Gagal membuat symlink: {}", e);
        } else {
            println!("✅ Link storage berhasil dibuat! [public/storage -> storage/app/public]");
        }
    }
}

fn prompt_choice(prompt: &str, min: usize, max: usize) -> usize {
    use std::io::{self, Write};
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if let Ok(choice) = input.trim().parse::<usize>() {
                if choice >= min && choice <= max {
                    return choice;
                }
            }
        }
        println!("⚠️ Pilihan tidak valid, silakan coba lagi.");
    }
}

async fn handle_build(args: &[String]) {
    let mut build_website = args.iter().any(|arg| arg == "--web" || arg == "--website");
    let mut build_desktop = args.iter().any(|arg| arg == "--desktop");
    let mut build_android = args.iter().any(|arg| arg == "--android");
    let mut release_mode = args.iter().any(|arg| arg == "--release" || arg == "-r");
    let mut target_os = String::new();
    let mut target_type = String::new(); // apk / aab
    
    // Parse arguments
    for i in 0..args.len() {
        if args[i] == "--os" && i + 1 < args.len() {
            target_os = args[i+1].clone();
        }
        if args[i] == "--type" && i + 1 < args.len() {
            target_type = args[i+1].to_lowercase();
        }
    }

    if !build_website && !build_desktop && !build_android {
        println!("🛠️  RustBasic Build CLI");
        println!("Pilih platform target untuk di-build:");
        println!("  [1] Website / Backend Server");
        println!("  [2] Desktop Wrapper (Windows, macOS, Linux)");
        println!("  [3] Android Wrapper (APK, AAB)");
        let choice = prompt_choice("👉 Pilih nomor platform (1-3): ", 1, 3);
        match choice {
            1 => build_website = true,
            2 => build_desktop = true,
            3 => build_android = true,
            _ => {}
        }
    }

    if build_website {
        let mut target_triple = "";
        if target_os.is_empty() {
            println!("\n--- Pilih Target OS website ---");
            println!("  [1] Native (Sesuai OS Anda)");
            println!("  [2] Windows x86_64 (x86_64-pc-windows-msvc)");
            println!("  [3] Linux x86_64 GNU (x86_64-unknown-linux-gnu)");
            println!("  [4] Linux x86_64 MUSL (x86_64-unknown-linux-musl)");
            println!("  [5] Linux ARM64 GNU (aarch64-unknown-linux-gnu)");
            println!("  [6] Linux ARM64 MUSL (aarch64-unknown-linux-musl)");
            println!("  [7] macOS ARM64 (aarch64-apple-darwin)");
            println!("  [8] macOS Intel (x86_64-apple-darwin)");
            println!("  [9] Batal");
            let choice = prompt_choice("👉 Pilih nomor target OS (1-9): ", 1, 9);
            if choice == 9 {
                println!("❌ Build dibatalkan.");
                return;
            }
            match choice {
                2 => target_triple = "x86_64-pc-windows-msvc",
                3 => target_triple = "x86_64-unknown-linux-gnu",
                4 => target_triple = "x86_64-unknown-linux-musl",
                5 => target_triple = "aarch64-unknown-linux-gnu",
                6 => target_triple = "aarch64-unknown-linux-musl",
                7 => target_triple = "aarch64-apple-darwin",
                8 => target_triple = "x86_64-apple-darwin",
                _ => {}
            }
        } else {
            match target_os.as_str() {
                "windows" => target_triple = "x86_64-pc-windows-msvc",
                "linux" => target_triple = "x86_64-unknown-linux-gnu",
                "linux-musl" | "linux_musl" => target_triple = "x86_64-unknown-linux-musl",
                "linux-arm64" | "linux_arm64" => target_triple = "aarch64-unknown-linux-gnu",
                "linux-arm64-musl" | "linux_arm64_musl" => target_triple = "aarch64-unknown-linux-musl",
                "macos-arm64" | "macos_arm64" | "macos-silicon" => target_triple = "aarch64-apple-darwin",
                "macos-intel" | "macos_intel" => target_triple = "x86_64-apple-darwin",
                "native" => {}
                _ => {
                    println!("⚠️ Warning: Target OS '{}' tidak dikenal, menggunakan default OS saat ini.", target_os);
                }
            }
        }

        if !args.iter().any(|arg| arg == "--release" || arg == "-r" || arg == "--debug" || arg == "-d") {
            println!("\nPilih Mode Build:");
            println!("  [1] Debug (Cepat compile)");
            println!("  [2] Release (Optimasi penuh)");
            let choice = prompt_choice("👉 Pilih nomor mode (1-2): ", 1, 2);
            if choice == 2 {
                release_mode = true;
            }
        }

        println!("\n🌐 Memulai proses build Website/Server...");
        let mut build_args = vec!["build", "--bin", "rustbasic"];
        if release_mode {
            build_args.push("--release");
        }
        if !target_triple.is_empty() {
            build_args.push("--target");
            build_args.push(target_triple);
            // Ensure target is installed
            let _ = std::process::Command::new("rustup")
                .args(&["target", "add", target_triple])
                .status();
        }

        println!("   Running: cargo {}", build_args.join(" "));
        let mut cmd = std::process::Command::new("cargo")
            .args(&build_args)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Gagal menjalankan cargo build");
        
        let status = cmd.wait().expect("Gagal menunggu proses cargo build");
        if status.success() {
            println!("\n✅ Build Website/Server selesai dengan sukses!");
            let mode_str = if release_mode { "release" } else { "debug" };
            let path_str = if target_triple.is_empty() {
                format!("target/{}/rustbasic", mode_str)
            } else {
                format!("target/{}/{}/rustbasic", target_triple, mode_str)
            };
            println!("📂 Output biner: {}", path_str);
        } else {
            println!("\n❌ Build Website/Server gagal.");
        }

    } else if build_desktop {
        let mut target_triple = "";
        if target_os.is_empty() {
            println!("\nPilih OS Target Desktop:");
            println!("  [1] Current OS (Sistem saat ini)");
            println!("  [2] macOS Intel (x86_64)");
            println!("  [3] macOS Apple Silicon (aarch64)");
            println!("  [4] Windows (x86_64)");
            println!("  [5] Linux (x86_64)");
            let choice = prompt_choice("👉 Pilih nomor target OS (1-5): ", 1, 5);
            match choice {
                2 => target_triple = "x86_64-apple-darwin",
                3 => target_triple = "aarch64-apple-darwin",
                4 => target_triple = "x86_64-pc-windows-msvc",
                5 => target_triple = "x86_64-unknown-linux-gnu",
                _ => {}
            }
        } else {
            match target_os.as_str() {
                "macos-intel" | "macos_intel" => target_triple = "x86_64-apple-darwin",
                "macos-silicon" | "macos_silicon" => target_triple = "aarch64-apple-darwin",
                "macos" => {
                    // detect current macos arch
                    #[cfg(target_arch = "aarch64")]
                    { target_triple = "aarch64-apple-darwin"; }
                    #[cfg(not(target_arch = "aarch64"))]
                    { target_triple = "x86_64-apple-darwin"; }
                }
                "windows" => target_triple = "x86_64-pc-windows-msvc",
                "linux" => target_triple = "x86_64-unknown-linux-gnu",
                _ => {
                    println!("⚠️ Warning: Target OS '{}' tidak dikenal, menggunakan default OS saat ini.", target_os);
                }
            }
        }

        if !args.iter().any(|arg| arg == "--release" || arg == "-r" || arg == "--debug" || arg == "-d") {
            println!("\nPilih Mode Build:");
            println!("  [1] Debug (Cepat compile)");
            println!("  [2] Release (Optimasi penuh)");
            let choice = prompt_choice("👉 Pilih nomor mode (1-2): ", 1, 2);
            if choice == 2 {
                release_mode = true;
            }
        }

        println!("\n🖥️  Memulai proses build Desktop...");
        let mut build_args = vec!["build", "--manifest-path", "native/desktop/Cargo.toml"];
        if release_mode {
            build_args.push("--release");
        }
        if !target_triple.is_empty() {
            build_args.push("--target");
            build_args.push(target_triple);
            // Ensure target is installed
            let _ = std::process::Command::new("rustup")
                .args(&["target", "add", target_triple])
                .status();
        }

        println!("   Running: cargo {}", build_args.join(" "));
        let mut cmd = std::process::Command::new("cargo")
            .args(&build_args)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Gagal menjalankan cargo build");
        
        let status = cmd.wait().expect("Gagal menunggu proses cargo build");
        if status.success() {
            println!("\n✅ Build Desktop selesai dengan sukses!");
            let mode_str = if release_mode { "release" } else { "debug" };
            let path_str = if target_triple.is_empty() {
                format!("native/desktop/target/{}/rustbasic-native-desktop", mode_str)
            } else {
                format!("native/desktop/target/{}/{}/rustbasic-native-desktop", target_triple, mode_str)
            };
            println!("📂 Output biner: {}", path_str);
        } else {
            println!("\n❌ Build Desktop gagal.");
        }

    } else if build_android {
        let mut is_aab = false;
        if target_type.is_empty() {
            println!("\nPilih Format Output Android:");
            println!("  [1] APK (Android Package - Siap install)");
            println!("  [2] AAB (Android App Bundle - Siap Google Play)");
            let choice = prompt_choice("👉 Pilih format (1-2): ", 1, 2);
            if choice == 2 {
                is_aab = true;
            }
        } else {
            is_aab = target_type == "aab";
        }

        if !args.iter().any(|arg| arg == "--release" || arg == "-r" || arg == "--debug" || arg == "-d") {
            println!("\nPilih Mode Build:");
            println!("  [1] Debug");
            println!("  [2] Release (Produksi)");
            let choice = prompt_choice("👉 Pilih nomor mode (1-2): ", 1, 2);
            if choice == 2 {
                release_mode = true;
            }
        }

        println!("\n🔨 Membangun JNI library untuk Android...");
        let mut jni_cmd = std::process::Command::new("bash")
            .arg("./native/build-android.sh")
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Gagal menjalankan build-android.sh");
        
        let jni_status = jni_cmd.wait().expect("Gagal menunggu build-android.sh");
        if !jni_status.success() {
            println!("❌ Gagal membangun JNI libraries.");
            return;
        }

        // Setup Android home and SDK environments
        let os = std::env::consts::OS;
        let home = std::env::var("HOME").unwrap_or_default();
        let android_home = if let Ok(val) = std::env::var("ANDROID_HOME") {
            val
        } else {
            if os == "macos" {
                format!("{}/Library/Android/sdk", home)
            } else {
                format!("{}/Android/Sdk", home)
            }
        };
        unsafe {
            std::env::set_var("ANDROID_HOME", &android_home);
        }
        
        if std::env::var("JAVA_HOME").is_err() {
            let studio_jbr = if os == "macos" {
                let mut jbr = "/Applications/Android Studio.app/Contents/jbr/Contents/Home".to_string();
                if !std::path::Path::new(&jbr).exists() {
                    jbr = "/Applications/Android Studio.app/Contents/jre/Contents/Home".to_string();
                }
                jbr
            } else {
                let mut jbr = "/opt/android-studio/jbr".to_string();
                if !std::path::Path::new(&jbr).exists() {
                    jbr = "/usr/local/android-studio/jbr".to_string();
                }
                jbr
            };
            if std::path::Path::new(&studio_jbr).exists() {
                unsafe {
                    std::env::set_var("JAVA_HOME", &studio_jbr);
                }
            }
        }

        let local_props = std::path::Path::new("native/android/local.properties");
        if !local_props.exists() {
            if let Ok(mut file) = std::fs::File::create(local_props) {
                use std::io::Write;
                let _ = writeln!(file, "sdk.dir={}", android_home);
            }
        }

        let gradle_task = match (is_aab, release_mode) {
            (false, false) => "assembleDebug",
            (false, true) => "assembleRelease",
            (true, false) => "bundleDebug",
            (true, true) => "bundleRelease",
        };

        println!("\n🔨 Membangun target Android via Gradle (task: {})...", gradle_task);
        
        let mut build_cmd = if std::path::Path::new("native/android/gradlew").exists() {
            let mut cmd = std::process::Command::new("./gradlew");
            cmd.arg(gradle_task);
            cmd.current_dir("native/android");
            cmd
        } else {
            let mut cmd = std::process::Command::new("gradle");
            cmd.arg(gradle_task);
            cmd.current_dir("native/android");
            cmd
        };

        let spawn_res = build_cmd
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn();

        if let Ok(mut child) = spawn_res {
            let status = child.wait().expect("Gagal menunggu Gradle build");
            if status.success() {
                println!("\n✅ Build Android selesai dengan sukses!");
                let output_dir = if is_aab {
                    let mode_folder = if release_mode { "release" } else { "debug" };
                    format!("native/android/app/build/outputs/bundle/{}", mode_folder)
                } else {
                    let mode_folder = if release_mode { "release" } else { "debug" };
                    format!("native/android/app/build/outputs/apk/{}", mode_folder)
                };
                println!("📂 Folder output: {}", output_dir);
            } else {
                println!("\n❌ Gradle build gagal.");
            }
        } else {
            println!("❌ Gagal mengeksekusi Gradle wrapper. Pastikan Java dan Gradle wrapper terkonfigurasi dengan benar.");
        }
    }
}
