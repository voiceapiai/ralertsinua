#[allow(unused_imports)]
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::action::*;

/// Groups up the kinds of errors that may happen in this crate.
#[derive(Error, Diagnostic, Debug)]
#[error("ralertsinua error!")]
pub enum AppError {
    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
    #[error("tokio send error: {0}")]
    TokioSend(#[from] tokio::sync::mpsc::error::SendError<Action>),
    #[error("tokio channel error: {0}")]
    TokioChannel(#[from] tokio::sync::mpsc::error::TrySendError<Action>),
    // #[error("encode error: {0}")]
    // Encode(#[from] rmp_serde::encode::Error),
    // #[error("decode error: {0}")]
    // Decode(#[from] rmp_serde::decode::Error),
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),
    #[error("cacache error: {0}")]
    Cache(#[from] cacache::Error),
    #[error("component error")]
    #[diagnostic(code(ralertsinua::component))]
    ComponentError,
    #[error("API error")]
    #[diagnostic(transparent)]
    ApiError(#[from] ralertsinua_http::ApiError),
    #[error("unknown error")]
    Unknown,
}
