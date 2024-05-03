#![allow(unused_variables)]
pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod data;
pub mod fs;
pub mod layout;
pub mod mode;
pub mod services;
pub mod tui;
pub mod ukraine;
pub mod utils;

rust_i18n::i18n!();

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use ralertsinua_http::AlertsInUaClient;
use services::{alerts::*, geo::*};
use std::sync::{Arc, RwLock};
use tracing::info;

use crate::{
    app::App,
    config::{Config, ConfigService},
    data::*,
    ukraine::Ukraine,
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

    let pool = db_pool().await?;
    let client = AlertsInUaClient::new(config.base_url(), config.token());

    let data_repository: Arc<dyn DataRepository> =
        Arc::new(DataRepositoryInstance::new(pool, client));
    let alerts_service: Arc<dyn AlertService> =
        Arc::new(AlertServiceImpl::new(data_repository.clone()));
    let geo_service: Arc<dyn GeoService> = Arc::new(GeoServiceImpl::new(data_repository.clone()));

    let regions = data_repository.fetch_regions().await?;
    let ukraine = Arc::new(RwLock::new(Ukraine::new(regions)));

    let mut app = App::new(
        config.clone(),
        ukraine.clone(),
        alerts_service.clone(),
        geo_service.clone(),
    )?;
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
