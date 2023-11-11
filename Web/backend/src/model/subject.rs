use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Subject {
    pub subject_id: String,
    pub subject_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondarySubject {
    pub secondary_subject_id: String,
    pub secondary_subject_name: String,
    #[serde(skip_serializing)]
    pub subject_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectWithSecondary {
    pub subject_id: String,
    pub subject_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_subject: Option<SecondarySubject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectWithSecondaryList {
    pub subject_id: String,
    pub subject_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_subjects: Option<Vec<SecondarySubject>>,
}
