//! chainthru-types contains all the types used in the chainthru project.

use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use url::Url;

pub mod erc20;
pub mod erc721;
pub mod macros;

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let require_ssl = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(require_ssl)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

impl From<String> for DatabaseSettings {
    fn from(s: String) -> Self {
        let url = Url::parse(&s).expect("Invalid database URL");

        Self {
            database_name: url.path().trim_start_matches('/').to_owned(),
            host: url.host_str().unwrap_or("localhost").to_owned(),
            port: url.port().unwrap_or(5432),
            username: url.username().to_owned(),
            password: url.password().unwrap_or("").to_owned(),
            require_ssl: false,
        }
    }
}
