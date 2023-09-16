use crate::r#const::{
    ENV_DATABASE_URL, ENV_JWT_EXPIRED_IN, ENV_JWT_MAX_AGE, ENV_JWT_SECRET, ENV_REDIS_HOSTNAME,
    ENV_REDIS_IS_TLS, ENV_REDIS_PORT, ENV_WEB_APP_HOST, ENV_WEB_APP_PORT,
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
    pub web_app_host: Arc<str>,
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
        let web_app_host = std::env::var(ENV_WEB_APP_HOST)
            .expect("WEB_APP_HOST must be set")
            .parse::<String>()
            .unwrap()
            .as_str()
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
            web_app_host,
        }
    }
}
