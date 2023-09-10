use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate};
use sqlx::Type;

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
where T: Serialize + Validate
{
    #[validate]
    pub user: T,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserSchema {
    pub nickname: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserSchema {
    #[validate(email)]
    pub email: String,
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
