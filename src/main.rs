#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod error;
pub mod fs;
pub mod layout;
pub mod mode;
pub mod tui;
pub mod tui_helpers;
pub mod utils;

rust_i18n::i18n!();

use clap::Parser;
use cli::Cli;
#[allow(unused_imports)]
use miette::{miette, IntoDiagnostic, Result};
use ralertsinua_geo::*;
use ralertsinua_http::*;
use std::sync::Arc;
#[allow(unused_imports)]
use std::{
    io::{stdin, stdout, Write},
    time::Duration,
};
use tracing::{debug, error, warn};
use tui_logger::set_level_for_target;

use crate::{app::App, config::Config, utils::*};

async fn tokio_main() -> Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging()?;
    set_level_for_target("app", log::LevelFilter::Debug);
    debug!(target:"app", "initialized logging");
    initialize_panic_handler()?;

    let mut config = Config::default();
    let args = Cli::parse();

    if config.token().is_empty() {
        warn!(target: "app", "token is empty, asking user for token");
        print!("enter your 'alerts.in.ua' token: ");
        stdout().flush().into_diagnostic()?;

        let mut token = String::new();
        stdin().read_line(&mut token).into_diagnostic()?;
        let token = token.trim().to_string();

        if token.is_empty() {
            error!(target: "app", "token cannot be empty, exiting");
            return Err(miette!("token cannot be empty"));
        } else {
            debug!(target: "app", "token from user input accepted");
            config.set_token(token)?;
        }
    } else if !args.token.is_empty() {
        debug!(target: "app", "token from parameters accepted, ignore env");
        config.set_token(args.token)?;
    }

    // Replace with a reliable public server (e.g., 8.8.8.8:53)
    match std::net::TcpStream::connect("8.8.8.8:53") {
        Ok(_) => {
            debug!(target: "app", "sucsessful ping 8.8.8.8:53, online=true");
            config.set_online(true);
        }
        Err(e) => {
            error!(target: "app", "failed to ping 8.8.8.8:53, online=false, error={}", e);
            config.set_online(false);
        }
    };

    debug!(target: "app", "\n{:?} \n\n-----------", config.settings());

    let directory = get_data_dir().join("cache");
    std::fs::create_dir_all(directory.clone()).into_diagnostic()?;
    let api_client: Arc<dyn AlertsInUaApi> = Arc::new(AlertsInUaClient::new(
        config.base_url(),
        config.token(),
        None,
    ));
    let geo_client: Arc<dyn AlertsInUaGeo> = Arc::new(AlertsInUaGeoClient::default());

    let mut app = App::new(config, api_client.clone(), geo_client.clone())?;
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} panicked: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
