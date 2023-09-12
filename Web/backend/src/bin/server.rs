use anyhow::Result;
use dotenv::dotenv;
use metaversitas::backend::{AppState, Backend};
use metaversitas::config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::init();
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888);
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap();
    let redis_uri_scheme = match config.redis_is_tls {
        true => "rediss".to_string(),
        false => "redis".to_string(),
    };
    let redis_conn_url = format!(
        "{}://{}:{}",
        redis_uri_scheme, &config.redis_host_name, &config.redis_port
    );
    let redis_client = redis::Client::open(redis_conn_url).unwrap();

    sqlx::migrate!("./migrations/").run(&db_pool).await?;
    let app_state = Arc::new(AppState {
        redis: redis_client,
        database: db_pool,
        config,
    });
    let server_backend = Backend {
        socket_address: socket,
        app_state,
    };
    server_backend.run().await;
    Ok(())
}
