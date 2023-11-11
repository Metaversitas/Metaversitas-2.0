use crate::model::question::Question;
use crate::model::subject::SubjectWithSecondary;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "exam_type")]
#[sqlx(rename_all = "lowercase")]
pub enum ExamType {
    Default,
    Upload,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exam {
    pub exam_id: String,
    pub r#type: ExamType,
    pub exam_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamWithSubject {
    #[serde(flatten)]
    pub exam: Exam,
    #[serde(flatten)]
    pub subject: SubjectWithSecondary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExamWithQuestion {
    #[serde(flatten)]
    pub exam: Exam,
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateExamParams {
    pub exam_name: String,
    pub r#type: ExamType,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateExamParams {
    pub exam_id: String,
    pub exam_name: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<ExamType>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QueryParamsExam {
    pub subject_id: Option<String>,
}
