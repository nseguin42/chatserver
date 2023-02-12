#![allow(dead_code)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use crate::config::Config;
use crate::error::Error;

mod api;
mod config;
mod dal;
mod error;
mod logger;
mod models;
mod server;
mod utils;

#[actix_rt::main]
async fn main() -> () {
    if let Err(err) = run().await {
        handle_error(err);
    }
}

fn handle_error(err: Error) {
    error!("{}", err);
    panic!("Caught error: {}", err);
}

async fn run() -> Result<(), Error> {
    let config = Config::load("config.json").await?;
    logger::setup_logger(&config).expect("Could not initialize logger");
    info!("Starting server");

    api::start(&config).await?;

    Ok(())
}
