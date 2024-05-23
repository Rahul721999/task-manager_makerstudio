#![allow(unused)]

// module imports
mod routes;
mod schema;
use crate::routes::*;
use crate::schema::*;

// extern crate imports
use log::{info, warn, error};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    Ok(())
}
