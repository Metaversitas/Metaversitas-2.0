use crate::backend::AppState;
use crate::helpers::authentication::{delete_session, must_authorized, new_session};
use crate::helpers::errors::{ApiError, AuthError};
use crate::model::user::{LoginUserSchema, RegisterUserSchema, RegisteredUser, UserJsonBody};
use crate::model::user::{User, UserRole};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_extra::extract::cookie::CookieJar;
use axum_extra::extract::WithRejection;
use serde_json::json;
use std::result::Result;
use std::sync::Arc;
use crate::helpers::extractor::ValidatedJson;

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
            get(refresh_token).route_layer(middleware::from_fn_with_state(
                Arc::clone(&app_state),
                must_authorized,
            )),
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

pub async fn refresh_token() -> Result<impl IntoResponse, AuthError> {
    let response = json!({"success": true, "message": "New token generated"});
    Ok((StatusCode::OK, Json(response)))
}

pub async fn register(
    State(data): State<Arc<AppState>>,
    WithRejection(ValidatedJson(body), _): WithRejection<ValidatedJson<UserJsonBody<RegisterUserSchema>>, ApiError>,
) -> Result<impl IntoResponse, AuthError> {
    const QUERY: &str = "";
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
    WithRejection(ValidatedJson(body), _): WithRejection<ValidatedJson<UserJsonBody<LoginUserSchema>>, ApiError>,
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
    .ok_or(AuthError::UserRegistered)?;

    verify_password(
        body.user.password.to_string(),
        user.password_hash.to_owned(),
    )
    .await
    .map_err(|_| AuthError::Unauthorized)?;

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
        PasswordHash::generate(argon2::Argon2::default(), password.as_str(), &salt)
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
