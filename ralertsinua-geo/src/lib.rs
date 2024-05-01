pub mod map;

#[derive(thiserror::Error, Debug)]
pub enum GeoError {
    #[error("Unknown error Geo")]
    Unknown,
}

pub use map::*;
pub use GeoError::*;