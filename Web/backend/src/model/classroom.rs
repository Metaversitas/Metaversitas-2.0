use redis_macros::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, FromRedisValue)]
pub struct Classroom {
    pub is_active: bool,
    pub class_id: String,
    pub subject_id: String,
    pub subject_name: String,
}

#[derive(Debug, Clone)]
pub struct SubjectClassroom {
    pub subject_id: String,
    pub subject_name: String,
}

pub struct TeacherClassroom {
    pub class_id: String,
    pub teacher_id: String,
}

pub struct StudentClassroom {
    pub class_id: String,
    pub student_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClassroomParams {
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
}
