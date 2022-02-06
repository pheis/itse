use crate::api::serve;
use anyhow::Context;
use clap::Parser;
use config::Config;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod api;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::parse();
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    serve(config, db).await?;

    Ok(())
}
