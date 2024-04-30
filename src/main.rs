#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod action;
pub mod alerts;
pub mod api;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod data;
pub mod error;
pub mod fs;
pub mod mode;
pub mod services;
pub mod tui;
pub mod ukraine;
pub mod utils;

rust_i18n::i18n!();

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use services::{alerts::*, geo::*};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tracing::info;

use crate::{
    api::AlertsInUaClient,
    app::App,
    data::*,
    ukraine::Ukraine,
    utils::{initialize_logging, initialize_panic_handler, version},
};

async fn tokio_main() -> Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging()?;
    initialize_panic_handler()?;

    // let config: Config = CONFIG
    //     .read()
    //     .unwrap()
    //     .clone()
    //     .try_deserialize()
    //     .map_err(|e| color_eyre::eyre::eyre!("Failed to deserialize config: {}", e))
    //     .unwrap();
    // info!("\n{:?} \n\n-----------", config);

    let pool = db_pool().await?;
    let client = AlertsInUaClient::default();

    let data_repository: Arc<dyn DataRepository> =
        Arc::new(DataRepositoryInstance::new(pool, client));
    let alerts_service: Arc<dyn AlertService> =
        Arc::new(AlertServiceImpl::new(data_repository.clone()));
    let geo_service: Arc<dyn GeoService> =
        Arc::new(GeoServiceImpl::new(data_repository.clone()));

    let regions = data_repository.fetch_regions().await?;
    let ukraine = Arc::new(RwLock::new(Ukraine::new(regions)));

    let args = Cli::parse();
    let mut app = App::new(args, ukraine, alerts_service, geo_service)?;
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
