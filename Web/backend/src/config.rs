#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::r#const::{
    ENV_DATABASE_URL, ENV_ENVIRONMENT, ENV_JWT_EXPIRED_IN, ENV_JWT_MAX_AGE, ENV_JWT_SECRET,
    ENV_MINIO_ACCESS_KEY, ENV_MINIO_BUCKET_NAME, ENV_MINIO_BUCKET_REGION, ENV_MINIO_HOST_URL,
    ENV_MINIO_SECRET_KEY, ENV_REDIS_HOSTNAME, ENV_REDIS_IS_TLS, ENV_REDIS_PORT, ENV_WEB_APP_HOST,
    ENV_WEB_APP_MODE_TLS, ENV_WEB_APP_PORT, ENV_WEB_APP_PORT_SSL,
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Config {
    pub database_url: Arc<str>,
    pub jwt_secret: Arc<str>,
    pub jwt_expires_in: Arc<str>,
    pub jwt_max_age: Arc<i32>,
    pub redis_host_name: Arc<str>,
    pub redis_port: Arc<usize>,
    pub redis_is_tls: Arc<bool>,
    pub web_app_port: Arc<str>,
    pub web_app_port_ssl: Arc<str>,
    pub web_app_host: Arc<str>,
    pub web_app_mode_tls: Arc<bool>,
    pub web_app_environment: Arc<str>,
    pub minio_bucket_name: Arc<str>,
    pub minio_access_key: Arc<str>,
    pub minio_secret_key: Arc<str>,
    pub minio_host_url: Arc<str>,
    pub minio_bucket_region: Arc<str>,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var(ENV_DATABASE_URL)
            .expect("DATABASE_URL must be set")
            .into();
        let jwt_secret = std::env::var(ENV_JWT_SECRET)
            .expect("JWT_SECRET must be set")
            .into();
        let jwt_expires_in = std::env::var(ENV_JWT_EXPIRED_IN)
            .expect("JWT_EXPIRED_IN must be set")
            .into();
        let jwt_max_age = std::env::var(ENV_JWT_MAX_AGE)
            .expect("JWT_MAX_AGE must be set")
            .parse::<i32>()
            .unwrap()
            .into();
        let redis_host_name = std::env::var(ENV_REDIS_HOSTNAME)
            .expect("REDIS_HOSTNAME must be set")
            .into();
        let redis_port = std::env::var(ENV_REDIS_PORT)
            .expect("REDIS_PORT must be set")
            .parse::<usize>()
            .unwrap()
            .into();
        let redis_is_tls = std::env::var(ENV_REDIS_IS_TLS)
            .expect("REDIS_IS_TLS must be set")
            .parse::<bool>()
            .unwrap()
            .into();
        let web_app_port = std::env::var(ENV_WEB_APP_PORT)
            .expect("WEB_APP_PORT must be set")
            .parse::<String>()
            .unwrap()
            .as_str()
            .into();
        let web_app_port_ssl = std::env::var(ENV_WEB_APP_PORT_SSL)
            .expect("WEB_APP_PORT_SSL must be set")
            .parse::<String>()
            .unwrap()
            .as_str()
            .into();
        let web_app_host = std::env::var(ENV_WEB_APP_HOST)
            .expect("WEB_APP_HOST must be set")
            .parse::<String>()
            .unwrap()
            .as_str()
            .into();
        let web_app_mode_tls = std::env::var(ENV_WEB_APP_MODE_TLS)
            .expect("ENV WEB_APP_MODE must be set")
            .parse::<bool>()
            .unwrap()
            .into();
        let web_app_environment = std::env::var(ENV_ENVIRONMENT)
            .unwrap_or("PROD".to_string())
            .into();
        let minio_bucket_name = std::env::var(ENV_MINIO_BUCKET_NAME)
            .expect("ENV MINIO_BUCKET_NAME must be set")
            .into();
        let minio_access_key = std::env::var(ENV_MINIO_ACCESS_KEY)
            .expect("ENV MINIO_ACCESS_KEY must be set")
            .into();
        let minio_secret_key = std::env::var(ENV_MINIO_SECRET_KEY)
            .expect("ENV MINIO_SECRET_KEY must be set")
            .into();
        let minio_host_url = std::env::var(ENV_MINIO_HOST_URL)
            .expect("ENV MINIO_HOST_URL must be set")
            .into();
        let minio_bucket_region = std::env::var(ENV_MINIO_BUCKET_REGION)
            .unwrap_or("".to_string())
            .into();

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
            redis_host_name,
            redis_port,
            redis_is_tls,
            web_app_port,
            web_app_port_ssl,
            web_app_host,
            web_app_mode_tls,
            web_app_environment,
            minio_bucket_name,
            minio_access_key,
            minio_secret_key,
            minio_host_url,
            minio_bucket_region,
        }
    }
}
