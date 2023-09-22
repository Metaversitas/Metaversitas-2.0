use crate::backend::AppState;
use crate::helpers::authentication::{COOKIE_AUTH_NAME, COOKIE_SESSION_TOKEN_NAME};
use crate::helpers::errors::AuthError;
use crate::model::user::SessionTokenClaims;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: String,
    pub session_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MaybeAuthenticatedUser(pub Option<AuthenticatedUser>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = Arc::<AppState>::from_ref(state);
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::Unknown)?;
        let session_token = cookie_jar
            .get(COOKIE_SESSION_TOKEN_NAME)
            .map(|cookie| cookie.value().to_owned())
            .ok_or(AuthError::Unauthorized)?;
        let jwt_session = cookie_jar
            .get(COOKIE_AUTH_NAME)
            .map(|cookie| {
                cookie
                    .value()
                    .to_owned()
                    .strip_prefix("Bearer ")
                    .map(|token| token.to_owned())
            })
            .ok_or(AuthError::Unauthorized)?
            .ok_or(AuthError::UnknownTokenFormat)?;

        let token_data = jsonwebtoken::decode::<SessionTokenClaims>(
            jwt_session.as_str(),
            &jsonwebtoken::DecodingKey::from_secret(app_state.config.jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AuthError::UnknownTokenFormat)?;

        let jwt_session_token = token_data.claims.session_id.to_owned();
        let jwt_user_id = token_data.claims.user_id;

        if session_token != jwt_session_token {
            return Err(AuthError::Unauthorized);
        }

        let auth_user = AuthenticatedUser {
            user_id: jwt_user_id,
            session_id: session_token,
        };

        Ok(auth_user)
    }
}
