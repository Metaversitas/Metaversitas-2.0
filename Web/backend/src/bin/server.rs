#![allow(clippy::unwrap_used, clippy::expect_used)]

use anyhow::Result;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use metaversitas::backend::{AppState, Backend};
use metaversitas::config::Config;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::init();
    let socket = if *config.web_app_mode_tls {
        SocketAddr::from_str(
            format!("{}:{}", &config.web_app_host, &config.web_app_port_ssl).as_str(),
        )
        .unwrap()
    } else {
        SocketAddr::from_str(format!("{}:{}", &config.web_app_host, &config.web_app_port).as_str())
            .unwrap()
    };

    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap();

    let redis_uri_scheme = if *config.redis_is_tls {
        "rediss".to_string()
    } else {
        "redis".to_string()
    };

    let redis_conn_url = format!(
        "{}://{}:{}",
        redis_uri_scheme, &config.redis_host_name, &config.redis_port
    );
    let redis_client = redis::Client::open(redis_conn_url).unwrap();
    let redis_connection_manager = redis_client
        .get_tokio_connection_manager()
        .await
        .expect("Can't create Redis connection manager");

    let minio_bucket = Bucket::new(
        config.minio_bucket_name.as_ref(),
        Region::Custom {
            region: config.minio_bucket_region.to_string(),
            endpoint: config.minio_host_url.to_string(),
        },
        Credentials::new(
            Some(config.minio_access_key.as_ref()),
            Some(config.minio_secret_key.as_ref()),
            None,
            None,
            None,
        )?,
    )
    .expect("Unable to create a S3 Bucket")
    .with_path_style();

    let ssl_config = RustlsConfig::from_pem_file(
        std::env::current_dir()
            .unwrap()
            .join("certs")
            .join("certificate.pem"),
        std::env::current_dir()
            .unwrap()
            .join("certs")
            .join("private.pem"),
    )
    .await
    .unwrap();

    sqlx::migrate!("./migrations/").run(&db_pool).await?;
    let app_state = Arc::new(AppState {
        redis: redis_connection_manager,
        database: db_pool,
        config,
        bucket: minio_bucket,
    });
    let server_backend = Backend {
        socket_address: socket,
        app_state,
        rust_ls_config: ssl_config,
    };
    server_backend.run().await;
    Ok(())
}
