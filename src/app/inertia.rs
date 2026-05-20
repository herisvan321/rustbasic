use rustbasic_core::requests::Request;
use rustbasic_core::axum::response::{IntoResponse, Response};
use rustbasic_core::axum::http::{header, StatusCode};
use rustbasic_core::serde_json::{json, Value};
use std::fs;

/// Helper untuk merender halaman SPA menggunakan React.js + Inertia.js
pub fn inertia(req: &Request, component: &str, props: Value) -> Response {
    let is_inertia = req.headers.get("x-inertia").map(|v| v == "true").unwrap_or(false);
    let url = req.path.clone();
    
    // Versi asset (bisa dikonfigurasi untuk deteksi kadaluwarsa aset)
    let version = ""; 

    let errors: std::collections::HashMap<String, String> = req.session.get("errors").unwrap_or_default();
    req.session.remove("errors");

    let success: Option<String> = req.session.get("success");
    req.session.remove("success");

    let error: Option<String> = req.session.get("error");
    req.session.remove("error");

    let warning: Option<String> = req.session.get("warning");
    req.session.remove("warning");

    let info: Option<String> = req.session.get("info");
    req.session.remove("info");

    let mut props = props;
    if let Value::Object(ref mut map) = props {
        map.insert("errors".to_string(), json!(errors));
        map.insert("flash".to_string(), json!({
            "success": success,
            "error": error,
            "warning": warning,
            "info": info
        }));
    }

    let page_object = json!({
        "component": component,
        "props": props,
        "url": url,
        "version": version
    });

    if is_inertia {
        // Return JSON response untuk navigasi SPA Inertia
        let body = rustbasic_core::serde_json::to_string(&page_object).unwrap_or_default();
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header("X-Inertia", "true")
            .body(rustbasic_core::axum::body::Body::from(body))
            .unwrap()
            .into_response()
    } else {
        // Return layout root HTML "app.rb.html" untuk initial page load
        let vite_assets = get_vite_assets();
        let ctx = rustbasic_core::minijinja::context! {
            page => page_object,
            vite_assets => vite_assets,
        };
        
        crate::app::view(req, "app.rb.html", ctx)
    }
}

/// Helper untuk mendapatkan HTML tag asset Vite (JS/CSS) secara dinamis
pub fn get_vite_assets() -> String {
    let cfg = rustbasic_core::Config::load();
    let debug = cfg.app_debug;

    if debug {
        let port = cfg.vite_port;
        let host = &cfg.app_host;
        let display_host = if host == "0.0.0.0" || host.is_empty() { "localhost" } else { host };

        // Mode Development: Hubungkan ke Vite Dev Server kustom host dan port
        format!(
            r#"
        <!-- Vite Dev Server Integration -->
         <script type="module">
          import RefreshRuntime from 'http://{host}:{port}/@react-refresh';
          RefreshRuntime.injectIntoGlobalHook(window);
          window.$RefreshReg$ = () => {{}};
          window.$RefreshSig$ = () => (type) => type;
          window.__vite_plugin_react_preamble_installed__ = true;
        </script>
        <script type="module" src="http://{host}:{port}/src/resources/js/main.jsx"></script>
        "#,
            host = display_host,
            port = port
        )
    } else {
        // Mode Production: Baca manifest.json dari build hasil compile Vite
        let mut manifest_content = String::new();
        let paths = ["public/build/.vite/manifest.json", "public/build/manifest.json"];
        for path in &paths {
            if let Ok(content) = fs::read_to_string(path) {
                manifest_content = content;
                break;
            }
        }
        if !manifest_content.is_empty() {
            if let Ok(manifest) = rustbasic_core::serde_json::from_str::<Value>(&manifest_content) {
                if let Some(entry) = manifest.get("src/resources/js/main.jsx") {
                    let file = entry.get("file").and_then(|f| f.as_str()).unwrap_or("assets/main.js");
                    let mut assets_html = format!(r#"<script type="module" src="/build/{}"></script>"#, file);
                    
                    if let Some(css_arr) = entry.get("css").and_then(|c| c.as_array()) {
                        for css in css_arr {
                            if let Some(css_str) = css.as_str() {
                                assets_html = format!(r#"<link rel="stylesheet" href="/build/{}" />"#, css_str) + &assets_html;
                            }
                        }
                    }
                    return assets_html;
                }
            }
        }
        
        // Fallback jika manifest.json tidak ditemukan
        r#"<script type="module" src="/build/assets/main.js"></script>"#.to_string()
    }
}
