use crate::backend::AppState;
use crate::helpers::authentication::{delete_session, must_authorized, COOKIE_SESSION_TOKEN_NAME, check_session};
use crate::helpers::errors::{AuthError, PhotonAuthError};
use crate::model::user::{AuthDataPhoton, LoginSchema, RegisterUserSchema, RequestPhotonAuth, UserJsonBody};
use crate::service::game::GameService;
use crate::service::user::UserService;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{middleware, Extension};
use axum::{Json, Router};
use axum_extra::extract::cookie::CookieJar;
use garde::Validate;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::result::Result;
use std::sync::Arc;

pub const AUTH_PATH_CONTROLLER: &str = "/auth";
pub const REGISTER_AUTH_PATH: &str = "/register";
pub const LOGIN_AUTH_PATH: &str = "/login";
pub const PHOTON_AUTH_PATH: &str = "/photon";
pub const LOGOUT_AUTH_PATH: &str = "/logout";
pub const REFRESH_TOKEN_AUTH_PATH: &str = "/refresh";

pub struct LoginStateService {
    pub user_service: Arc<UserService>,
    pub game_service: Arc<GameService>,
}

pub async fn auth_router(
    app_state: Arc<AppState>,
    game_service: Arc<GameService>,
    user_service: Arc<UserService>,
) -> Router {
    let login_state_service = Arc::new(LoginStateService {
        game_service: Arc::clone(&game_service),
        user_service: Arc::clone(&user_service),
    });

    Router::new()
        .route(REGISTER_AUTH_PATH, post(register))
        .with_state(Arc::clone(&user_service))
        .route(
            LOGIN_AUTH_PATH,
            post(login).with_state(Arc::clone(&login_state_service)),
        )
        .route(REFRESH_TOKEN_AUTH_PATH, get(refresh_token))
        .with_state(Arc::clone(&user_service))
        .route(
            LOGOUT_AUTH_PATH,
            post(logout).route_layer(middleware::from_fn_with_state(
                Arc::clone(&app_state),
                must_authorized,
            )),
        )
        .with_state(Arc::clone(&app_state))
        .route(
            PHOTON_AUTH_PATH,
            post(photon_auth)
                .with_state(Arc::clone(&app_state))
                .layer(Extension(Arc::clone(&user_service))),
        )
}

pub async fn refresh_token(
    State(user_service): State<Arc<UserService>>,
    cookie_jar: CookieJar,
) -> Result<impl IntoResponse, AuthError> {
    let session_token = cookie_jar
        .get(COOKIE_SESSION_TOKEN_NAME)
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AuthError::Unauthorized)?;
    let cookie_jar = user_service
        .refresh_token(session_token.as_str(), cookie_jar)
        .await?;
    let response = json!({"success": true, "message": "New token generated"});
    Ok((StatusCode::OK, cookie_jar, Json(response)))
}

pub async fn register(
    State(user_service): State<Arc<UserService>>,
    payload: Result<Json<UserJsonBody<RegisterUserSchema>>, JsonRejection>,
) -> Result<impl IntoResponse, AuthError> {
    let payload = {
        let Json(payload) = payload?;
        payload
            .validate(&())
            .map_err(|err| AuthError::Other(anyhow!(err.to_string())))?;
        payload
    };

    let email = payload.user.email.to_owned();
    let nickname = payload.user.nickname.to_owned();

    let registered_user = user_service
        .register(email.as_str(), nickname.as_str())
        .await?;

    let response = json!({"success": true, "message": "User successfully registered!", "data": registered_user});

    Ok((StatusCode::OK, Json(response)))
}

pub async fn login(
    State(login_service): State<Arc<LoginStateService>>,
    params: Option<Query<ParamsAuthenticate>>,
    payload: Result<Json<UserJsonBody<LoginSchema>>, JsonRejection>,
) -> Result<Response, AuthError> {
    if let Some(params) = params {
        //Validate params
        params.validate(&()).map_err(|err| {
            let message = err.to_string();
            AuthError::Other(anyhow::anyhow!(message))
        })?;

        if let Some(game_version) = params.game_version.as_ref() {
            login_service
                .game_service
                .verify_game_version(game_version.as_str())
                .await?;
        }
    }
    let payload = {
        // If not a valid Json
        let Json(payload) =
            payload?;
        // Validate json body
        payload.validate(&()).map_err(|_err| AuthError::Other(anyhow::anyhow!("Unable to validate json payload")))?;
        payload
    };

    match &payload.user {
        LoginSchema::Default(user) => {
            let (_user_data, cookie_jar) = login_service
                .user_service
                .login(user.email.as_str(), user.password.as_str())
                .await?;
            let response = json!({"success": true, "message": "Successfully logged in"});
            Ok((StatusCode::OK, cookie_jar, Json(response)).into_response())
        }
        LoginSchema::Metamask(_user) => {
            //TODO: Add a service for validating metamask
            Err(AuthError::Other(anyhow!(
                "not implemented yet"
            )))
        }
    }
}

pub async fn photon_auth(
    State(_app_state): State<Arc<AppState>>,
    Extension(user_service): Extension<Arc<UserService>>,
    payload: Result<Json<RequestPhotonAuth>, JsonRejection>
) -> Result<Response, PhotonAuthError> {
    let Json(payload) = payload?;
    let cookie_jar = CookieJar::default();
    let (_is_changed, auth_user, _cookie_jar) = check_session(cookie_jar, _app_state, payload.auth_data.cookie_session, payload.auth_data.cookie_auth).await?;

    let result = user_service.get_profile(auth_user.user_id.as_str()).await?;
    let auth_data = AuthDataPhoton {
        user_id: result.user_id.to_owned(),
        in_game_nickname: result.in_game_nickname.to_owned(),
        full_name: result.full_name.to_owned(),
        university_name: result.university_name.to_owned(),
        faculty_name: result.faculty_name.to_owned(),
        faculty_id: result.faculty_id,
        user_university_id: result.user_university_id,
        user_univ_role: result.user_univ_role.clone(),
        gender: result.gender,
    };
    let user_id = result.user_id.to_owned();
    let nickname = result.in_game_nickname.to_owned();
    let response = json!({
                        "ResultCode": 1,
                        "UserId": user_id,
                        "Nickname": nickname,
                        "Data": auth_data,
    });
    let response = (StatusCode::OK, Json(response)).into_response();
    Ok(response)
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

pub enum AuthFormatType {
    Photon,
    Default,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ParamsAuthenticate {
    #[serde(deserialize_with = "deserialize_format_params_auth")]
    #[garde(skip)]
    format: Option<FormatParamsAuth>,
    #[garde(length(min = 1))]
    game_version: Option<String>,
    #[garde(length(min = 1))]
    api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum FormatParamsAuth {
    Photon,
    NotExist,
}

fn deserialize_format_params_auth<'de, D>(
    deserializer: D,
) -> Result<Option<FormatParamsAuth>, D::Error>
where
    D: Deserializer<'de>,
{
    let format_as_string: Option<String> = Deserialize::deserialize(deserializer)?;

    if let Some(format_as_string) = format_as_string {
        return match format_as_string.to_ascii_lowercase().as_str() {
            "photon" => Ok(Some(FormatParamsAuth::Photon)),
            &_ => Ok(Some(FormatParamsAuth::NotExist)),
        };
    } else {
        Ok(None)
    }
}
