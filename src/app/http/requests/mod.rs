/* ---------------------------------------------------------
 * 📑 LABEL: REQUEST HELPER (requests/mod.rs)
 * Library untuk memproses input (form/query) dengan gaya Laravel.
 * Kini mendukung Validasi dan Session (Flash Input).
 * --------------------------------------------------------- */

use axum::{
    extract::{FromRequest, FromRequestParts, Query, Form},
    http::Method,
    response::{IntoResponse, Response},
};
use serde_json::{json, Value};
use std::collections::HashMap;

use validator::Validate;
use axum_session::Session;
use crate::database::session_manager::LaravelSessionStore;

pub struct Request {
    pub inputs: Value,
    pub method: Method,
    #[allow(dead_code)]
    pub headers: HashMap<String, String>,
    pub session: Session<LaravelSessionStore>,
}

impl Request {
    /// Mengambil input berdasarkan key
    pub fn input(&self, key: &str) -> Option<String> {
        self.inputs.get(key).and_then(|v| {
            if v.is_string() {
                v.as_str().map(|s| s.to_string())
            } else {
                Some(v.to_string())
            }
        })
    }

    /// Mengambil semua input dalam bentuk JSON
    pub fn all(&self) -> &Value {
        &self.inputs
    }

    /// Validasi input berdasarkan struct yang mengimplementasikan trait Validate
    pub fn validate<T: Validate + serde::de::DeserializeOwned>(&self) -> Result<T, HashMap<String, Vec<String>>> {
        let data: T = serde_json::from_value(self.inputs.clone()).map_err(|_| {
            let mut err = HashMap::new();
            err.insert("error".to_string(), vec!["Gagal memproses data".to_string()]);
            err
        })?;

        if let Err(errors) = data.validate() {
            let mut field_errors = HashMap::new();
            for (field, errs) in errors.field_errors() {
                let messages: Vec<String> = errs.iter()
                    .map(|e| e.message.clone().unwrap_or_else(|| "Input tidak valid".into()).to_string())
                    .collect();
                field_errors.insert(field.to_string(), messages);
            }
            
            // Simpan input lama ke session (Flash Input)
            self.session.set("old_input", self.inputs.clone());
            self.session.set("errors", field_errors.clone());
            
            return Err(field_errors);
        }

        // Jika berhasil, hapus data lama dan error
        self.session.remove("old_input");
        self.session.remove("errors");

        Ok(data)
    }

    /// Mengambil header berdasarkan key
    #[allow(dead_code)]
    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }
}

// Implementasi FromRequest secara native untuk Axum 0.8
impl<S> FromRequest<S> for Request
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let method = req.method().clone();
        
        // Ambil Session
        let session = req.extensions()
            .get::<Session<LaravelSessionStore>>()
            .cloned()
            .ok_or_else(|| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Session tidak ditemukan").into_response())?;

        let mut inputs = json!({});

        // Ambil Query Params dan Body
        let (mut parts, body) = req.into_parts();
        
        // 1. Query
        if let Ok(Query(query)) = Query::<Value>::from_request_parts(&mut parts, state).await {
            if let Some(obj) = query.as_object() {
                for (k, v) in obj {
                    inputs[k] = v.clone();
                }
            }
        }

        // 2. Body
        let req_for_body = axum::extract::Request::from_parts(parts, body);
        if let Ok(Form(form)) = Form::<Value>::from_request(req_for_body, state).await {
            if let Some(obj) = form.as_object() {
                for (k, v) in obj {
                    inputs[k] = v.clone();
                }
            }
        }

        Ok(Request {
            inputs,
            method,
            headers: HashMap::new(),
            session,
        })
    }
}
