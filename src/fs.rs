use miette::{IntoDiagnostic, Result, WrapErr};
use std::fs::{read_to_string, File};

// use rust_embed::RustEmbed;
// #[derive(RustEmbed)]
// #[folder = "assets/"]
// pub struct Asset;

#[inline]
pub fn open_file(file_path: &str) -> Result<File> {
    File::open(file_path)
        .into_diagnostic()
        .wrap_err(format!("Error opening file, {}", file_path))
}

#[inline]
pub fn read_file_into_string(file_path: &str) -> Result<String> {
    read_to_string(file_path)
        .into_diagnostic()
        .wrap_err(format!("Error opening file, {}", file_path))
}
