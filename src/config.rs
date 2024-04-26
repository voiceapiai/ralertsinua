#![allow(deprecated)]
#![allow(non_camel_case_types)]
use color_eyre::eyre::WrapErr;
use color_eyre::eyre::{Error, Result};
use config::{Config, ValueKind};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::RwLock;
use strum_macros::{AsRefStr, Display};

use crate::utils::*;

const FILE_NAME: &str = "config.toml";

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::builder()
        // .set_default(key, value)
        // Add in `./Settings.toml`
        .add_source(config::File::from(Path::new(".data").join(FILE_NAME)))
        // Add in
        .add_source(config::File::from(get_config_dir().join(FILE_NAME)).required(false))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("ALERTSINUA"))
        .build()
        .wrap_err("Error loading configuration, {}")
        .unwrap());
}

pub fn toggle_locale() -> Result<()> {
    let locale = get_locale()?;
    CONFIG.write().unwrap().set("settings.locale", {
        if locale == Locale::en {
            Locale::uk
        } else {
            Locale::en
        }
    })?;
    Ok(())
}

pub fn get_locale() -> Result<Locale> {
    let locale = CONFIG.read().unwrap().get::<Locale>("settings.locale")?;
    Ok(locale)
}

#[derive(AsRefStr, Display, Debug, PartialEq, Deserialize, Serialize)]
pub enum Locale {
    en,
    uk,
}

impl FromStr for Locale {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Locale::en),
            "uk" => Ok(Locale::uk),
            _ => Err(Error::msg("Invalid locale code")),
        }
    }
}

impl Into<ValueKind> for Locale {
    fn into(self) -> ValueKind {
        match self {
            Locale::en => ValueKind::String(Locale::en.to_string()),
            Locale::uk => ValueKind::String(Locale::uk.to_string()),
        }
    }
}
