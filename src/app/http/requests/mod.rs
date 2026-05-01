/* ---------------------------------------------------------
 * 📑 LABEL: LARAVEL STYLE REQUEST (app/http/requests/mod.rs)
 * File ini menyediakan cara mudah mengambil data input dari request.
 * --------------------------------------------------------- */

use axum::{
    extract::{FromRequest, FromRequestParts, Query, Request as AxumRequest},
    http::Method,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct Request {
    pub inputs: Value,
    pub method: Method,
    pub headers: HashMap<String, String>,
}

impl Request {
    /// Mengambil input berdasarkan key, contoh: req.input("name")
    pub fn input(&self, key: &str) -> Option<String> {
        self.inputs.get(key).and_then(|v| {
            match v {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                Value::Bool(b) => Some(b.to_string()),
                _ => None,
            }
        })
    }

    /// Mengambil semua input sebagai JSON Value
    pub fn all(&self) -> &Value {
        &self.inputs
    }

    /// Mengambil header berdasarkan key
    pub fn header(&self, key: &str) -> Option<String> {
        self.headers.get(key).cloned()
    }
}

impl<S> FromRequest<S> for Request
where
    S: Send + Sync,
{
    type Rejection = (axum::http::StatusCode, String);

    async fn from_request(req: AxumRequest, state: &S) -> Result<Self, Self::Rejection> {
        let method = req.method().clone();
        let mut headers = HashMap::new();
        for (name, value) in req.headers() {
            headers.insert(
                name.to_string(),
                value.to_str().unwrap_or("").to_string()
            );
        }

        // 1. Ambil Query Parameters
        let mut combined_inputs = json!({});
        let (mut parts, _body) = req.into_parts();
        
        if let Ok(Query(query_map)) = Query::<HashMap<String, String>>::from_request_parts(&mut parts, state).await {
            for (k, v) in query_map {
                combined_inputs[k] = json!(v);
            }
        }

        Ok(Self {
            inputs: combined_inputs,
            method,
            headers,
        })
    }
}
