use deadpool_redis::{Config, Pool, Runtime};
use salvo::prelude::*;
use std::path::Path;

// Expose these modules publicly so tests can use the structs (LoginResponse, etc.)
pub mod handlers;
pub mod models;
pub mod redis_store;
pub mod state;

use handlers::{health, login, logout, try_luck};
use state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .hoop(state)
        .push(Router::with_path("api/health").get(health))
        .push(Router::with_path("api/login").post(login))
        .push(Router::with_path("api/logout").post(logout))
        .push(Router::with_path("api/try_luck").post(try_luck))
}

// Helper to create a fresh state (useful for tests and main)
pub async fn create_initial_state() -> AppState {
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    let pool: Pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create Redis pool");

    let password = load_password();
    AppState { redis_pool: pool, password }
}

fn load_password() -> String {
    if let Ok(password) = std::env::var("APP_PASSWORD") {
        return password;
    }

    let secret_path = std::env::var("APP_PASSWORD_FILE")
        .unwrap_or_else(|_| "/run/secrets/app_password".to_string());
    if !Path::new(&secret_path).exists() {
        panic!(
            "APP_PASSWORD or APP_PASSWORD_FILE must be set; secret file not found at {secret_path}"
        );
    }

    std::fs::read_to_string(&secret_path)
        .unwrap_or_else(|err| panic!("Failed to read secret file {secret_path}: {err}"))
        .trim()
        .to_string()
}
