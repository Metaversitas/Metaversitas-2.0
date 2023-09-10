use axum::{async_trait, BoxError, extract::{FromRequest}, Json, body::{HttpBody}};
use axum::http::Request;
use serde::de::DeserializeOwned;
use validator::Validate;
use crate::helpers::errors::ApiError;

#[derive(Debug)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
  T: DeserializeOwned + Validate,
  B: HttpBody + Send + 'static,
  B::Data: Send,
  B::Error: Into<BoxError>,
  S: Send + Sync
{
  type Rejection = ApiError;

  async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
    let Json(value) = Json::<T>::from_request(req, state).await?;
    let _ = value.validate().map_err(|_| ApiError::ValidationError)?;
    Ok(ValidatedJson(value))
  }
}