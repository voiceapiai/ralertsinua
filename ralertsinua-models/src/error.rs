// use color_eyre::eyre::Error;
use serde::Deserialize;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ModelResult<T> = Result<T, ModelError>;

/// Matches errors that are returned from the alerts.in.ua
/// API as part of the JSON response object.
#[derive(Debug, Error, Deserialize)]
pub enum ApiError {
    /// See [Error Object](https://devs.alerts.in.ua/en#documentationerrors)
    #[error("{status}: {message}")]
    #[serde(alias = "error")]
    Regular { status: u16, message: String },

    /// See [Play Error Object](https://devs.alerts.in.ua/en#documentationerrors)
    #[error("{status} ({reason}): {message}")]
    #[serde(alias = "error")]
    Player {
        status: u16,
        message: String,
        reason: String,
    },
}

/// Groups up the kinds of errors that may happen in this crate.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum RAlertsError {
    #[error(transparent)]
    Other(#[from] color_eyre::eyre::Error), // source and Display delegate to color_eyre::eyre::Error
}
