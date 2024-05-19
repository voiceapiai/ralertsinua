use crate::utils::version;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "TOKEN",
        help = "API token provided by alerts.in.ua",
        default_value = "",
        required = false
    )]
    pub token: String,

    #[arg(short, long, value_name = "LOCALE", help = "Locale", required = false, value_parser = get_available_locales(), default_value = get_default_locale())]
    pub locale: String,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Tick rate, i.e. number of ticks per second",
        default_value_t = 1.0
    )]
    pub tick_rate: f64,

    #[arg(
        long,
        value_name = "FLOAT",
        help = "Frame rate, i.e. number of frames per second",
        default_value_t = 1.0
    )]
    pub frame_rate: f64,
}

#[inline]
fn get_available_locales() -> Vec<&'static str> {
    let locales = rust_i18n::available_locales!();
    if locales.is_empty() {
        return vec!["en", "uk"];
    }

    locales
}

#[inline]
fn get_default_locale() -> String {
    match sys_locale::get_locale() {
        Some(sl) => sl[..2].to_string(),
        None => "en".to_string(),
    }
}
