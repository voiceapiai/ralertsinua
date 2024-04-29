#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod action;
pub mod alerts;
pub mod app;
pub mod api;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod data;
pub mod error;
pub mod mode;
pub mod tui;
pub mod ukraine;
pub mod utils;

rust_i18n::i18n!();

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

use crate::{
    api::AlertsInUaClient,
    app::App,
    // config::{Config, CONFIG},
    data::*,
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
    let data_repository = DataRepository::new(pool, client);

    let args = Cli::parse();
    let mut app = App::new(args, data_repository)?;
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
