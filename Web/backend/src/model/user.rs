use chrono::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "lowercase")]
pub enum UserRole {
    Administrator,
    Dosen,
    Mahasiswa,
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[validate]
    pub user: T,
}

const SPECIAL_ASCII_CHAR: &str = "!@#$%^&*()";
fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special_char = password.chars().any(|c| SPECIAL_ASCII_CHAR.contains(c));
    let length_is_valid = {
        let length = password.len();

        length > 12 && length < 128
    };

    if !(has_uppercase && has_lowercase && has_digit && has_special_char && length_is_valid) {
        return Err(ValidationError::new("failed to validate password"));
    }
    Ok(())
}

static REGEX_NICKNAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z]\w{3,12}$").unwrap());

fn validate_nickname(nickname: &str) -> Result<(), ValidationError> {
    if !REGEX_NICKNAME.is_match(nickname) {
        return Err(ValidationError::new("failed to validate nickname"));
    };
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserSchema {
    #[validate(custom = "validate_nickname")]
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validate_password")]
    pub password: String,
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
