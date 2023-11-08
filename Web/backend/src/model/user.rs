use chrono::prelude::*;
use garde::Validate;
use once_cell::sync::Lazy;
use redis_macros::{FromRedisValue, ToRedisArgs};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};

#[derive(Debug, Clone, Serialize, Deserialize, Type, FromRedisValue, ToRedisArgs)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Administrator,
    Staff,
    User,
}

#[derive(Debug, Serialize, Deserialize, FromRedisValue)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<UserRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserJsonBody<T>
where
    T: Serialize + Validate,
{
    #[garde(skip)]
    pub user: T,
}

const SPECIAL_ASCII_CHAR: &str = "!@#$%^&*()";
fn validate_password(password: &str, _ctx: &()) -> Result<(), garde::Error> {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special_char = password.chars().any(|c| SPECIAL_ASCII_CHAR.contains(c));
    let length_is_valid = {
        let length = password.len();

        length > 12 && length < 128
    };

    if !(has_uppercase && has_lowercase && has_digit && has_special_char && length_is_valid) {
        return Err(garde::Error::new("failed to validate password"));
    }
    Ok(())
}

#[allow(clippy::unwrap_used)]
static REGEX_NICKNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z]\w{3,12}$").unwrap());

fn validate_nickname(nickname: &str, _ctx: &()) -> Result<(), garde::Error> {
    if !REGEX_NICKNAME.is_match(nickname) {
        return Err(garde::Error::new("failed to validate nickname"));
    };
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserSchema {
    #[garde(custom(validate_nickname))]
    pub nickname: String,
    #[garde(email)]
    pub email: String,
    #[garde(custom(validate_password))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(untagged)]
#[garde(allow_unvalidated)]
pub enum LoginSchema {
    Default(LoginUserSchema),
    Metamask(MetamaskLoginUserSchema),
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[garde(email)]
    pub email: String,
    #[garde(custom(validate_password))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct MetamaskLoginUserSchema {
    #[garde(length(min = 1))]
    pub wallet_address: String,
    #[garde(length(min = 1))]
    pub signed_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredUser {
    pub user_id: String,
    pub email: String,
    pub is_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenClaims {
    #[serde(rename = "sub")]
    pub user_id: String,
    pub iat: usize,
    pub exp: usize,
    #[serde(rename = "sid")]
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse<T>
where
    T: Serialize,
{
    pub status: bool,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize, Type, FromRedisValue, ToRedisArgs, Clone)]
#[sqlx(type_name = "user_university_role")]
#[sqlx(rename_all = "lowercase")]
pub enum UserUniversityRole {
    Dosen,
    Mahasiswa,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[sqlx(type_name = "users_identity_gender")]
#[sqlx(rename_all = "lowercase")]
pub enum UserGender {
    Male,
    Female,
}

#[derive(Debug, Serialize, Deserialize, FromRedisValue, ToRedisArgs, FromRow)]
pub struct ProfileUserData {
    pub user_id: String,
    pub in_game_nickname: String,
    pub full_name: String,
    pub university_name: String,
    pub faculty_name: String,
    #[sqlx(try_from = "i32")]
    pub faculty_id: u64,
    #[sqlx(try_from = "i32")]
    pub user_university_id: u64,
    pub user_univ_role: UserUniversityRole,
    pub gender: UserGender,
    pub profile_image_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BodyPhotonAuth {
    pub cookie_auth: String,
    pub cookie_session: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestPhotonAuth {
    pub auth_data: BodyPhotonAuth,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthDataPhoton {
    pub user_id: String,
    pub in_game_nickname: String,
    pub full_name: String,
    pub university_name: String,
    pub faculty_name: String,
    pub faculty_id: u64,
    pub user_university_id: u64,
    pub user_univ_role: UserUniversityRole,
    pub gender: UserGender,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateParamsUserIdentity {
    pub full_name: Option<String>,
    pub gender: Option<UserGender>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateParamsUserData {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub nickname: Option<String>,
    pub is_verified: Option<bool>,
    pub role: Option<UserRole>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserPasswordParams {
    pub current_password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}