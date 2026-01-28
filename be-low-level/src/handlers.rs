use chrono::Utc;
use deadpool_redis::Connection;
use rand::Rng;
use salvo::prelude::*;
use uuid::Uuid;

use crate::models::{ErrorResponse, LoginRequest, LoginResponse, TryLuckResponse};
use crate::redis_store::{self, RedisError};
use crate::state::AppState;
use crate::PASSWORD;

const WIN_RATE_REDUCTION_THRESHOLD: u64 = 30;
const HIGH_WIN_RATE: f64 = 0.7;
const REDUCED_WIN_RATE: f64 = 0.4;

#[handler]
pub async fn health(_req: &mut Request, _dep: &mut Depot, res: &mut Response) {
    res.render("OK");
}

#[handler]
pub async fn login(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    // If this succeeds, the email is guaranteed to be valid syntax-wise.
    // Salvo will return 400 Bad Request automatically if Serde fails.
    let payload = match req.parse_json::<LoginRequest>().await {
        Ok(p) => p,
        Err(_) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(ErrorResponse {
                error: "Invalid JSON or Email format".into(),
            }));
            return;
        }
    };

    if payload.password != PASSWORD {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(ErrorResponse {
            error: "Invalid credentials".into(),
        }));
        return;
    }

    let token = Uuid::new_v4().to_string();

    let mut conn = match get_connection_or_respond(state, res).await {
        Some(c) => c,
        None => return,
    };

    if let Err(err) = redis_store::add_active_token(&mut conn, &token).await {
        render_redis_error(res, err);
        return;
    }

    res.render(Json(LoginResponse { token }));
}

#[handler]
pub async fn logout(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    let token = match extract_token_or_unauthorized(
        req,
        res,
        "Missing or invalid Authorization header",
    ) {
        Some(t) => t,
        None => return,
    };

    let mut conn = match get_connection_or_respond(state, res).await {
        Some(c) => c,
        None => return,
    };

    let removed: i32 = match redis_store::remove_active_token(&mut conn, &token).await {
        Ok(r) => r,
        Err(err) => {
            render_redis_error(res, err);
            return;
        }
    };

    if removed > 0 {
        res.render("OK");
    } else {
        render_unauthorized(res, "Invalid Token");
    }
}

#[handler]
pub async fn try_luck(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    // 1. Auth Check
    let token = match extract_token_or_unauthorized(req, res, "Missing Authorization header") {
        Some(t) => t,
        None => return,
    };

    let mut conn = match get_connection_or_respond(state, res).await {
        Some(c) => c,
        None => return,
    };

    let is_member: bool = match redis_store::is_active_token(&mut conn, &token).await {
        Ok(v) => v,
        Err(err) => {
            render_redis_error(res, err);
            return;
        }
    };

    if !is_member {
        render_unauthorized(res, "Invalid Token");
        return;
    }

    // 2. Game Logic
    let now = Utc::now();
    let today = now.date_naive();
    let wins_key = format!("wins:{today}");

    let wins_today: u64 = match redis_store::get_wins_today(&mut conn, &wins_key).await {
        Ok(v) => v,
        Err(err) => {
            render_redis_error(res, err);
            return;
        }
    };

    // Win Logic: 0.7 chance normally, 0.4 chance if >= 30 wins today
    let probability = if wins_today >= WIN_RATE_REDUCTION_THRESHOLD {
        REDUCED_WIN_RATE
    } else {
        HIGH_WIN_RATE
    };

    let is_win: bool = {
        let mut rng = rand::rng();
        rng.random_bool(probability)
    };

    if is_win && let Err(err) = redis_store::increment_wins(&mut conn, &wins_key).await {
        render_redis_error(res, err);
        return;
    }

    // Set expiry to next local midnight (idempotent; cheap if already set)
    let tomorrow_midnight = (now + chrono::Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let seconds_until_midnight = (tomorrow_midnight - now.naive_utc()).num_seconds();
    if seconds_until_midnight > 0 {
        let _ = redis_store::set_expiry(&mut conn, &wins_key, seconds_until_midnight).await;
    }

    res.render(Json(TryLuckResponse { win: is_win }));
}

// Helper to parse "Bearer <token>"
fn extract_token(req: &Request) -> Option<String> {
    req.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
}

async fn get_connection_or_respond(
    state: &AppState,
    res: &mut Response,
) -> Option<Connection> {
    match redis_store::get_connection(&state.redis_pool).await {
        Ok(c) => Some(c),
        Err(err) => {
            render_redis_error(res, err);
            None
        }
    }
}

fn extract_token_or_unauthorized(
    req: &Request,
    res: &mut Response,
    message: &str,
) -> Option<String> {
    match extract_token(req) {
        Some(token) => Some(token),
        None => {
            render_unauthorized(res, message);
            None
        }
    }
}

fn render_unauthorized(res: &mut Response, message: &str) {
    res.status_code(StatusCode::UNAUTHORIZED);
    res.render(Json(ErrorResponse {
        error: message.into(),
    }));
}

fn render_redis_error(res: &mut Response, err: RedisError) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
    let message = match err {
        RedisError::Unavailable => "Redis unavailable",
        RedisError::Command => "Redis error",
    };
    res.render(Json(ErrorResponse {
        error: message.into(),
    }));
}
