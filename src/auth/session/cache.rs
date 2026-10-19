use crate::auth::session::KVStore;
use crate::auth::session::session::Session;
use std::convert::Infallible;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MemorySessionCache(moka::future::Cache<Uuid, Session>);

impl MemorySessionCache {
    pub fn new() -> Self {
        Self(moka::future::Cache::builder().initial_capacity(128).build())
    }
}

impl KVStore<Uuid, Session> for MemorySessionCache {
    type Error = Infallible;

    async fn get(&self, key: &Uuid) -> Result<Option<Session>, Self::Error> {
        let v = self.0.get(key).await;
        Ok(v)
    }

    async fn insert(&self, key: &Uuid, value: Session) -> Result<(), Self::Error> {
        self.0.insert(*key, value).await;
        Ok(())
    }

    async fn invalidate(&self, key: &Uuid) -> Result<(), Self::Error> {
        self.0.invalidate(key).await;
        Ok(())
    }
}
