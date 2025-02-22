use std::fmt::{Display, Formatter};

use figment::{providers::Env, value::Value, Figment};
use plerkle_messenger::MessengerConfig;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::error::IngesterError;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct IngesterConfig {
    pub database_config: DatabaseConfig,
    pub messenger_config: MessengerConfig,
    pub env: Option<String>,
    pub rpc_config: RpcConfig,
    pub metrics_port: Option<u16>,
    pub metrics_host: Option<String>,
    pub backfiller: Option<bool>,
    pub role: Option<IngesterRole>,
    pub max_postgres_connections: Option<u32>,
    pub account_stream_worker_count: Option<u32>,
    pub transaction_stream_worker_count: Option<u32>,
    pub code_version: Option<&'static str>,
}

impl IngesterConfig {
    /// Get the db url out of the dict, this is built a a dict so that future extra db parameters can be easily shoved in.
    /// this panics if the key is not present
    pub fn get_database_url(&self) -> String {
        self.database_config
            .get(DATABASE_URL_KEY)
            .and_then(|u| u.clone().into_string())
            .ok_or(IngesterError::ConfigurationError {
                msg: format!("Database connection string missing: {}", DATABASE_URL_KEY),
            })
            .unwrap()
    }

    pub fn get_rpc_url(&self) -> String {
        self.rpc_config
            .get(RPC_URL_KEY)
            .and_then(|u| u.clone().into_string())
            .ok_or(IngesterError::ConfigurationError {
                msg: format!("RPC connection string missing: {}", RPC_URL_KEY),
            })
            .unwrap()
    }

    pub fn get_messneger_client_config(&self) -> MessengerConfig {
        let mut mc = self.messenger_config.clone();
        mc.connection_config
            .insert("consumer_id".to_string(), Value::from(rand_string()));
        mc
    }

    pub fn get_account_stream_worker_count(&self) -> u32 {
        self.account_stream_worker_count.unwrap_or(2)
    }

    pub fn get_transaction_stream_worker_count(&self) -> u32 {
        self.transaction_stream_worker_count.unwrap_or(2)
    }
}

// Types and constants used for Figment configuration items.
pub type DatabaseConfig = figment::value::Dict;

pub const DATABASE_URL_KEY: &str = "url";
pub const DATABASE_LISTENER_CHANNEL_KEY: &str = "listener_channel";

pub type RpcConfig = figment::value::Dict;

pub const RPC_URL_KEY: &str = "url";
pub const RPC_COMMITMENT_KEY: &str = "commitment";
pub const CODE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum IngesterRole {
    All,
    Backfiller,
    BackgroundTaskRunner,
    Ingester,
}

impl Display for IngesterRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IngesterRole::All => write!(f, "All"),
            IngesterRole::Backfiller => write!(f, "Backfiller"),
            IngesterRole::BackgroundTaskRunner => write!(f, "BackgroundTaskRunner"),
            IngesterRole::Ingester => write!(f, "Ingester"),
        }
    }
}

pub fn rand_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn setup_config() -> IngesterConfig {
    let mut config: IngesterConfig = Figment::new()
        .join(Env::prefixed("INGESTER_"))
        .extract()
        .map_err(|config_error| IngesterError::ConfigurationError {
            msg: format!("{}", config_error),
        })
        .unwrap();
    config.code_version = Some(CODE_VERSION);
    config
}
