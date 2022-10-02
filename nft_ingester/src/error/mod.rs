use crate::BgTask;
use blockbuster::error::BlockbusterError;
use plerkle_messenger::MessengerError;
use sea_orm::{DbErr, TransactionError};
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

#[derive(Error, Debug)]
pub enum IngesterError {
    #[error("ChangeLog Event Malformed")]
    ChangeLogEventMalformed,
    #[error("Compressed Asset Event Malformed")]
    CompressedAssetEventMalformed,
    #[error("Error downloading batch files")]
    BatchInitNetworkingError,
    #[error("Error writing batch files")]
    BatchInitIOError,
    #[error("Storage listener error: ({msg})")]
    StorageListenerError { msg: String },
    #[error("Storage Write Error {0}")]
    StorageWriteError(String),
    #[error("NotImplemented")]
    NotImplemented,
    #[error("Deserialization Error {0}")]
    DeserializationError(String),
    #[error("Task Manager Error {0}")]
    TaskManagerError(String),
    #[error("Missing or invalid configuration: ({msg})")]
    ConfigurationError { msg: String },
    #[error("Error getting RPC data {0}")]
    RpcGetDataError(String),
    #[error("RPC returned data in unsupported format {0}")]
    RpcDataUnsupportedFormat(String),
    #[error("Data serializaton error {0}")]
    SerializatonError(String),
    #[error("Messenger error {0}")]
    MessengerError(String),
    #[error("Blockbuster Parsing error {0}")]
    ParsingError(String),
}

impl From<reqwest::Error> for IngesterError {
    fn from(_err: reqwest::Error) -> Self {
        IngesterError::BatchInitNetworkingError
    }
}

impl From<BlockbusterError> for IngesterError {
    fn from(err: BlockbusterError) -> Self {
        IngesterError::ParsingError(err.to_string())
    }
}

impl From<std::io::Error> for IngesterError {
    fn from(_err: std::io::Error) -> Self {
        IngesterError::BatchInitIOError
    }
}

impl From<DbErr> for IngesterError {
    fn from(e: DbErr) -> Self {
        IngesterError::StorageWriteError(e.to_string())
    }
}

impl From<TransactionError<IngesterError>> for IngesterError {
    fn from(e: TransactionError<IngesterError>) -> Self {
        IngesterError::StorageWriteError(e.to_string())
    }
}

impl From<SendError<Box<dyn BgTask>>> for IngesterError {
    fn from(err: SendError<Box<dyn BgTask>>) -> Self {
        IngesterError::TaskManagerError(format!("Could not create task: {:?}", err.to_string()))
    }
}

impl From<MessengerError> for IngesterError {
    fn from(e: MessengerError) -> Self {
        IngesterError::MessengerError(e.to_string())
    }
}
