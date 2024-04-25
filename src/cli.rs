// use std::path::PathBuf;
use clap::Parser;

use crate::utils::version;

#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    #[arg(short, long, value_name = "TOKEN", help = "API token provided by alerts.in.ua", default_value_t = dotenvy_macro::dotenv!("ALERTSINUA_TOKEN").to_string())]
    pub token: String,

    #[arg(short, long, value_name = "LOCALE", help = "Locale", default_value_t = String::from("en"))]
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
