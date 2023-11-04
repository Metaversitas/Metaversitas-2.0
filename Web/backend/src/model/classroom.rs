use crate::model::exam::{CreateExamParams, Exam, UpdateExamParams};
use crate::model::student::Student;
use crate::model::subject::{Subject, SubjectWithSecondary};
use crate::model::teacher::Teacher;
use chrono::{DateTime, Utc};
use redis_macros::FromRedisValue;
use serde::de::{MapAccess, Visitor};
use serde::{de, Deserialize, Deserializer, Serialize};
use sqlx::Type;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Type, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "semester")]
#[sqlx(rename_all = "lowercase")]
pub enum ClassSemester {
    Odd,
    Even,
}

#[derive(Serialize, Deserialize, Debug, Clone, FromRedisValue)]
pub struct Classroom {
    pub class_id: String,
    pub class_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub is_active: bool,
    pub capacity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_meeting_id: Option<String>,
    #[serde(skip_serializing)]
    pub have_multiple_meeting: bool,
    #[serde(flatten)]
    pub subject: SubjectWithSecondary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teachers: Option<Vec<TeacherClassroom>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meetings: Option<Vec<ClassMeeting>>,
    pub semester: ClassSemester,
    pub year_start: String,
    pub year_end: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enrolled: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeacherClassroom {
    #[serde(skip)]
    pub class_id: String,
    pub teacher_id: String,
    pub teacher_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StudentClassroom {
    pub class_id: String,
    pub student_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CreateClassroomParams {
    pub class_name: String,
    pub semester: ClassSemester,
    pub year_start: String,
    pub year_end: String,
    pub capacity: Option<i64>,
    pub description: Option<String>,
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub secondary_subject_id: Option<String>,
    pub secondary_subject_name: Option<String>,
    pub students: Option<Vec<Student>>,
    pub teachers: Option<Vec<Teacher>>,
    pub meetings: Option<Vec<CreateClassMeetingParams>>,
    pub exams: Option<Vec<ExamParams>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassMeeting {
    pub meeting_id: String,
    #[serde(skip_serializing)]
    pub class_id: String,
    pub meeting_number: i64,
    pub is_active: bool,
    pub meeting_name: String,
    pub topic_description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "upcoming_type")]
pub enum UpcomingScheduledMeetingOrClass {
    Class(Classroom),
    Meeting(ClassMeeting),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpcomingScheduled {
    pub class_id: String,
    pub class_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teachers: Option<Vec<TeacherClassroom>>,
    #[serde(flatten)]
    pub subject: SubjectWithSecondary,
    #[serde(flatten)]
    pub upcoming: UpcomingScheduledMeetingOrClass,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExamParams {
    pub exam_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CreateClassMeetingParams {
    pub meeting_name: String,
    pub meeting_number: i64,
    pub topic_description: String,
    pub description: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub exams: Option<Vec<ExamParams>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "id")]
pub enum Action {
    Add(String),
    Delete(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "action")]
pub enum ActionType {
    All(Vec<Action>),
    Single(Vec<Action>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BaseAction {
    Add,
    Delete,
    Edit,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ParamsActionUpdateExam {
    Create(CreateExamParams),
    Update(UpdateExamParams),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionUpdateExam {
    pub action: BaseAction,
    #[serde(flatten)]
    pub params: ParamsActionUpdateExam,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "action")]
pub enum ActionTypeUpdateExam {
    All(Vec<ActionUpdateExam>),
    Single(Vec<ActionUpdateExam>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionUpdateClassroom {
    pub action: BaseAction,
    #[serde(flatten)]
    pub params: UpdateClassroomParams,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ParamsActionUpdateClassMeeting {
    Create {
        create_meeting_name: String,
        create_meeting_number: i64,
        create_topic_description: String,
        create_description: Option<String>,
        create_start_time: Option<DateTime<Utc>>,
        create_end_time: Option<DateTime<Utc>>,
        create_exams: Option<Vec<ExamParams>>,
    },
    Update {
        update_meeting_id: String,
        update_meeting_number: Option<i64>,
        update_meeting_name: Option<String>,
        update_is_active: Option<bool>,
        update_topic_description: Option<String>,
        update_description: Option<String>,
        update_start_time: Option<DateTime<Utc>>,
        update_end_time: Option<DateTime<Utc>>,
        // Update exams params
        update_exams: Option<ActionTypeUpdateExam>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActionUpdateClassMeeting {
    pub action: BaseAction,
    #[serde(flatten)]
    pub params: ParamsActionUpdateClassMeeting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "action")]
pub enum ActionTypeUpdateClassMeeting {
    All(Vec<ActionUpdateClassMeeting>),
    Single(Vec<ActionUpdateClassMeeting>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "action")]
pub enum ActionTypeUpdateClassroom {
    All(Vec<ActionUpdateClassroom>),
    Single(Vec<ActionUpdateClassroom>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateClassMeetingParams {
    pub meeting_id: String,
    pub meeting_number: Option<i64>,
    pub is_active: Option<bool>,
    pub meeting_name: Option<String>,
    pub topic_description: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    // Update exams params
    pub exams: Option<ActionTypeUpdateExam>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct UpdateClassroomParams {
    pub class_name: Option<String>,
    pub semester: Option<ClassSemester>,
    pub year_start: Option<String>,
    pub year_end: Option<String>,
    pub capacity: Option<i64>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub subjects: Option<UpdateClassSubjectParams>,
    // Update meetings params
    pub meetings: Option<ActionTypeUpdateClassMeeting>,
    // Update exams
    pub exams: Option<ActionTypeUpdateExam>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub subject_id: Option<String>,
    pub subject_name: Option<String>,
    pub students: Option<ActionType>,
    pub teachers: Option<ActionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteClassroomParams {
    pub class_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateClassSubjectParams {
    pub subject_id: Option<String>,
    pub secondary_subject_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryParamsClassMode {
    AvailableClass,
    CreatedClass,
    EnrolledClass,
    UpcomingScheduledClass,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QuerySemesterFilterClass {
    Odd,
    Even,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryFilter {
    pub semester_filter: Option<QuerySemesterFilterClass>,
    pub subject_name_filter: Option<String>,
    pub subject_id_filter: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryPagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryParamsClasses {
    #[serde(flatten)]
    pub pagination: Option<QueryPagination>,
    pub mode: Option<QueryParamsClassMode>,
    pub search: Option<String>,
    #[serde(flatten)]
    pub filter: Option<QueryFilter>,
}

impl Default for QueryParamsClasses {
    fn default() -> Self {
        Self {
            pagination: None,
            mode: None,
            filter: None,
            search: None,
        }
    }
}
