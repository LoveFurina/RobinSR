use anyhow::Result;
use tokio::net::TcpListener;
use tracing::{info_span, Instrument};

use crate::{log_error, net::PlayerSession};

pub async fn listen(host: &str, port: u16) -> Result<()> {
    let listener = TcpListener::bind(format!("{host}:{port}")).await?;
    tracing::info!("Listening at {host}:{port}");

    loop {
        let Ok((client_socket, client_addr)) = listener.accept().await else {
            continue;
        };

        let mut session = PlayerSession::new(client_socket);
        tokio::spawn(
            async move {
                log_error!(
                    "Session from {client_addr} disconnected",
                    format!("An error occurred while processing session ({client_addr})"),
                    Box::pin(session.run()).await
                );
            }
            .instrument(info_span!("session", addr = %client_addr)),
        );
    }
}
