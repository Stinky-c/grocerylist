use figment::Figment;
use figment::providers::{Env, Format, Toml};
use figment_file_provider_adapter::FileAdapter;
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database_url: String,
    #[serde(default = "config_database_max_connections")]
    pub database_max_connections: u32,

    pub jwt_secret: String,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        Figment::new()
            .merge(Env::raw().only(&["DATABASE_URL"]))
            .merge(FileAdapter::wrap(Env::prefixed("APP_")))
            .merge(FileAdapter::wrap(Toml::file("config.toml")))
            .extract()
            .expect("Failed to extract app config")
    }

    pub fn jwt_pair(&self) -> (EncodingKey, DecodingKey) {
        (
            EncodingKey::from_secret(self.jwt_secret.as_ref()),
            DecodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }
}

fn config_database_max_connections() -> u32 {
    6
}
