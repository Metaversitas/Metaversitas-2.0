use crate::config::Config;
use crate::route::create_router;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

pub struct AppState {
    pub redis: redis::Client,
    pub database: Pool<Postgres>,
    pub config: Config,
}

pub struct Backend {
    pub socket_address: SocketAddr,
    pub app_state: Arc<AppState>,
}

impl Backend {
    pub async fn run(self) {
        let app = create_router(self.app_state.clone()).await;

        tracing::info!(
            "server listening on: {}:{}",
            &self.socket_address.ip(),
            &self.socket_address.port()
        );
        axum::Server::bind(&self.socket_address)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .unwrap();
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received! Trying to shutdown");
}
