use anyhow::Context;
use axum::{AddExtensionLayer, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::config::Config;

mod error;
mod user;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let ctx = ApiContext {
        config: Arc::new(config),
        db,
    };

    let app = router().layer(AddExtensionLayer::new(ctx));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn router() -> Router {
    user::router()
}
