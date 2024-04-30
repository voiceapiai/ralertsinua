use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum AiUaError {
    #[error("Custom unknown error")]
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct CommonError {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}