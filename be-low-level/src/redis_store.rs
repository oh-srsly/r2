use deadpool_redis::{Connection, Pool, redis::AsyncCommands};

#[derive(Debug)]
pub enum RedisError {
    Unavailable,
    Command,
}

pub async fn get_connection(pool: &Pool) -> Result<Connection, RedisError> {
    pool.get().await.map_err(|_| RedisError::Unavailable)
}

pub async fn add_active_token(conn: &mut Connection, token: &str) -> Result<(), RedisError> {
    conn.sadd::<_, _, i32>("active_tokens", token)
        .await
        .map(|_| ())
        .map_err(|_| RedisError::Command)
}

pub async fn remove_active_token(conn: &mut Connection, token: &str) -> Result<i32, RedisError> {
    conn.srem("active_tokens", token)
        .await
        .map_err(|_| RedisError::Command)
}

pub async fn is_active_token(conn: &mut Connection, token: &str) -> Result<bool, RedisError> {
    conn.sismember("active_tokens", token)
        .await
        .map_err(|_| RedisError::Command)
}

pub async fn get_wins_today(conn: &mut Connection, wins_key: &str) -> Result<u64, RedisError> {
    match conn.get::<_, Option<u64>>(wins_key).await {
        Ok(Some(v)) => Ok(v),
        Ok(None) => Ok(0),
        Err(_) => Err(RedisError::Command),
    }
}

pub async fn increment_wins(conn: &mut Connection, wins_key: &str) -> Result<(), RedisError> {
    conn.incr::<_, _, u64>(wins_key, 1)
        .await
        .map(|_| ())
        .map_err(|_| RedisError::Command)
}

pub async fn set_expiry(
    conn: &mut Connection,
    wins_key: &str,
    seconds: i64,
) -> Result<(), RedisError> {
    conn.expire::<_, ()>(wins_key, seconds)
        .await
        .map_err(|_| RedisError::Command)
}
