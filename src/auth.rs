use crate::kv;
use axum::http::header::HeaderMap;
use worker::Env;

pub async fn check(headers: &HeaderMap, env: &Env) -> Result<String, ()> {
    let token = match headers.get("Authorization") {
        Some(header_value) => {
            let auth_token = match header_value.to_str() {
                Ok(auth_token) => auth_token,
                Err(_) => return Err(()),
            };

            if auth_token.starts_with("Bearer ") {
                auth_token[7..].to_string()
            } else {
                return Err(());
            }
        }
        None => return Err(()),
    };

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return Err(());
    }

    let id = parts[0].to_string();
    let secret = parts[1].to_string();
    match kv::get(&env, id.clone()).await {
        Ok(kv_secret) => {
            if kv_secret == secret {
                return Ok(id);
            } else {
                return Err(());
            }
        }
        Err(_) => {
            return Err(());
        }
    }
}
