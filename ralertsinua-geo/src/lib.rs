pub mod client;
pub mod constants;
// pub mod grid;
pub mod location;
pub mod utils;

#[derive(thiserror::Error, Debug)]
// coveralls-ignore-next-line
pub enum GeoError {
    #[error("Unknown error Geo")]
    Unknown,
}

pub use client::*;
pub use constants::*;
pub use location::*;
pub use utils::*;
pub use GeoError::*;
