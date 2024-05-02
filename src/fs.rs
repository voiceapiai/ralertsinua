use color_eyre::eyre::Result;
// use directories::ProjectDirs;
// use lazy_static::lazy_static;
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use crate::utils::get_config_dir;

pub fn copy_config_files() -> Result<()> {
    let config_dir = get_config_dir();
    fs::create_dir_all(&config_dir)?;

    for file in &[
        "ukraine.csv",
        "ukraine.wkt",
        "ukraine.sqlite",
        "create_regions_table.sql",
    ] {
        let from: PathBuf = Path::new(".config").join(file);
        let to = config_dir.join(file);
        fs::copy(from, to)?;
    }

    Ok(())
}
