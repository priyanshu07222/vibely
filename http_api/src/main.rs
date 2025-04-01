use config::Config;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
mod config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Hello, world!");

    let config: Config = Config::init();
    let pool = PgPoolOptions::new().max_connections(5).connect(&config.database_url).await.expect("failed to connect to databased");
    println!("Connected to the databased");
}
