use color_eyre::eyre::{Result, WrapErr};
use serde::Deserialize;
use std::fs::{read_to_string, File};
use tracing::error;

// use rust_embed::RustEmbed;
// #[derive(RustEmbed)]
// #[folder = "assets/"]
// pub struct Asset;

#[inline]
pub fn open_file(file_path: &str) -> Result<File> {
    File::open(file_path).wrap_err(format!("Error opening file, {}", file_path))
}

#[inline]
pub fn read_csv_file_into<R>(file_path: &str) -> Result<Vec<R>>
where
    R: for<'de> Deserialize<'de> + Default,
{
    use csv::ReaderBuilder;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)
        .wrap_err("Error opening file")?;
    let data = rdr
        .deserialize::<R>()
        .map(|r| {
            let rg: R = match r {
                Ok(rg) => rg,
                Err(e) => {
                    error!("Error deserializing csv row: {}", e);
                    return R::default();
                }
            };
            rg
        })
        .collect::<Vec<R>>();

    Ok(data)
}

#[inline]
pub fn read_file_into_string(file_path: &str) -> Result<String> {
    read_to_string(file_path).wrap_err("Error opening file")
}
