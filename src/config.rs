#![allow(deprecated)]
use color_eyre::eyre::{Error, Result};
use config::Config;
use lazy_static::lazy_static;
use std::sync::RwLock;
use crate::utils::*;


const FILE_NAME: &str = "config.toml";
// const DEFAULT_CONFIG: &str = include_str!("../.config/config.toml");

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::builder()
        // .set_default(key, value)
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(".config/config.toml"))
        // Add in
        .add_source(config::File::from(get_config_dir().join(FILE_NAME)))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("ALERTSINUA"))
        .build()
        .unwrap());
}

pub fn toggle_locale() -> Result<()> {
    let locale = CONFIG.read().unwrap().get::<String>("settings.locale")?;
    let new_locale = if locale == "en" {
        "uk".to_string()
    } else {
        "en".to_string()
    };
    CONFIG.write().unwrap().set("settings.locale", new_locale)?;
    Ok(())
}
