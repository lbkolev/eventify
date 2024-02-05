use std::fmt::Display;

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use url::Url;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub require_ssl: bool,
}

impl DatabaseConfig {
    pub fn without_db(&self) -> PgConnectOptions {
        let require_ssl = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(require_ssl)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_name: "eventify".to_owned(),
            host: "localhost".to_owned(),
            port: 5432,
            username: "postgres".to_owned(),
            password: Secret::new("password".to_owned()),
            require_ssl: false,
        }
    }
}

impl From<String> for DatabaseConfig {
    fn from(s: String) -> Self {
        let url = Url::parse(&s).expect("Invalid database URL");

        Self {
            database_name: url.path().trim_start_matches('/').to_owned(),
            host: url.host_str().unwrap_or("localhost").to_owned(),
            port: url.port().unwrap_or(5432),
            username: url.username().to_owned(),
            password: Secret::new(url.password().unwrap_or("").to_owned()),
            require_ssl: false,
        }
    }
}

impl From<DatabaseConfig> for String {
    fn from(settings: DatabaseConfig) -> Self {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            settings.username,
            settings.password.expose_secret(),
            settings.host,
            settings.port,
            settings.database_name
        )
    }
}

impl From<&str> for DatabaseConfig {
    fn from(s: &str) -> Self {
        Self::from(s.to_owned())
    }
}

impl Display for DatabaseConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        )
    }
}
