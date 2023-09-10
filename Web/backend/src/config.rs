use crate::r#const::{
    ENV_DATABASE_URL, ENV_JWT_EXPIRED_IN, ENV_JWT_MAX_AGE, ENV_JWT_SECRET, ENV_REDIS_HOSTNAME,
    ENV_REDIS_IS_TLS, ENV_REDIS_PORT,
};

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i32,
    pub redis_host_name: String,
    pub redis_port: usize,
    pub redis_is_tls: bool,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var(ENV_DATABASE_URL).expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var(ENV_JWT_SECRET).expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var(ENV_JWT_EXPIRED_IN).expect("JWT_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var(ENV_JWT_MAX_AGE)
            .expect("JWT_MAX_AGE must be set")
            .parse::<i32>()
            .unwrap();
        let redis_host_name =
            std::env::var(ENV_REDIS_HOSTNAME).expect("REDIS_HOSTNAME must be set");
        let redis_port = std::env::var(ENV_REDIS_PORT)
            .expect("REDIS_PORT must be set")
            .parse::<usize>()
            .unwrap();
        let redis_is_tls = std::env::var(ENV_REDIS_IS_TLS)
            .expect("REDIS_IS_TLS must be set")
            .parse::<bool>()
            .unwrap();

        Config {
            database_url,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age,
            redis_host_name,
            redis_port,
            redis_is_tls,
        }
    }
}
