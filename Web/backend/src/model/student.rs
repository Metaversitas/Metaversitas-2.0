use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Student {
    pub student_id: String,
    pub user_id: String,
}
