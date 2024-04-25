#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod action;
pub mod alerts;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod constants;
pub mod data;
pub mod mode;
pub mod tui;
pub mod ukraine;
pub mod utils;


use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;
use std::sync::Arc;

use crate::{
    app::App,
    data::*,
    utils::{initialize_logging, initialize_panic_handler, version},
};

async fn tokio_main() -> Result<()> {
    dotenvy::dotenv().ok();
    initialize_logging()?;
    initialize_panic_handler()?;

    let pool = db_pool().await;
    let data_repository = Arc::new(DataRepository::new(pool));

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
