use argon2::{Algorithm, Argon2, Params, Version};
use figment::Figment;
use figment::providers::{Env, Format, Toml};
use figment_file_provider_adapter::FileAdapter;
use fred::prelude::{Builder, ClientLike, Config, HeartbeatInterface};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database_url: String,

    pub valkey_url: String,

    #[serde(default = "config_database_max_connections")]
    pub database_max_connections: u32,

    pub jwt_secret: String,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        Figment::new()
            .merge(Env::raw().only(&["DATABASE_URL", "VALKEY_URL"]))
            .merge(FileAdapter::wrap(Env::prefixed("APP_")))
            .merge(FileAdapter::wrap(Toml::file("config.toml")))
            .extract()
            .expect("Failed to extract app config")
    }

    pub async fn database(&self) -> PgPool {
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(self.database_max_connections)
            .connect(&self.database_url)
            .await
            .expect("Failed to connect to database")
    }

    pub async fn kv(&self) -> fred::prelude::Client {
        let config = Config::from_url(&self.valkey_url).expect("Failed to parse to valkey url");
        let builder = Builder::from_config(config);

        let client = builder.build().unwrap();
        client.init().await.expect("Failed to init client");
        client
    }

    pub fn jwt_pair(&self) -> (EncodingKey, DecodingKey) {
        (
            EncodingKey::from_secret(self.jwt_secret.as_ref()),
            DecodingKey::from_secret(self.jwt_secret.as_ref()),
        )
    }

    pub fn password_hasher(&self) -> Argon2<'static> {
        Argon2::new(Algorithm::default(), Version::default(), Params::default())
    }

    pub fn validation(&self) -> Validation {
        let mut v = Validation::new(jsonwebtoken::Algorithm::HS256);
        v.set_required_spec_claims(&["exp", "aud", "iss", "sub"]);
        v.set_audience(&[env!("CARGO_CRATE_NAME")]);
        v.set_issuer(&[env!("CARGO_CRATE_NAME")]);
        v.validate_aud = true;
        v.validate_exp = true;
        v.validate_nbf = false;

        v
    }
}

fn config_database_max_connections() -> u32 {
    6
}
