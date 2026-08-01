use std::{env, net::IpAddr};

use rocket::Config;

pub struct AppConfig {
    pub database_url: String,
    pub secret_key: String,
    pub rocket: Config,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),

            secret_key: env::var("SECRET_KEY")
                .expect("SECRET_KEY must be set"),

            rocket: Config {
                port: env::var("PORT")
                    .expect("PORT must be set")
                    .parse()
                    .expect("Invalid PORT"),

                address: env::var("IP_ADDR")
                    .expect("IP_ADDR must be set")
                    .parse::<IpAddr>()
                    .expect("Invalid IP_ADDR"),

                ..Config::debug_default()
            },
        }
    }
}
