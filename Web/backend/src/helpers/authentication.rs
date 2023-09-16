use crate::backend::AppState;
use crate::helpers::errors::AuthError;
use crate::model::user::{SessionTokenClaims, User};
use axum::extract::State;
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderValue, Request};
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use rand::Rng;
use redis::{AsyncCommands, Value};
use serde::Serialize;
use std::sync::Arc;

pub const COOKIE_SESSION_TOKEN_NAME: &str = "session_token";
pub const COOKIE_AUTH_NAME: &str = "Authorization";
pub struct SessionId(String);

impl SessionId {
    pub fn generate_new() -> Self {
        SessionId(
            rand::thread_rng()
                .sample_iter(rand::distributions::Alphanumeric)
                .take(60)
                .map(char::from)
                .collect::<String>(),
        )
    }

    pub fn into_cookie_value(self) -> String {
        self.0
    }
}

pub struct AuthToken(String);

impl AuthToken {
    pub fn new<T>(claims: T, jwt_secret: String) -> Result<Self, AuthError>
    where
        T: Serialize,
    {
        let jwt_tokens = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|_| AuthError::UnableCreateSession)?;
        Ok(Self(jwt_tokens))
    }

    pub fn into_cookie_value(self) -> String {
        self.0
    }
}

pub async fn must_authorized<B>(
    cookie_jar: CookieJar,
    State(state): State<Arc<AppState>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, AuthError> {
    let session_token = cookie_jar
        .get(COOKIE_SESSION_TOKEN_NAME)
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AuthError::Unauthorized)?;
    let jwt_session = cookie_jar
        .get(COOKIE_AUTH_NAME)
        .map(|cookie| {
            let value = cookie.value().to_string();
            value.strip_prefix("Bearer ").map(|token| token.to_owned())
        })
        .ok_or(AuthError::Unauthorized)?
        .ok_or(AuthError::UnknownTokenFormat)?;
    let (is_changed, cookie_jar) = check_session(
        cookie_jar,
        Arc::clone(&state),
        session_token.to_owned(),
        jwt_session.to_owned(),
    )
    .await?;

    if is_changed {
        let (mut parts, body) = req.into_parts();
        for cookie in cookie_jar.iter() {
            parts
                .headers
                .append(SET_COOKIE, HeaderValue::try_from(cookie.value()).unwrap());
        }
        let req = Request::from_parts(parts, body);
        return Ok(next.run(req).await);
    }
    Ok(next.run(req).await)
}

pub async fn new_session(
    state: Arc<AppState>,
    user: User,
    cookie_jar: CookieJar,
) -> anyhow::Result<CookieJar> {
    let user_id = user.id.ok_or(anyhow::anyhow!("user_id is empty"))?;
    let timestamp_now = chrono::Utc::now();
    let jwt_iat = chrono::Utc::now().timestamp();
    let jwt_expire = (timestamp_now + chrono::Duration::minutes(10)).timestamp();

    let session_id = SessionId::generate_new().into_cookie_value();
    let session_timestamp_expire =
        (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize;

    let jwt_claims = SessionTokenClaims {
        user_id: user_id.to_owned(),
        iat: jwt_iat as usize,
        exp: jwt_expire as usize,
        session_id: session_id.to_owned(),
    };
    let jwt_auth_token =
        AuthToken::new(jwt_claims, state.config.jwt_secret.to_string())?.into_cookie_value();
    let cookie_jar = cookie_jar
        .add(
            Cookie::build(COOKIE_AUTH_NAME, format!("Bearer {}", jwt_auth_token))
                .path("/")
                .secure(true)
                .same_site(SameSite::Lax)
                .max_age(time::Duration::minutes(5))
                .http_only(true)
                .finish(),
        )
        .add(
            Cookie::build(COOKIE_SESSION_TOKEN_NAME, session_id.to_owned())
                .path("/")
                .secure(true)
                .finish(),
        );

    let mut redis_conn = state.redis.get_async_connection().await?;
    let set_redis = redis_conn
        .set_nx::<String, String, usize>(session_id.to_owned(), user_id.to_owned())
        .await?;
    if set_redis == 0 {
        tracing::error!("Cannot set key in redis because exists!");
    }
    redis_conn
        .expire_at::<String, ()>(session_id.to_owned(), session_timestamp_expire)
        .await?;

    Ok(cookie_jar)
}

pub async fn check_session(
    cookie_jar: CookieJar,
    state: Arc<AppState>,
    session_token: String,
    jwt_session: String,
) -> Result<(bool, CookieJar), AuthError> {
    let token_data = jsonwebtoken::decode::<SessionTokenClaims>(
        jwt_session.as_str(),
        &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| AuthError::UnknownTokenFormat)?;

    let jwt_session_token = token_data.claims.session_id.to_owned();
    let jwt_user_id = token_data.claims.user_id.to_owned();

    if session_token != jwt_session_token {
        return Err(AuthError::Unauthorized);
    }

    let mut redis_conn = state.redis.get_async_connection().await.map_err(|_| {
        tracing::error!("Can't get connection into redis");
        AuthError::DatabaseError
    })?;

    let result = redis_conn
        .get::<String, redis::Value>(session_token.to_owned())
        .await
        .map_err(|_| {
            tracing::error!("Can't get connection into redis");
            AuthError::DatabaseError
        })?;

    let user_id = match result {
        Value::Nil => {
            return Err(AuthError::Unauthorized);
        }
        Value::Data(bytes) => String::from_utf8(bytes).map_err(|_| AuthError::Unknown)?,
        _ => {
            return Err(AuthError::Unknown);
        }
    };

    if user_id != jwt_user_id {
        return Err(AuthError::Unauthorized);
    }

    let current_time = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp < current_time {
        let jwt_expire = (chrono::Utc::now() + chrono::Duration::minutes(10)).timestamp();

        let new_jwt_claims = SessionTokenClaims {
            user_id,
            iat: current_time,
            exp: jwt_expire as usize,
            session_id: session_token.to_owned(),
        };
        let new_jwt_auth_token =
            AuthToken::new(new_jwt_claims, state.config.jwt_secret.to_string())?
                .into_cookie_value();
        let cookie_jar = cookie_jar.add(
            Cookie::build(COOKIE_AUTH_NAME, format!("Bearer {}", new_jwt_auth_token))
                .path("/")
                .secure(true)
                .same_site(SameSite::Lax)
                .max_age(time::Duration::minutes(5))
                .http_only(true)
                .finish(),
        );
        return Ok((true, cookie_jar));
    }

    Ok((false, cookie_jar))
}

pub async fn delete_session(
    state: Arc<AppState>,
    cookie_jar: CookieJar,
) -> Result<CookieJar, AuthError> {
    let session_token = cookie_jar
        .get(COOKIE_SESSION_TOKEN_NAME)
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AuthError::Unauthorized)?;

    let mut redis_conn = state.redis.get_async_connection().await.map_err(|_| {
        tracing::error!("Can't get a connection to redis");
        AuthError::DatabaseError
    })?;
    redis_conn
        .del::<String, ()>(session_token)
        .await
        .map_err(|_| AuthError::DatabaseError)?;

    let cookie_session_token = Cookie::build(COOKIE_SESSION_TOKEN_NAME, "")
        .path("/")
        .secure(true)
        .max_age(time::Duration::minutes(-1))
        .finish();
    let cookie_auth_token = Cookie::build(COOKIE_AUTH_NAME, "")
        .path("/")
        .secure(true)
        .max_age(time::Duration::minutes(-1))
        .finish();
    let cookie_jar = cookie_jar.add(cookie_auth_token).add(cookie_session_token);
    Ok(cookie_jar)
}
