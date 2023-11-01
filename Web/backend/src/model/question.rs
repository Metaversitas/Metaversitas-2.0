use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Type};

#[derive(Clone, Debug, Type, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
#[sqlx(type_name = "question_types")]
pub enum QuestionType {
    Choice,
    Descriptive,
    Table,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct ChoiceAnswerBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_correct: Option<bool>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub question_id: String,
    pub question_type: QuestionType,
    pub question_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_question: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<Vec<ChoiceAnswerBody>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub key_answer: Option<KeyAnswerOfQuestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub question_type: QuestionType,
    pub question_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_question: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice: Option<Vec<ChoiceAnswerBody>>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub key_answer: Option<KeyAnswerOfQuestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateQuestionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_type: Option<QuestionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_question: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_answer: Option<KeyAnswerOfQuestion>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyAnswerOfQuestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choice_answer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_answer: Option<Value>,
}
