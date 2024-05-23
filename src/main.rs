#![allow(unused)]

// module imports
mod routes;
mod schema;
mod server;

use crate::schema::{load_data, AppStateData};
use log::error;
use server::start_service;

// extern crate imports
use actix_web::web;
use env_logger::Env;
use std::sync::Mutex;

// AppState
pub struct AppState {
    data: Mutex<AppStateData>,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // load data
    let app_state: web::Data<_> = web::Data::new(AppState {
        data: Mutex::new(load_data()),
    });

    let server = start_service(app_state)
        .await
        .expect("failed to run server");
    if let Err(err) = server.await {
        error!("failed to run server: {}", err);
    };
    Ok(())
}
