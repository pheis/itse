use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::routing::post;
use axum::Router;
use axum::{extract::Extension, Json};
use uuid::Uuid;

use crate::api::error::{Error, ResultExt};
use crate::api::{ApiContext, Result};

pub fn router() -> Router {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CreatedUser {
    user_id: Uuid,
    username: String,
    email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct NewUser {
    username: String,
    email: String,
    password: String,
}

async fn create_user(
    ctx: Extension<ApiContext>,
    Json(NewUser {
        username,
        email,
        password,
    }): Json<NewUser>,
) -> Result<Json<CreatedUser>> {
    let password_hash = hash_password(password).await?;

    let user_id = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        username,
        email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::conflict([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::conflict([("email", "email taken")])
    })?;

    Ok(Json(CreatedUser {
        email,
        username,
        user_id,
    }))
}

#[derive(serde::Serialize, serde::Deserialize)]
enum LoginUser {
    Username { username: String, password: String },
    Email { email: String, password: String },
}

async fn login_user(ctx: Extension<ApiContext>, Json(creds): Json<LoginUser>) -> Result<()> {
    let query = match &creds {
        LoginUser::Username { username, .. } => sqlx::query_scalar!(
            r#"select (password_hash) from "user" where username=$1"#,
            username,
        ),
        LoginUser::Email { email, .. } => sqlx::query_scalar!(
            r#"select (password_hash) from "user" where email=$1"#,
            email,
        ),
    };

    let password_hash = query.fetch_one(&ctx.db).await?;

    let password = match creds {
        LoginUser::Username { password, .. } | LoginUser::Email { password, .. } => password,
    };

    verify_password(password, password_hash).await?;

    Ok(())
}

async fn hash_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("failed to parse password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")?
}
