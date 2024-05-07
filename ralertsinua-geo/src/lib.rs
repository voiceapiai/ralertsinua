pub mod client;
pub mod constants;
pub mod utils;

#[derive(thiserror::Error, Debug)]
pub enum GeoError {
    #[error("Unknown error Geo")]
    Unknown,
}

pub use client::*;
pub use constants::*;
pub use utils::*;
pub use GeoError::*;
