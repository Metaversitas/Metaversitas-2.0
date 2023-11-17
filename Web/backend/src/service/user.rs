use crate::backend::AppState;
use crate::helpers::authentication::{new_session, AuthToken, COOKIE_AUTH_NAME};
use crate::helpers::errors::user::UserServiceError;
use crate::model::user::{
    ProfileUserData, RegisteredUser, UpdateParamsUserData, UpdateParamsUserIdentity, User,
    UserGender, UserRole, UserTypeProfile,
};
use crate::model::user::{SessionTokenClaims, UserUniversityRole};
use crate::r#const::{ENV_ENVIRONMENT_DEVELOPMENT, ENV_ENVIRONMENT_PRODUCTION};
use crate::service::object_storage::ObjectStorage;
use anyhow::anyhow;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use redis::{AsyncCommands, JsonAsyncCommands};
use sqlx::{PgConnection, Postgres, QueryBuilder};
use std::sync::Arc;

pub struct UserService {
    app_state: Arc<AppState>,
    object_storage: Arc<ObjectStorage>,
}

const DEFAULT_TIME_CACHE_EXIST: time::Duration = time::Duration::minutes(30);

impl UserService {
    pub fn new(app_state: Arc<AppState>, object_storage: Arc<ObjectStorage>) -> Self {
        Self {
            app_state,
            object_storage,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<CookieJar, UserServiceError> {
        let query = sqlx::query!(
            r#"
        select
            users.user_id as user_id,
            users.password_hash as password_hash
        from users
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

        let cookie_jar = CookieJar::default();
        let cookie_jar = new_session(Arc::clone(&self.app_state), user, cookie_jar)
            .await
            .map_err(|_| UserServiceError::UnableCreateSession)?;
        Ok(cookie_jar)
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
        let mut redis_conn = self.app_state.redis.clone();

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
        let cookie_auth_token = {
            let cookie = Cookie::build(COOKIE_AUTH_NAME, format!("Bearer {}", jwt_auth_token))
                .path("/")
                .secure(true)
                .max_age(time::Duration::minutes(5))
                .http_only(true);
            if self
                .app_state
                .config
                .web_app_environment
                .contains(ENV_ENVIRONMENT_PRODUCTION)
            {
                cookie.same_site(SameSite::Strict).finish()
            } else if self
                .app_state
                .config
                .web_app_environment
                .contains(ENV_ENVIRONMENT_DEVELOPMENT)
            {
                cookie.same_site(SameSite::None).domain("").finish()
            } else {
                cookie.same_site(SameSite::Strict).finish()
            }
        };
        let cookie_jar = cookie_jar.add(cookie_auth_token);

        Ok(cookie_jar)
    }

    pub async fn get_user_data(&self, user_id: &str) -> Result<User, UserServiceError> {
        let user_data_key = format!("user_data:{}", &user_id);
        let mut redis_conn = self.app_state.redis.clone();
        let is_exists = redis_conn
            .exists::<String, usize>(user_data_key.to_owned())
            .await
            .map_err(|_| {
                UserServiceError::UnexpectedError(anyhow!("Unable to do exists on redis"))
            })?;

        if is_exists == 0 {
            let query = sqlx::query!(
                r#"
            select
                users.role as "role!: UserRole",
                users.user_id::text as "user_id!",
                users.email::text as "email!",
                users.password_hash,
                users.created_at,
                users.updated_at,
                users.nickname,
                users.is_verified
            from users
            where users.user_id::text = $1
            "#,
                &user_id
            )
            .fetch_one(&self.app_state.database)
            .await
            .map_err(|_| UserServiceError::DatabaseConnectionError)?;

            let data = User {
                id: Some(query.user_id),
                role: Some(query.role),
                email: Some(query.email),
                password_hash: Some(query.password_hash),
                nickname: Some(query.nickname),
                verified: Some(query.is_verified),
                created_at: Some(query.created_at),
                updated_at: Some(query.updated_at),
            };

            let _ = redis_conn
                .json_set::<String, String, User, redis::Value>(
                    user_data_key.to_owned(),
                    "$".to_string(),
                    &data,
                )
                .await
                .map_err(|_| {
                    UserServiceError::UnexpectedError(anyhow!("Unable to do json_set on redis"))
                })?;
            Ok(data)
        } else {
            let query_from_redis = redis_conn
                .json_get::<String, &str, User>(user_data_key.to_owned(), "$")
                .await
                .map_err(|_| {
                    UserServiceError::UnexpectedError(anyhow!("Unable to do json_get on redis"))
                })?;
            Ok(query_from_redis)
        }
    }

    pub async fn get_profile(&self, user_id: &str) -> Result<ProfileUserData, UserServiceError> {
        let mut redis_conn = self.app_state.redis.clone();
        let is_exists = redis_conn
            .exists::<String, usize>(format!("profile:{}", &user_id))
            .await
            .map_err(|_| {
                UserServiceError::UnexpectedError(anyhow!("Unable to do exists on redis"))
            })?;

        if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, ProfileUserData>(format!("profile:{}", &user_id), "$")
                .await
                .map_err(|_| {
                    UserServiceError::UnexpectedError(anyhow!("Unable to do json_get on redis"))
                })?;

            Ok(query_from_redis)
        } else if is_exists == 0 {
            let query = sqlx::query!(r#"
        select
            users.user_id as user_id,
            users.nickname as in_game_nickname,
            users.is_verified,
            users.role as "role!: UserRole",
            identity.full_name as full_name,
            identity.gender as "gender!: UserGender",
            identity.photo_url as "photo_url!",
            university.name as university_name,
            university_faculty.faculty_name as faculty_name,
            university_faculty.faculty_id as faculty_id,
            university_identity.users_university_id as user_university_id,
            university_identity.users_university_role as "user_univ_role!: UserUniversityRole",
            case
                when university_identity.users_university_role = 'dosen' then teachers.teacher_id
                when university_identity.users_university_role = 'mahasiswa' then students.student_id
            end::text as "role_id!"
        from users
        inner join users_identity as identity on users.user_id = identity.users_id
        inner join users_university_identity as university_identity on identity.users_identity_id = university_identity.users_identity_id
        inner join university on university_identity.university_id = university.university_id
        inner join university_faculty on university.university_id = university_faculty.university_id
        left join teachers on users.user_id = teachers.user_id and university_identity.users_university_role = 'dosen'
        left join students on users.user_id = students.user_id and university_identity.users_university_role = 'mahasiswa'
        where users.user_id::text = $1;"#, &user_id)
            .fetch_one(&self.app_state.database)
            .await
            .map_err(|_| UserServiceError::DatabaseConnectionError)?;

            let signed_url = self
                .object_storage
                .bucket_presigned_get_url(query.photo_url.as_str(), None)
                .await
                .map_err(|err| {
                    UserServiceError::UnexpectedError(anyhow!(
                        "Unable to get presigned bucket url, with an error: {}",
                        err.to_string()
                    ))
                })?;

            let data = ProfileUserData {
                user_id: query.user_id.to_string(),
                in_game_nickname: query.in_game_nickname.to_owned(),
                full_name: query.full_name.to_owned(),
                university_name: query.university_name.to_owned(),
                faculty_name: query.faculty_name.to_owned(),
                faculty_id: query.faculty_id as u64,
                user_university_id: query.user_university_id as u64,
                user_univ_role: query.user_univ_role.clone(),
                gender: query.gender,
                profile_image_url: signed_url,
                user_role: query.role,
                user_type: {
                    match query.user_univ_role {
                        UserUniversityRole::Dosen => UserTypeProfile::Teacher {
                            teacher_id: query.role_id,
                        },
                        UserUniversityRole::Mahasiswa => UserTypeProfile::Student {
                            student_id: query.role_id,
                        },
                    }
                },
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
                .map_err(|_| {
                    UserServiceError::UnexpectedError(anyhow!("Unable to do json_set on redis"))
                })?;
            let _ = redis_conn
                .expire_at::<String, redis::Value>(profile_user_id.to_owned(), timestamp_expire)
                .await
                .map_err(|_| {
                    UserServiceError::UnexpectedError(anyhow!("Unable to do expire_at on redis"))
                })?;

            Ok(data)
        } else {
            Err(UserServiceError::UnableToParse)
        }
    }

    pub async fn update_user_identity(
        &self,
        conn: &mut PgConnection,
        user_id: &str,
        params: &UpdateParamsUserIdentity,
    ) -> Result<(), UserServiceError> {
        let mut count = 0;
        let mut _curr_count = 0;

        if params.photo_url.is_some() {
            count += 1;
        }
        if params.full_name.is_some() {
            count += 1;
        }
        if params.gender.is_some() {
            count += 1;
        }

        if count == 0 {
            return Err(UserServiceError::UnexpectedError(anyhow!(
                "No value to be changed"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(r#"update users_identity set "#);

        if let Some(photo_url) = &params.photo_url {
            query_builder.push("photo_url = ");
            query_builder.push_bind(photo_url);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(full_name) = &params.full_name {
            query_builder.push("full_name = ");
            query_builder.push_bind(full_name);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(gender) = &params.gender {
            query_builder.push("gender = ");
            query_builder.push_bind(gender);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        query_builder.push(" where users_id::text = ");
        query_builder.push_bind(user_id);

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            UserServiceError::UnexpectedError(anyhow!(
                "Unable to execute query to database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn update_user_data(
        &self,
        conn: &mut PgConnection,
        user_id: &str,
        params: &UpdateParamsUserData,
    ) -> Result<(), UserServiceError> {
        let mut count = 0;
        let mut _curr_count = 0;

        if params.email.is_some() {
            count += 1;
        }
        if params.password_hash.is_some() {
            count += 1;
        }
        if params.is_verified.is_some() {
            count += 1;
        }
        if params.nickname.is_some() {
            count += 1;
        }
        if params.role.is_some() {
            count += 1;
        }

        if count == 0 {
            return Err(UserServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new(r#"update users set "#);

        if let Some(email) = &params.email {
            query_builder.push("email = ");
            query_builder.push_bind(email);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(password_hash) = &params.password_hash {
            query_builder.push("password_hash = ");
            query_builder.push_bind(password_hash);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(nickname) = &params.nickname {
            query_builder.push("nickname = ");
            query_builder.push_bind(nickname);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(is_verified) = &params.is_verified {
            query_builder.push("is_verified = ");
            query_builder.push_bind(is_verified);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(role) = &params.role {
            query_builder.push("role = ");
            query_builder.push_bind(role);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        query_builder.push(" where user_id::text = ");
        query_builder.push_bind(user_id);

        let query = query_builder.build();
        query.execute(&mut *conn).await.map_err(|err| {
            UserServiceError::UnexpectedError(anyhow!(
                "Unable to execute update user, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }
}

#[allow(clippy::expect_used)]
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

#[allow(clippy::expect_used)]
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
