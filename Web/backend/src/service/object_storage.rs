use crate::backend::AppState;
use crate::helpers::errors::object_storage::ObjectStorageError;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use std::sync::Arc;

static DEFAULT_PRESIGNED_URL_EXPIRATION: Lazy<chrono::Duration> =
    Lazy::new(|| chrono::Duration::hours(1));

pub struct ObjectStorage {
    app_state: Arc<AppState>,
}

impl ObjectStorage {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn bucket_presigned_get_url(
        &self,
        path_to_object: &str,
        expiration: Option<i32>,
    ) -> Result<String, ObjectStorageError> {
        let url = self
            .app_state
            .bucket
            .presign_get(
                path_to_object,
                expiration
                    .unwrap_or(
                        DEFAULT_PRESIGNED_URL_EXPIRATION
                            .num_seconds()
                            .try_into()
                            .map_err(|_| {
                                ObjectStorageError::UnexpectedError(anyhow!(
                                    "Unable to convert default url expiration into i32"
                                ))
                            })?,
                    )
                    .try_into()
                    .map_err(|_| {
                        ObjectStorageError::UnexpectedError(anyhow!(
                            "Unable to convert expiration into u32"
                        ))
                    })?,
                None,
            )
            .map_err(|err| {
                ObjectStorageError::UnexpectedError(anyhow!(
                    "Unable to get presigned url from s3, with an error: {}",
                    err.to_string()
                ))
            })?;
        Ok(url)
    }
}
