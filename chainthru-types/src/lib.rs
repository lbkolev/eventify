//! chainthru-types contains all the types used in the chainthru project.
#![allow(clippy::option_map_unit_fn)]

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::PgPool;
use url::Url;
use web3::types::{H160, H256};

pub mod block;
pub mod erc20;
pub mod erc721;
pub mod error;
pub mod macros;
pub mod tx;

pub use block::IndexedBlock;

// re-exports necessary for macros to work
pub use async_trait::async_trait;
pub use convert_case::{Case, Casing};

/// The result type used through the types application code.
type Result<T> = std::result::Result<T, error::Error>;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub database_name: String,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
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
            .password(self.password.expose_secret())
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
            password: Secret::new(url.password().unwrap_or("").to_owned()),
            require_ssl: false,
        }
    }
}

#[async_trait::async_trait]
pub trait Insertable: Sized {
    async fn insert(&self, conn: &PgPool) -> Result<()>;
}

#[derive(
    derive_builder::Builder, Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq,
)]
#[serde(rename_all = "camelCase")]
pub struct TXBoilerplate {
    contract_addr: H160,
    transaction_hash: H256,
    transaction_sender: H160,
}
