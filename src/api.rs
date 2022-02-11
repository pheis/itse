use anyhow::Context;
use axum::{AddExtensionLayer, Router};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use std::sync::Arc;

mod error;
mod extractor;
mod user;

pub type Result<T, E = error::Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    decoding_key: Arc<DecodingKey>,
    encoding_key: Arc<EncodingKey>,
    db: PgPool,
}

pub async fn serve(
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    db: PgPool,
    port: u16,
) -> anyhow::Result<()> {
    let ctx = ApiContext {
        encoding_key: Arc::new(encoding_key),
        decoding_key: Arc::new(decoding_key),
        db,
    };

    let app = router().layer(AddExtensionLayer::new(ctx));

    axum::Server::bind(&format!("0.0.0.0:{port}").parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn router() -> Router {
    user::router()
}
