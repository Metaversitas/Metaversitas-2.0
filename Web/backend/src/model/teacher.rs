use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher {
    pub teacher_id: String,
    pub user_id: String,
}
