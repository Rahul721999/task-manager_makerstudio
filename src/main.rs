// module imports
use task_manager::{load_data, start_service, AppState};

// extern crate imports
use actix_web::web;
use env_logger::Env;
use log::error;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // load data
    let app_state: web::Data<_> = web::Data::new(AppState {
        data: Mutex::new(load_data()),
    });

    let _ = start_service(app_state)
        .await
        .map_err(|err| error!("{}", err))
        .expect("failed to run server")
        .await
        .map_err(|err| error!("failed to run server: {}", err));

    Ok(())
}
