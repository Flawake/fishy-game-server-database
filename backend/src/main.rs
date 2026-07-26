use crate::config::AppConfig;

extern crate rocket;

// Import all the different layers that make up the backend.
pub mod app;
pub mod config;
pub mod controller;
pub mod docs;
pub mod domain;
pub mod entity;
pub mod fish_catalog;
pub mod repository;
pub mod service;
pub mod state;
pub mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = AppConfig::from_env();

    app::build(config)
        .await?
        .launch()
        .await?;

    Ok(())
}
