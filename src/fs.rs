use color_eyre::eyre::{Result, WrapErr};
use serde::Deserialize;
use std::{fs::File, io::Read};
use tracing::error;

#[tracing::instrument]
pub fn open_file(file_path: &str) -> Result<File> {
    return File::open(file_path).wrap_err("Error opening file, {}");
}

#[tracing::instrument]
pub fn read_csv_file_into<R>(file_path: &str) -> Result<Vec<R>>
where
    R: for<'de> Deserialize<'de> + Default,
{
    use csv::ReaderBuilder;
    let file = open_file(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
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

pub fn read_wkt_file(file_path: &str) -> Result<String> {
    let mut file = open_file(file_path)?;
    let mut wkt_string = String::new();
    file.read_to_string(&mut wkt_string)?;

    Ok(wkt_string)
}
