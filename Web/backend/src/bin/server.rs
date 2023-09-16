use anyhow::Result;
use dotenv::dotenv;
use metaversitas::backend::{AppState, Backend};
use metaversitas::config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::init();
    let socket =
        SocketAddr::from_str(format!("{}:{}", &config.web_app_host, &config.web_app_port).as_str())
            .unwrap();
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

    // sqlx::migrate!("./migrations/").run(&db_pool).await?;
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
