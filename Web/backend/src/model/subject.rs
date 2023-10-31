use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub subject_id: String,
    pub name: String,
}
