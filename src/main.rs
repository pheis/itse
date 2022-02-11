use anyhow::Context;
use clap::Parser;
use dotenv::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::postgres::PgPoolOptions;

mod api;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config::Config {
        private_key,
        public_key,
        database_url,
        port,
    } = config::Config::parse();

    let encoding_key =
        EncodingKey::from_ed_pem(&private_key.as_bytes()).context("Could not parse private_key")?;

    let decoding_key =
        DecodingKey::from_ed_pem(&public_key.as_bytes()).context("Could not parse public_key")?;

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .context("could not connect to database_url")?;

    api::serve(encoding_key, decoding_key, db, port).await?;

    Ok(())
}
