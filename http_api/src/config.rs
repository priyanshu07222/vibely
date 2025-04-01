use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16
}

impl Config {
    pub fn init() -> Config {
        Config {
            database_url: env::var("DATABASE_URL").expect("Database url must be set"),
            port: 8080
        }
    }
}
