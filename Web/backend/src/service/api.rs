use crate::backend::AppState;
use std::sync::Arc;
use thiserror::Error;

pub struct ApiKeyService {
    app_state: Arc<AppState>,
}

impl ApiKeyService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn verify_api_key(&self, _api_key: &str) -> Result<(), ApiKeyServiceError> {
        let _ = self.app_state;
        todo!()
    }
}

#[derive(Error, Debug)]
pub enum ApiKeyServiceError {
    #[error("Failed to verify key.")]
    FailedToVerify,
}
