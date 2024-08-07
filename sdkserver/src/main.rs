use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use logging::init_tracing;
use services::{auth, dispatch, errors};
use tracing::Level;

mod config;
mod logging;
mod services;

const PORT: u16 = 21000;

pub fn info_supremacy() {
    println!("\n这个本地端是免费的，如果你是买来的说明你被骗了，请立刻去退款");
}

#[tokio::main]
async fn main() -> Result<()> {
    info_supremacy();
    init_tracing();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let router = Router::new()
        .route(
            dispatch::QUERY_DISPATCH_ENDPOINT,
            get(dispatch::query_dispatch),
        )
        .route(
            dispatch::QUERY_GATEWAY_ENDPOINT,
            get(dispatch::query_gateway),
        )
        .route(auth::RISKY_API_CHECK_ENDPOINT, post(auth::risky_api_check))
        .route(
            auth::LOGIN_WITH_PASSWORD_ENDPOINT,
            post(auth::login_with_password),
        )
        .route(
            auth::LOGIN_WITH_SESSION_TOKEN_ENDPOINT,
            post(auth::login_with_session_token),
        )
        .route(
            auth::GRANTER_LOGIN_VERIFICATION_ENDPOINT,
            post(auth::granter_login_verification),
        )
        .fallback(errors::not_found);

    let addr = format!("127.0.0.1:{PORT}");
    let server = axum_server::bind(addr.parse()?);

    tracing::info!("sdkserver is listening at {addr}");
    server.serve(router.into_make_service()).await?;

    Ok(())
}
