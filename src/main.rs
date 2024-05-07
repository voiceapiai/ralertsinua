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
use color_eyre::eyre::Result;
use ralertsinua_geo::*;
use ralertsinua_http::*;
use std::sync::Arc;
use tracing::info;

use crate::{
    app::App,
    config::{Config, ConfigService},
    utils::{initialize_logging, initialize_panic_handler},
};

async fn tokio_main() -> Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging()?;
    info!(target:"AlertsInUa", "Logging initialized");
    initialize_panic_handler()?;

    let config = Config::init().unwrap();
    let args = Cli::parse();
    // TODO: override config with args ?
    let config: Arc<dyn ConfigService> = Arc::new(config);
    info!("\n{:?} \n\n-----------", config.settings());

    let api_client: Arc<dyn AlertsInUaApi> =
        Arc::new(AlertsInUaClient::new(config.base_url(), config.token()));
    let geo_client: Arc<dyn AlertsInUaGeo> =
        Arc::new(AlertsInUaGeoClient::default());

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
