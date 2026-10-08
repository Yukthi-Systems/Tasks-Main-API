use redis::{Client, aio::MultiplexedConnection};
use serde::{Serialize, de::DeserializeOwned};
use crate::errors::ServiceResult;
use redis::AsyncCommands;



/// Set a key-value pair in Redis with an expiration time
pub async fn set_redis_cache<T>(conn: &MultiplexedConnection, key: &str, value: &T, expiration_seconds: u64) -> ServiceResult<()>
    where
        T: Serialize,
{
    // Create a mutable reference to the Redis connection
    let mut redis_conn = conn.clone();

    // Serialize the value to a JSON string
    let json_value: String = serde_json::to_string(value)?;

    Ok(redis_conn
        .set_ex(
            key,
            json_value,
            expiration_seconds
        )
        .await?
    )
}


/// Get a value from Redis by key
pub async fn get_redis_cache<T>(conn: &MultiplexedConnection, key: &str) -> ServiceResult<Option<T>> 
    where
        T: DeserializeOwned,
{
    // Create a mutable reference to the Redis connection
    let mut redis_conn = conn.clone();

    // Get the JSON string from Redis
    let json_string: Option<String> = redis_conn.get(key).await?;

    if let Some(json) = json_string {
        let value: T = serde_json::from_str(&json)?;
        Ok(Some(value))
    } else {
        Ok(None)
    }
}


/// Delete a key from Redis
pub async fn delete_redis_cache(conn: &MultiplexedConnection, key: &str) -> ServiceResult<()> {
    let mut redis_conn = conn.clone();
    Ok(redis_conn.del(key).await?)
}


/// Initialize a Redis connection and verify it with a PING command
pub async fn init_redis(url: &str) -> MultiplexedConnection {
    let client = Client::open(url).expect("Failed to create Redis client");

    let mut conn = client.get_multiplexed_async_connection().await.expect("Failed to connect to Redis");

    let pong: String = redis::cmd("PING").query_async(&mut conn).await.expect("Failed to ping Redis");
    if pong != "PONG" {
        tracing::warn!("Unexpected PING response from Redis: {}", pong);
        panic!("Failed to connect to Redis");
    }

    conn
}
