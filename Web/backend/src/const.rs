use sqlx::{Postgres, Transaction};

pub const ENV_ENVIRONMENT: &str = "ENVIRONMENT";
pub const ENV_ENVIRONMENT_PRODUCTION: &str = "PROD";
pub const ENV_ENVIRONMENT_DEVELOPMENT: &str = "DEV";
pub const ENV_DATABASE_URL: &str = "DATABASE_URL";
pub const ENV_JWT_SECRET: &str = "JWT_SECRET";
pub const ENV_JWT_EXPIRED_IN: &str = "JWT_EXPIRED_IN";
pub const ENV_JWT_MAX_AGE: &str = "JWT_MAX_AGE";
pub const ENV_REDIS_IS_TLS: &str = "REDIS_IS_TLS";
pub const ENV_REDIS_HOSTNAME: &str = "REDIS_HOSTNAME";
pub const ENV_REDIS_PASSWORD: &str = "REDIS_PASSWORD";
pub const ENV_REDIS_PORT: &str = "REDIS_PORT";
pub const ENV_WEB_APP_PORT: &str = "WEB_APP_PORT";
pub const ENV_WEB_APP_PORT_SSL: &str = "WEB_APP_PORT_SSL";
pub const ENV_WEB_APP_HOST: &str = "WEB_APP_HOST";
pub const ENV_WEB_APP_MODE_TLS: &str = "WEB_APP_TLS_MODE";

pub type PgTransaction = Transaction<'static, Postgres>;
