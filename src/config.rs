use delegate::delegate;
use dotenv_config::EnvConfig;
use getset::{Getters, Setters};
use icu_locid::subtags::{language, Language};
#[allow(unused_imports)]
use miette::{miette, Error, IntoDiagnostic, Result};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, string::ToString};
use tracing::warn;

#[allow(unused_imports)]
use crate::error::*;

#[derive(Debug, Clone, EnvConfig, Getters, Setters)]
pub struct Config {
    // pub keybindings: HashMap<String, String>, // FIXME: fails with new EnvConfig derive
    #[getset(get = "pub")]
    settings: Settings,
    #[env_config(default = false)]
    #[getset(get = "pub", set = "pub")]
    online: bool,
}

#[derive(Debug, Deserialize, Clone, EnvConfig, Getters, Setters, Serialize)]
pub struct Settings {
    #[env_config(name = "ALERTSINUA_BASE_URL", default = "https://api.alerts.in.ua")]
    #[getset(get = "pub", set = "pub")]
    pub base_url: String,
    #[env_config(name = "ALERTSINUA_TOKEN", default = "")]
    #[getset(get = "pub")]
    pub token: String,
    #[env_config(name = "ALERTSINUA_POLLING_INTERVAL_SEC", default = 30)]
    #[getset(get = "pub")]
    pub polling_interval: u32,
    #[env_config(name = "LOG_FILE", default = "")]
    #[getset(get = "pub", set = "pub")]
    pub log_file: String,
    /// [`Language`] represents a Unicode base language code conformant to the
    /// [`unicode_language_id`] field of the Language and Locale Identifier.
    #[env_config(default = "en", help = "Available locales: en, uk", parse(true))]
    #[getset(get = "pub with_prefix")]
    pub locale: String, // Language, // FIXME: fails with new EnvConfig derive
    #[env_config(name = "TICK_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub tick_rate: f64,
    #[env_config(name = "FRAME_RATE", default = 1.0)]
    #[getset(get = "pub")]
    pub frame_rate: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config::init().map_err(|e| miette!(e)).unwrap()
    }
}

impl Config {
    delegate! {
        to self.settings {
            pub fn base_url(&self) -> &str;
            pub fn set_base_url(&mut self, val: String) -> &mut Settings;
            pub fn token(&self) -> &str;
            pub fn log_file(&self) -> &str;
            pub fn set_log_file(&mut self, v: String) -> &mut Settings;
            pub fn polling_interval(&self) -> &u32;
            pub fn tick_rate(&self) -> &f64;
            pub fn frame_rate(&self) -> &f64;
        }
    }

    #[inline]
    pub fn set_token(&mut self, val: String) -> Result<&mut Settings> {
        if Self::validate_token(&val) {
            self.settings.token = val;
            Ok(&mut self.settings)
        } else {
            Err(miette!("token is invalid, must be 46 characters long"))
        }
    }

    /// For example, check if the token is 46 characters long and contains only alphanumeric characters
    #[inline]
    fn validate_token(token: &str) -> bool {
        token.len() == 46 && token.chars().all(|c| c.is_alphanumeric())
    }

    #[inline]
    pub fn get_locale(&self) -> Language {
        let i18n_locale: &str = &rust_i18n::locale();
        let curr_lang: Language = i18n_locale.parse().unwrap();
        curr_lang
    }

    #[inline]
    pub fn toggle_locale(&mut self) -> &mut Settings {
        let curr_lang: Language = self.get_locale();
        let lang = if curr_lang == language!("en") {
            language!("uk")
        } else {
            language!("en")
        };
        self.set_locale(lang);
        &mut self.settings
    }

    #[inline]
    pub fn set_locale<L>(&mut self, value: L) -> &mut Settings
    where
        L: FromStr,
        Language: From<L>,
    {
        let lang: Language = value.into();
        let locales = rust_i18n::available_locales!();
        if !locales.contains(&lang.as_str()) {
            warn!("Locale '{}' is not available, using fallback 'en'", lang);
            return &mut self.settings;
        }
        rust_i18n::set_locale(lang.as_str());
        &mut self.settings
    }
}
