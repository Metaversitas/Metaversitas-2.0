use redis_macros::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRedisValue)]
pub struct Classroom {
    pub is_active: bool,
    pub name: String,
    pub description: String,
    pub class_id: String,
    pub subject_id: String,
    pub subject_name: String,
    pub start_time: String,
    pub end_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedClassroom {
    pub class_id: String,
    pub subject_id: String,
    pub subject_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherClassroom {
    pub class_id: String,
    pub teacher_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentClassroom {
    pub class_id: String,
    pub student_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClassroomParams {
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub students: Option<Vec<String>>,
    pub teachers: Option<Vec<String>>,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "id")]
pub enum Action {
    Add(String),
    Delete(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum ActionType {
    All(Vec<Action>),
    Single(Vec<Action>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateClassroomParams {
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub students: Option<ActionType>,
    pub teachers: Option<ActionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteClassroomParams {
    pub class_ids: Vec<String>,
}
