use crate::config::Config;
use crate::route::create_router;
use axum::extract::Host;
use axum::handler::{HandlerWithoutStateExt};
use axum::http::{StatusCode, Uri};
use axum::response::Redirect;
use axum::{BoxError};
use axum_server::tls_rustls::RustlsConfig;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub struct AppState {
    pub redis: redis::Client,
    pub database: Pool<Postgres>,
    pub config: Config,
}

pub struct Backend {
    pub socket_address: SocketAddr,
    pub app_state: Arc<AppState>,
    pub rust_ls_config: RustlsConfig,
}

impl Backend {
    pub async fn run(self) {
        let app = create_router(self.app_state.clone()).await;

        tracing::info!(
            "server listening on: {}:{}",
            &self.socket_address.ip(),
            &self.socket_address.port()
        );

        let http_port = self.app_state.config.web_app_port.parse::<usize>().unwrap();
        let https_port = self
            .app_state
            .config
            .web_app_port_ssl
            .parse::<usize>()
            .unwrap();
        let web_host = self.app_state.config.web_app_host.clone();

        tokio::spawn(redirect_http_to_https(web_host, http_port, https_port));
        axum_server::bind_rustls(self.socket_address, self.rust_ls_config)
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
}

async fn redirect_http_to_https(web_host: Arc<str>, http_port: usize, https_port: usize) {
    fn make_https(
        host: String,
        uri: Uri,
        http_port: usize,
        https_port: usize,
    ) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&http_port.to_string(), &https_port.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, http_port, https_port) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr =
        SocketAddr::from_str(format!("{}:{}", web_host.to_owned(), http_port.to_owned()).as_str())
            .unwrap();

    tracing::info!(
        "redirector to ssl is listening on {}:{}",
        &addr.ip(),
        &addr.port()
    );
    axum::Server::bind(&addr)
        .serve(redirect.into_make_service())
        .await
        .unwrap();
}
