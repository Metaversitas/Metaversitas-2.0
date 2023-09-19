use crate::backend::AppState;
use crate::helpers::authentication::must_authorized;
use crate::helpers::errors::AuthError;
use crate::helpers::extractor::AuthenticatedUser;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Json, Router};
use redis::{AsyncCommands, JsonAsyncCommands};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

pub const USER_PATH_CONTROLLER: &str = "/user";
const DEFAULT_TIME_CACHE_EXIST: time::Duration = time::Duration::minutes(30);
pub async fn user_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/profile",
            get(get_profile).route_layer(middleware::from_fn_with_state(
                Arc::clone(&app_state),
                must_authorized,
            )),
        )
        .with_state(Arc::clone(&app_state))
}

pub async fn get_profile(
    auth_user: AuthenticatedUser,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AuthError> {
    let mut redis_conn = state.redis.get_async_connection().await.unwrap();
    let is_exists = redis_conn
        .exists::<String, usize>(format!("profile:{}", auth_user.user_id.to_owned()))
        .await
        .unwrap();

    if is_exists == 1 {
        let query_from_redis = redis_conn
            .json_get::<String, &str, ProfileUserData>(
                format!("profile:{}", auth_user.user_id.to_owned()),
                "$",
            )
            .await
            .unwrap();
        let response = json!(ProfileResponse {
            status: true,
            data: query_from_redis
        });
        Ok((StatusCode::OK, Json(response)))
    } else if is_exists == 0 {
        let query = sqlx::query!(r#"
        select
            users.user_id::uuid as user_id,
            users.nickname as in_game_nickname,
            identity.full_name as full_name,
            university.name as university_name,
            university_faculty.faculty_name as faculty_name,
            university_faculty.faculty_id as faculty_id,
            university_identity.users_university_id as user_univ_id
        from users
        inner join users_identity as identity on users.user_id = identity.users_id
        inner join users_university_identity as university_identity on identity.users_identity_id = university_identity.users_identity_id
        inner join university on university_identity.university_id = university.university_id
        inner join university_faculty on university.university_id = university_faculty.university_id
        where users.user_id::text = $1;"#, &auth_user.user_id)
            .fetch_one(&state.database)
            .await
            .unwrap();
        let data = ProfileUserData {
            user_id: query.user_id.to_string(),
            in_game_nickname: query.in_game_nickname.to_owned(),
            full_name: query.full_name.to_owned(),
            university_name: query.university_name.to_owned(),
            faculty_name: query.faculty_name.to_owned(),
            faculty_id: query.faculty_id as usize,
            user_university_id: query.user_univ_id as usize,
        };

        let profile_user_id = format!("profile:{}", auth_user.user_id.to_owned());
        let timestamp_expire =
            (time::OffsetDateTime::now_utc() + DEFAULT_TIME_CACHE_EXIST).unix_timestamp() as usize;
        let _ = redis_conn
            .json_set::<String, String, ProfileUserData, redis::Value>(
                profile_user_id.to_owned(),
                "$".to_string(),
                &data,
            )
            .await
            .unwrap();
        let _ = redis_conn
            .expire_at::<String, redis::Value>(profile_user_id.to_owned(), timestamp_expire)
            .await
            .unwrap();

        let response = json!(ProfileResponse { status: true, data });
        return Ok((StatusCode::OK, Json(response)));
    } else {
        return Err(AuthError::Unknown);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileResponse<T>
where
    T: Serialize,
{
    pub status: bool,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
struct ProfileUserData {
    pub user_id: String,
    pub in_game_nickname: String,
    pub full_name: String,
    pub university_name: String,
    pub faculty_name: String,
    pub faculty_id: usize,
    pub user_university_id: usize,
}
