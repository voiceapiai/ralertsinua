use crate::api::API_BASE_URL;
use crate::utils::version;
use clap::{Arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(short, long, value_name = "BASE_URL", help = "API base URL", default_value_t = String::from(API_BASE_URL), required = false)]
    pub base_url: String,

    #[arg(
        short,
        long,
        value_name = "TOKEN",
        help = "API token provided by alerts.in.ua",
        default_value = "",
        required = false
    )]
    pub token: String,

    #[arg(short, long, value_name = "LOCALE", help = "Locale", required = false, value_parser = rust_i18n::available_locales!(), default_value = sys_locale::get_locale().unwrap()[..2].to_string())]
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
