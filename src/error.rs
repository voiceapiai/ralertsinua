#[derive(thiserror::Error, Debug)]
pub enum AiUaError {
    #[error("Custom unknown error")]
    Unknown,
}
