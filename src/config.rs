#![allow(deprecated)]
#![allow(non_camel_case_types)]
use color_eyre::eyre::{Error, Result, WrapErr};
use config::{Config as ConfigRs, Value, ValueKind};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::string::ToString;
use std::sync::RwLock;
use strum_macros::{Display, EnumString};
use tracing::warn;

use crate::utils::*;

const FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub keybindings: HashMap<String, String>,
    pub settings: Settings,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub token: String,
    pub locale: Locale,
}

lazy_static! {
    /// Global `Config` instance
    ///
    /// example taken from
    pub static ref CONFIG: RwLock<ConfigRs> = RwLock::new(ConfigRs::builder()
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

pub fn set_token(token: String) -> Result<()> {
    if token.is_empty() {
        return Err(Error::msg("Token cannot be empty"));
    }
    if token.len() != 46 {
        return Err(Error::msg(format!(
            "Token must be 32 characters long, but {} characteers provided",
            token.len()
        )));
    }
    CONFIG
        .write()
        .map_err(|_| color_eyre::eyre::eyre!("Failed to acquire write lock on CONFIG"))?
        .set("settings.token", ValueKind::String(token))?;
    Ok(())
}

pub fn toggle_locale() -> Result<()> {
    let curr_locale = get_locale()?;
    let locale = if curr_locale == Locale::en {
        Locale::uk
    } else {
        Locale::en
    };
    set_locale(locale)?;
    Ok(())
}

pub fn get_locale() -> Result<Locale> {
    let locale = CONFIG
        .read()
        .map_err(|_| color_eyre::eyre::eyre!("Failed to acquire read lock on CONFIG"))?
        .get::<Locale>("settings.locale")?;
    Ok(locale)
}

/// Set locale to `Config` and `rust_i18n`
///
/// Will accept `Locale` enum or `&str` or `String`
pub fn set_locale(locale: impl Into<String>) -> Result<()> {
    let locales = rust_i18n::available_locales!();
    let locale: &str = &locale.into();
    if !locales.contains(&locale) {
        warn!("Locale '{}' is not available, using fallback 'en'", locale);
        Ok(())
    } else {
        CONFIG
            .write()
            .map_err(|_| color_eyre::eyre::eyre!("Failed to acquire write lock on CONFIG"))?
            .set("settings.locale", locale)?;
        rust_i18n::set_locale(locale);
        Ok(())
    }
}

#[derive(Display, Debug, EnumString, PartialEq, Deserialize, Serialize)]
pub enum Locale {
    en,
    uk,
}

impl Into<ValueKind> for Locale {
    fn into(self) -> ValueKind {
        ValueKind::String(self.to_string())
    }
}

impl Into<String> for Locale {
    fn into(self) -> String {
        self.to_string()
    }
}
