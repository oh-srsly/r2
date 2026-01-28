use be_low_level::models::{LoginResponse, TryLuckResponse};
use be_low_level::{build_router, create_initial_state};
use be_low_level::state::AppState;
use salvo::prelude::*;
use salvo::test::{ResponseExt, TestClient};

const BASE_URL: &str = "http://localhost";

// Build a fresh router each time, but keep shared state across requests/tests.
fn router_with_state(state: &AppState) -> Router {
    build_router(state.clone())
}

#[tokio::test]
async fn test_full_user_flow() {
    let state = create_initial_state().await;

    // 1. Login
    let mut res = TestClient::post(format!("{BASE_URL}/api/login"))
        .json(&serde_json::json!({
            "email": "test@gmail.com",
            "password": "r2isthebest"
        }))
        .send(router_with_state(&state))
        .await;

    assert_eq!(res.status_code, Some(StatusCode::OK));

    let login_resp: LoginResponse = res.take_json().await.unwrap();
    let token = login_resp.token;

    // 2. Try Luck
    let mut res = TestClient::post(format!("{BASE_URL}/api/try_luck"))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .send(router_with_state(&state))
        .await;

    assert_eq!(res.status_code, Some(StatusCode::OK));
    let _luck: TryLuckResponse = res.take_json().await.unwrap();

    // 3. Logout
    let res = TestClient::post(format!("{BASE_URL}/api/logout"))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .send(router_with_state(&state))
        .await;

    assert_eq!(res.status_code, Some(StatusCode::OK));
}

#[tokio::test]
async fn test_invalid_credentials() {
    let state = create_initial_state().await;

    let res = TestClient::post(format!("{BASE_URL}/api/login"))
        .json(&serde_json::json!({
            "email": "test@gmail.com",
            "password": "wrong"
        }))
        .send(router_with_state(&state))
        .await;

    assert_eq!(res.status_code, Some(StatusCode::UNAUTHORIZED));
}
