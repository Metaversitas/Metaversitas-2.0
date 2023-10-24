use std::sync::Arc;
use crate::backend::AppState;
use crate::helpers::errors::question::QuestionServiceError;
use crate::r#const::PgTransaction;

pub struct QuestionService {
    app_state: Arc<AppState>
}

impl QuestionService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self {
            app_state
        }
    }

    pub async fn get_question_by_id(&self, transaction: &mut PgTransaction, question_id: &str) -> Result<(), QuestionServiceError> {
        todo!()
    }

    pub async fn delete_question(&self, transaction: &mut PgTransaction, question_id: &str) -> Result<(), QuestionServiceError> {
        todo!()
    }

    pub async fn create_question(&self, transaction: &mut PgTransaction) -> Result<(), QuestionServiceError> {
        todo!()
    }
}