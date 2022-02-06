use anyhow::Context;

use axum::{AddExtensionLayer, Router};
use std::sync::Arc;

use sqlx::PgPool;

use crate::config::Config;

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

    let app = Router::new().layer(AddExtensionLayer::new(ctx));

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}
