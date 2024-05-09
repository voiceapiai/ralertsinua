#![allow(unused_variables)]
pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod fs;
pub mod layout;
pub mod mode;
pub mod tui;
pub mod utils;

rust_i18n::i18n!();

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::{eyre, Result};
use ralertsinua_geo::*;
use ralertsinua_http::*;
use std::io::{stdin, stdout, Write};
use std::sync::Arc;
use tracing::{debug, error, warn};
use tui_logger::set_level_for_target;

use crate::{
    app::App,
    config::{Config, ConfigService},
    utils::{initialize_logging, initialize_panic_handler},
};

async fn tokio_main() -> Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging()?;
    set_level_for_target("app", log::LevelFilter::Debug);
    debug!(target:"app", "initialized logging");
    initialize_panic_handler()?;

    let mut config = Config::init().map_err(|e| eyre!(e))?;
    let args = Cli::parse();

    if config.token().is_empty() {
        warn!(target: "app", "token is empty, asking user for token");
        print!("enter your 'alerts.in.ua' token: ");
        stdout().flush()?;

        let mut token = String::new();
        stdin().read_line(&mut token)?;
        let token = token.trim().to_string();

        if token.is_empty() {
            error!(target: "app", "token cannot be empty, exiting");
            return Err(eyre!("token cannot be empty"));
        } else {
            debug!(target: "app", "token from user input accepted");
            std::env::set_var("ALERTSINUA_TOKEN", &token);
            config.set_token(token);
        }
    }
    debug!(target: "app", "\n{:?} \n\n-----------", config.settings());

    let config: Arc<dyn ConfigService> = Arc::new(config);
    let api_client: Arc<dyn AlertsInUaApi> =
        Arc::new(AlertsInUaClient::new(config.base_url(), config.token()));
    let geo_client: Arc<dyn AlertsInUaGeo> = Arc::new(AlertsInUaGeoClient::default());

    let mut app = App::new(config.clone(), api_client.clone(), geo_client.clone())?;
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
