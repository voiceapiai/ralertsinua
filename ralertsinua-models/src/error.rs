use thiserror::Error;

/// Groups up the kinds of errors that may happen in this crate.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}
