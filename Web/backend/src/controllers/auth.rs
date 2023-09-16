use crate::backend::AppState;
use crate::helpers::authentication::{AuthToken, COOKIE_AUTH_NAME, COOKIE_SESSION_TOKEN_NAME, delete_session, must_authorized, new_session};
use crate::helpers::errors::{ApiError, AuthError};
use crate::helpers::extractor::ValidatedJson;
use crate::model::user::{LoginUserSchema, RegisterUserSchema, RegisteredUser, UserJsonBody, SessionTokenClaims};
use crate::model::user::{User, UserRole};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use axum_extra::extract::WithRejection;
use serde_json::json;
use std::result::Result;
use std::sync::Arc;
use redis::AsyncCommands;

pub const AUTH_PATH_CONTROLLER: &str = "/auth";
pub const REGISTER_AUTH_PATH: &str = "/register";
pub const LOGIN_AUTH_PATH: &str = "/login";
pub const LOGOUT_AUTH_PATH: &str = "/logout";
pub const REFRESH_TOKEN_AUTH_PATH: &str = "/refresh";

pub async fn auth_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(REGISTER_AUTH_PATH, post(register))
        .route(LOGIN_AUTH_PATH, post(login))
        .route(
            REFRESH_TOKEN_AUTH_PATH,
            get(refresh_token),
        )
        .route(
            LOGOUT_AUTH_PATH,
            post(logout).route_layer(middleware::from_fn_with_state(
                Arc::clone(&app_state),
                must_authorized,
            )),
        )
        .with_state(Arc::clone(&app_state))
}

pub async fn refresh_token(
    State(app_state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let session_token = cookie_jar
        .get(COOKIE_SESSION_TOKEN_NAME)
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AuthError::Unauthorized)?;

    let mut redis_conn = app_state.redis.get_async_connection().await.map_err(|_| {
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
        redis::Value::Nil => {
            return Err(AuthError::Unauthorized);
        }
        redis::Value::Data(bytes) => String::from_utf8(bytes).map_err(|_| AuthError::Unknown)?,
        _ => {
            return Err(AuthError::Unknown);
        }
    };

    let timestamp_now = chrono::Utc::now();
    let jwt_iat = chrono::Utc::now().timestamp();
    let jwt_expire = (timestamp_now + chrono::Duration::minutes(10)).timestamp();
    let jwt_claims = SessionTokenClaims {
        user_id: user_id.to_owned(),
        iat: jwt_iat as usize,
        exp: jwt_expire as usize,
        session_id: session_token.to_owned(),
    };
    let jwt_auth_token = AuthToken::new(jwt_claims, app_state.config.jwt_secret.to_string())?.into_cookie_value();
    let cookie_jar = cookie_jar.add(
        Cookie::build(COOKIE_AUTH_NAME, format!("Bearer {}", jwt_auth_token))
            .path("/")
            .secure(true)
            .same_site(SameSite::Lax)
            .max_age(time::Duration::minutes(5))
            .http_only(true)
            .finish()
    );

    let response = json!({"success": true, "message": "New token generated"});
    Ok((StatusCode::OK, cookie_jar, Json(response)))
}

pub async fn register(
    State(data): State<Arc<AppState>>,
    WithRejection(ValidatedJson(body), _): WithRejection<
        ValidatedJson<UserJsonBody<RegisterUserSchema>>,
        ApiError,
    >,
) -> Result<impl IntoResponse, AuthError> {
    let email = body.user.email.to_owned();
    let nickname = body.user.nickname.to_owned();

    let query = sqlx::query!(
        "select exists(select 1 from users where email = ($1));",
        email.to_owned(),
    )
    .fetch_one(&data.database)
    .await
    .map_err(|_| AuthError::DatabaseError)?;

    if let Some(is_email_exists) = query.exists {
        if is_email_exists {
            return Err(AuthError::UserRegistered);
        }
    }

    let password_hash = hash_password(body.user.password.to_string())
        .await
        .map_err(|_| {
            tracing::error!("Error when hashing password");
            AuthError::Unknown
        })?;

    let user_roles = UserRole::Mahasiswa;
    let row = sqlx::query!(r#"insert into users (email, password_hash, nickname, role, is_verified) values ($1::text, $2, $3, $4, $5) returning user_id, email, is_verified;"#, email.to_owned(), password_hash, nickname.to_owned(), user_roles as UserRole, false)
        .fetch_one(&data.database)
        .await
        .map_err(|_| {
        AuthError::DatabaseError
    })?;

    let registered_user = RegisteredUser {
        user_id: row.user_id.to_string(),
        email: row.email,
        is_verified: row.is_verified,
    };

    Ok((
        StatusCode::OK,
        Json(
            json!({"success": true, "message": "User successfully registered!", "data": registered_user}),
        ),
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    WithRejection(ValidatedJson(body), _): WithRejection<
        ValidatedJson<UserJsonBody<LoginUserSchema>>,
        ApiError,
    >,
) -> Result<impl IntoResponse, AuthError> {
    let user = sqlx::query!(
        "select user_id, email, password_hash from users where email = $1;",
        body.user.email
    )
    .fetch_optional(&state.database)
    .await
    .map_err(|err| {
        tracing::error!("error on the database: {}", err);
        AuthError::DatabaseError
    })?
    .ok_or(AuthError::InvalidUsernameOrPassword)?;

    verify_password(
        body.user.password.to_string(),
        user.password_hash.to_owned(),
    )
    .await
    .map_err(|_| AuthError::InvalidUsernameOrPassword)?;

    let user = User {
        id: Some(user.user_id.to_string()),
        role: None,
        email: None,
        password_hash: None,
        nickname: None,
        verified: None,
        created_at: None,
        updated_at: None,
    };
    let cookie_jar = CookieJar::default();
    let cookie_jar = new_session(Arc::clone(&state), user, cookie_jar)
        .await
        .map_err(|_| AuthError::UnableCreateSession)?;

    let response = json!({"success": true, "message": "Successfully logged in"});
    Ok((StatusCode::OK, cookie_jar, Json(response)))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let cookie_jar = delete_session(Arc::clone(&state), cookie_jar).await?;
    let response = json!({"success": true, "message": "Successfully logged out"});
    Ok((StatusCode::OK, cookie_jar, Json(response)))
}

pub async fn verify_email() {
    unimplemented!()
}

pub async fn forgot_password() {
    unimplemented!()
}

async fn hash_password(password: String) -> Result<String, ()> {
    tokio::task::spawn_blocking(move || -> Result<String, ()> {
        let salt = SaltString::generate(rand::thread_rng());
        PasswordHash::generate(Argon2::default(), password.as_str(), &salt)
            .map_err(|e| {
                tracing::error!("failed to generate password hash");
                anyhow::anyhow!("failed to generate password hash: {}", e)
            })
            .map_or_else(
                |_| Err(()),
                |password_hashed| Ok(password_hashed.to_string()),
            )
    })
    .await
    .expect("can't join the result from tokio")
}

async fn verify_password(password: String, password_hash: String) -> Result<(), ()> {
    tokio::task::spawn_blocking(move || -> Result<(), ()> {
        let hash = PasswordHash::new(&password_hash).map_err(|_| {
            tracing::error!("failed to parse argon2 password hashed");
        })?;
        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|_| {
                tracing::error!("failed to verify password");
            })?;
        Ok(())
    })
    .await
    .expect("can't join the result from tokio")
}
