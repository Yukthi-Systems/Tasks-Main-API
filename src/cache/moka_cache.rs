use moka::future::Cache;
use std::time::Duration;
use std::sync::Arc;


/// Cache key type
pub type Key = Arc<str>;

/// Typed cache storing shared values
pub type AppCache<T> = Cache<Key, Arc<T>>;


/// Insert a shared value into the cache
pub async fn cache_data<T>(cache: &AppCache<T>, key: &str, value: Arc<T>)
    where
        T: Send + Sync + 'static,
{
    cache.insert(Arc::<str>::from(key), value).await;
}


/// Retrieve a shared value from the cache
pub async fn get_cached_data<T>(cache: &AppCache<T>, key: &str) -> Option<Arc<T>>
    where
        T: Send + Sync + 'static,
{
    cache.get(&Arc::<str>::from(key)).await
}


/// Remove a value from the cache
pub async fn remove_cached_data<T>(cache: &AppCache<T>, key: &str)
    where
        T: Send + Sync + 'static,
{
    cache.invalidate(&Arc::<str>::from(key)).await;
}


/// Create a new Moka cache with the specified size and expiration time
pub fn moka_builder<T: Send + Sync + 'static>(cache_size: u64, expiration_time: Duration) -> AppCache<T> {
    AppCache::builder()
        .max_capacity(cache_size)
        .time_to_live(expiration_time)
        .build()
}
