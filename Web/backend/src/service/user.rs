use crate::backend::AppState;
use crate::helpers::authentication::{new_session, AuthToken, COOKIE_AUTH_NAME};
use crate::helpers::errors::{UserServiceError};
use crate::model::user::{ProfileUserData, RegisteredUser, User, UserGender, UserRole};
use crate::model::user::{SessionTokenClaims, UserUniversityRole};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use redis::{AsyncCommands, JsonAsyncCommands};
use std::sync::Arc;

pub struct UserService {
    pub app_state: Arc<AppState>,
}

const DEFAULT_TIME_CACHE_EXIST: time::Duration = time::Duration::minutes(30);

impl UserService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(ProfileUserData, CookieJar), UserServiceError> {
        let query = sqlx::query!(r#"
        select
            users.user_id as user_id,
            users.password_hash as password_hash,
            users.nickname as in_game_nickname,
            identity.full_name as full_name,
            identity.gender as "gender!: UserGender",
            university.name as university_name,
            university_faculty.faculty_name as faculty_name,
            university_faculty.faculty_id as faculty_id,
            university_identity.users_university_id as user_university_id,
            university_identity.users_university_role as "user_univ_role!: UserUniversityRole"
        from users
        inner join users_identity as identity on users.user_id = identity.users_id
        inner join users_university_identity as university_identity on identity.users_identity_id = university_identity.users_identity_id
        inner join university on university_identity.university_id = university.university_id
        inner join university_faculty on university.university_id = university_faculty.university_id
        where users.email::text = $1"#,
            email
        )
        .fetch_optional(&self.app_state.database)
        .await
        .map_err(|err| {
            tracing::error!("error on the database: {}", err);
            UserServiceError::DatabaseConnectionError
        })?
        .ok_or(UserServiceError::UserDoesNotExist)?;

        verify_password(password.to_string(), query.password_hash.to_owned())
            .await
            .map_err(|_| UserServiceError::PasswordNotMatch)?;

        let user = User {
            id: Some(query.user_id.to_string()),
            role: None,
            email: None,
            password_hash: None,
            nickname: None,
            verified: None,
            created_at: None,
            updated_at: None,
        };

        let user_data: ProfileUserData = ProfileUserData {
            user_id: query.user_id.to_string(),
            in_game_nickname: query.in_game_nickname.to_owned(),
            full_name: query.full_name.to_owned(),
            university_name: query.university_name.to_owned(),
            faculty_name: query.faculty_name.to_owned(),
            faculty_id: query.faculty_id as u64,
            user_university_id: query.user_university_id as u64,
            user_univ_role: query.user_univ_role,
            gender: query.gender,
        };

        let cookie_jar = CookieJar::default();
        let cookie_jar = new_session(Arc::clone(&self.app_state), user, cookie_jar)
            .await
            .map_err(|_| UserServiceError::UnableCreateSession)?;
        Ok((user_data, cookie_jar))
    }

    pub async fn register(
        &self,
        email: &str,
        nickname: &str,
    ) -> Result<RegisteredUser, UserServiceError> {
        let query = sqlx::query!(
            "select exists(select 1 from users where email = ($1));",
            email,
        )
        .fetch_one(&self.app_state.database)
        .await
        .map_err(|_| UserServiceError::DatabaseConnectionError)?;

        if let Some(is_email_exists) = query.exists {
            if is_email_exists {
                return Err(UserServiceError::UserAlreadyExists);
            }
        }

        let password_hash = hash_password(email.to_string()).await.map_err(|_| {
            tracing::error!("Error when hashing password");
            UserServiceError::UnableHashPassword
        })?;

        let user_roles = UserRole::User;
        let row = sqlx::query!(r#"insert into users (email, password_hash, nickname, role, is_verified) values ($1::text, $2, $3, $4, $5) returning user_id, email, is_verified;"#, email.to_owned(), password_hash, nickname.to_owned(), user_roles as UserRole, false)
            .fetch_one(&self.app_state.database)
            .await
            .map_err(|_| {
                UserServiceError::DatabaseConnectionError
            })?;

        let registered_user = RegisteredUser {
            user_id: row.user_id.to_string(),
            email: row.email,
            is_verified: row.is_verified,
        };

        Ok(registered_user)
    }

    pub async fn refresh_token(
        &self,
        session_token: &str,
        cookie_jar: CookieJar,
    ) -> Result<CookieJar, UserServiceError> {
        let mut redis_conn = self
            .app_state
            .redis
            .get_async_connection()
            .await
            .map_err(|_| {
                tracing::error!("Can't get connection into redis");
                UserServiceError::RedisConnectionError
            })?;

        let result = redis_conn
            .get::<String, redis::Value>(session_token.to_owned())
            .await
            .map_err(|_| {
                tracing::error!("Can't get connection into redis");
                UserServiceError::RedisConnectionError
            })?;

        let user_id = match result {
            redis::Value::Nil => {
                return Err(UserServiceError::UnauthorizedAccess);
            }
            redis::Value::Data(bytes) => {
                String::from_utf8(bytes).map_err(|_| UserServiceError::UnableToParse)?
            }
            _ => {
                return Err(UserServiceError::UnableToParse);
            }
        };

        let timestamp_now = chrono::Utc::now();
        let jwt_iat = chrono::Utc::now().timestamp();
        let jwt_expire = (timestamp_now + chrono::Duration::minutes(10)).timestamp();
        let jwt_claims = SessionTokenClaims {
            user_id,
            iat: jwt_iat as usize,
            exp: jwt_expire as usize,
            session_id: session_token.to_owned(),
        };
        let jwt_auth_token =
            AuthToken::new(jwt_claims, self.app_state.config.jwt_secret.to_string())
                .map_err(|_| UserServiceError::UnableCreateSession)?
                .into_cookie_value();
        let cookie_jar = cookie_jar.add(
            Cookie::build(COOKIE_AUTH_NAME, format!("Bearer {}", jwt_auth_token))
                .path("/")
                .secure(true)
                .same_site(SameSite::Lax)
                .max_age(time::Duration::minutes(5))
                .http_only(true)
                .finish(),
        );

        Ok(cookie_jar)
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<ProfileUserData, UserServiceError> {
        let mut redis_conn = self.app_state.redis.get_async_connection().await.unwrap();
        let is_exists = redis_conn
            .exists::<String, usize>(format!("profile:{}", &user_id))
            .await
            .unwrap();

        if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, ProfileUserData>(format!("profile:{}", &user_id), "$")
                .await
                .unwrap();

            Ok(query_from_redis)
        } else if is_exists == 0 {
            let query = sqlx::query!(r#"
        select
            users.user_id as user_id,
            users.nickname as in_game_nickname,
            identity.full_name as full_name,
            identity.gender as "gender!: UserGender",
            university.name as university_name,
            university_faculty.faculty_name as faculty_name,
            university_faculty.faculty_id as faculty_id,
            university_identity.users_university_id as user_university_id,
            university_identity.users_university_role as "user_univ_role!: UserUniversityRole"
        from users
        inner join users_identity as identity on users.user_id = identity.users_id
        inner join users_university_identity as university_identity on identity.users_identity_id = university_identity.users_identity_id
        inner join university on university_identity.university_id = university.university_id
        inner join university_faculty on university.university_id = university_faculty.university_id
        where users.user_id::text = $1"#, &user_id)
            .fetch_one(&self.app_state.database)
            .await
            .map_err(|_| UserServiceError::DatabaseConnectionError)?;

            let data = ProfileUserData {
                user_id: query.user_id.to_string(),
                in_game_nickname: query.in_game_nickname.to_owned(),
                full_name: query.full_name.to_owned(),
                university_name: query.university_name.to_owned(),
                faculty_name: query.faculty_name.to_owned(),
                faculty_id: query.faculty_id as u64,
                user_university_id: query.user_university_id as u64,
                user_univ_role: query.user_univ_role,
                gender: query.gender,
            };

            let profile_user_id = format!("profile:{}", &user_id);
            let timestamp_expire = (time::OffsetDateTime::now_utc() + DEFAULT_TIME_CACHE_EXIST)
                .unix_timestamp() as usize;
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

            Ok(data)
        } else {
            Err(UserServiceError::UnableToParse)
        }
    }
}

pub async fn hash_password(password: String) -> Result<String, ()> {
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

pub async fn verify_password(password: String, password_hash: String) -> Result<(), ()> {
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
