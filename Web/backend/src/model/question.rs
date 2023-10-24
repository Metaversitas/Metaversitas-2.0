use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Type};

#[derive(Clone, Debug, Type, Serialize, Deserialize)]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "question_type")]
pub enum QuestionType {
    Choice,
    Descriptive,
    Table,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChoiceAnswerBody {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateQuestionParams {
    pub question_text: String,
    pub table_question: Option<Value>,
    pub question_type: QuestionType,
    pub choice_answers: Option<Vec<ChoiceAnswerBody>>,
    pub text_answer: Option<String>,
    pub table_answer: Option<Value>,
}