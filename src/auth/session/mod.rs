pub(super) mod cache;
pub(super) mod session;
pub(super) mod store;

/// Generic trait to
#[allow(async_fn_in_trait)] // TODO: unsure about this
pub trait KVStore<K, V> {
    type Error: std::error::Error;
    /// Get a value
    async fn get(&self, key: &K) -> Result<Option<V>, Self::Error>;
    /// Insert key
    async fn insert(&self, key: &K, value: V) -> Result<(), Self::Error>;
    /// Drop a value
    async fn invalidate(&self, key: &K) -> Result<(), Self::Error>;
}
