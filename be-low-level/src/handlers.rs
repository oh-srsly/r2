use chrono::Local;
use deadpool_redis::redis::AsyncCommands;
use rand::Rng;
use salvo::prelude::*;
use uuid::Uuid;

use crate::models::{ErrorResponse, LoginRequest, LoginResponse, TryLuckResponse};
use crate::state::AppState;

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

    if payload.password != "r2isthebest" {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(ErrorResponse {
            error: "Invalid credentials".into(),
        }));
        return;
    }

    let token = Uuid::new_v4().to_string();

    let mut conn = match state.redis_pool.get().await {
        Ok(c) => c,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis unavailable".into(),
            }));
            return;
        }
    };

    if let Err(_) = conn.sadd::<_, _, i32>("active_tokens", &token).await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(ErrorResponse {
            error: "Redis error".into(),
        }));
        return;
    }

    res.render(Json(LoginResponse { token }));
}

#[handler]
pub async fn logout(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    let token = match extract_token(req) {
        Some(t) => t,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(ErrorResponse {
                error: "Missing or invalid Authorization header".into(),
            }));
            return;
        }
    };

    let mut conn = match state.redis_pool.get().await {
        Ok(c) => c,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis unavailable".into(),
            }));
            return;
        }
    };

    let removed: i32 = match conn.srem("active_tokens", &token).await {
        Ok(r) => r,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis error".into(),
            }));
            return;
        }
    };

    if removed > 0 {
        res.render("OK");
    } else {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(ErrorResponse {
            error: "Invalid Token".into(),
        }));
    }
}

#[handler]
pub async fn try_luck(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    // 1. Auth Check
    let token = match extract_token(req) {
        Some(t) => t,
        None => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(ErrorResponse {
                error: "Missing Authorization header".into(),
            }));
            return;
        }
    };

    let mut conn = match state.redis_pool.get().await {
        Ok(c) => c,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis unavailable".into(),
            }));
            return;
        }
    };

    let is_member: bool = match conn.sismember("active_tokens", &token).await {
        Ok(v) => v,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis error".into(),
            }));
            return;
        }
    };

    if !is_member {
        res.status_code(StatusCode::UNAUTHORIZED);
        res.render(Json(ErrorResponse {
            error: "Invalid Token".into(),
        }));
        return;
    }

    // 2. Game Logic
    let today = Local::now().date_naive();
    let wins_key = format!("wins:{today}");

    let wins_today: u64 = match conn.get::<_, Option<u64>>(&wins_key).await {
        Ok(Some(v)) => v,
        Ok(None) => 0,
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis error".into(),
            }));
            return;
        }
    };

    // Win Logic: 0.7 chance normally, 0.4 chance if >= 30 wins today
    let probability = if wins_today >= 30 { 0.4 } else { 0.7 };

    let is_win: bool = {
        let mut rng = rand::rng();
        rng.random_bool(probability)
    };

    if is_win {
        if let Err(_) = conn.incr::<_, _, u64>(&wins_key, 1).await {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(ErrorResponse {
                error: "Redis error".into(),
            }));
            return;
        }
    }

    // Set expiry to next local midnight (idempotent; cheap if already set)
    let now = Local::now();
    let tomorrow_midnight = (now + chrono::Duration::days(1))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let seconds_until_midnight = (tomorrow_midnight - now.naive_local()).num_seconds();
    if seconds_until_midnight > 0 {
        let _ = conn
            .expire::<_, ()>(&wins_key, seconds_until_midnight)
            .await;
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
