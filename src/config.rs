#![allow(non_camel_case_types)]
use async_trait::async_trait;
use color_eyre::eyre::Result;
use delegate::delegate;
use dotenv_config::EnvConfig;
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};
use std::string::ToString;
use strum_macros::{Display, EnumString};
use tracing::warn;

// const FILE_NAME: &str = "config.toml";

#[derive(Debug, Clone, EnvConfig, MutGetters)]
pub struct Config {
    // pub keybindings: HashMap<String, String>, // FIXME: fails with new EnvConfig derive
    #[getset(get_mut)]
    pub settings: Settings,
}

#[derive(Debug, Deserialize, Clone, EnvConfig, Getters, Setters, Serialize)]
pub struct Settings {
    #[env_config(name = "ALERTSINUA_BASE_URL", default = "https://api.alerts.in.ua")]
    #[getset(get = "pub", set = "pub")]
    pub base_url: String,
    #[env_config(name = "ALERTSINUA_TOKEN", default = "")]
    #[getset(get = "pub", set = "pub")]
    pub token: String,
    #[env_config(default = "en", help = "Available locales: en, uk", parse(false))]
    #[getset(get = "pub with_prefix", set = "pub")]
    pub locale: String, // Locale, // FIXME: fails with new EnvConfig derive
    #[env_config(name = "TICK_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub tick_rate: f64,
    #[env_config(name = "FRAME_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub frame_rate: f64,
}

#[async_trait]
pub trait ConfigService: Sync + Send + core::fmt::Debug {
    fn settings(&self) -> &Settings;
    fn base_url(&self) -> &str;
    fn set_base_url(&mut self, val: String) -> &mut Settings;
    fn token(&self) -> &str;
    fn set_token(&mut self, val: String) -> &mut Settings;
    fn get_locale(&self) -> String;
    fn set_locale<L>(&self, val: L) -> String
    where
        L: Into<String>,
        Self: Sized;
    fn toggle_locale(&self) -> String;
    fn tick_rate(&self) -> &f64;
    fn frame_rate(&self) -> &f64;
}

impl ConfigService for Config {
    delegate! {
        to self.settings {
            fn base_url(&self) -> &str;
            fn set_base_url(&mut self, val: String) -> &mut Settings;
            fn token(&self) -> &str;
            fn set_token(&mut self, val: String) -> &mut Settings;
            fn tick_rate(&self) -> &f64;
            fn frame_rate(&self) -> &f64;
        }
    }

    fn settings(&self) -> &Settings {
        &self.settings
    }

    fn get_locale(&self) -> String {
        rust_i18n::locale().to_string() // FIXME: dirty fix to not mutate self
    }

    fn toggle_locale(&self) -> String {
        let curr_locale = &self.get_locale();
        let locale = if curr_locale == &Locale::en.to_string() {
            Locale::uk
        } else {
            Locale::en
        };
        self.set_locale(locale.to_string());
        self.get_locale()
    }

    fn set_locale<L>(&self, locale: L) -> String
    where
        L: Into<String>,
    {
        let locales = rust_i18n::available_locales!();
        let locale: &str = &locale.into();
        if !locales.contains(&locale) {
            warn!("Locale '{}' is not available, using fallback 'en'", locale);
            return self.get_locale();
        }
        // self.settings_mut().locale = locale.to_string(); // FIXME: dirty fix to not mutate self
        rust_i18n::set_locale(locale);
        self.get_locale()
    }
}

#[derive(Default, Display, Debug, Clone, EnumString, PartialEq, Deserialize, Serialize)]
pub enum Locale {
    #[default]
    en,
    uk,
}

impl From<Locale> for String {
    fn from(val: Locale) -> Self {
        val.to_string()
    }
}
