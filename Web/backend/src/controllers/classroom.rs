use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::classroom::ClassroomControllerError;
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::model::classroom::{Classroom, CreateClassroomParams, UpdateClassroomParams};
use crate::model::subject::Subject;
use crate::model::user::{UserRole, UserUniversityRole};
use crate::service::classroom::ClassroomService;
use crate::service::subject::SubjectService;
use crate::service::teacher::TeacherService;
use crate::service::user::UserService;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::*;
use axum::{Json, Router};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, FromRef)]
pub struct ClassroomRouterService {
    pub app_state: Arc<AppState>,
    pub classroom_service: Arc<ClassroomService>,
    pub user_service: Arc<UserService>,
    pub teacher_service: Arc<TeacherService>,
    pub subject_service: Arc<SubjectService>,
}

pub const CLASSROOM_PATH_CONTROLLER: &str = "/classroom";
pub async fn classroom_router(
    app_state: Arc<AppState>,
    classroom_service: Arc<ClassroomService>,
    user_service: Arc<UserService>,
    teacher_service: Arc<TeacherService>,
    subject_service: Arc<SubjectService>,
) -> Router {
    let classroom_router_service = ClassroomRouterService {
        app_state: Arc::clone(&app_state),
        classroom_service: Arc::clone(&classroom_service),
        user_service: Arc::clone(&user_service),
        teacher_service: Arc::clone(&teacher_service),
        subject_service: Arc::clone(&subject_service),
    };

    Router::new()
        .route(
            HOME_CLASSROOM_PATH,
            get(get_available_classes).post(create_classes),
        )
        .route(UPDATE_CLASSROOM_WITH_ID_PATH, put(update_classroom))
        .with_state(classroom_router_service.clone())
}

pub const HOME_CLASSROOM_PATH: &str = "/";
pub async fn get_available_classes(
    State(classroom_service): State<Arc<ClassroomService>>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, ClassroomControllerError> {
    // 1. Authenticated as a student or lecturer
    // 2. get available classes
    let auth_user = auth_user?;
    let available_classroom = classroom_service
        .get_available_classroom(
            auth_user.user_id.as_str(),
            &auth_user.user_role,
            &auth_user.university_role,
        )
        .await
        .map_err(|_| ClassroomControllerError::Unknown)?;

    let response = json!({
        "status": true,
        "data": available_classroom
    });

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub const CREATE_CLASSROOM_PATH: &str = "/";
pub async fn create_classes(
    State(app_state): State<Arc<AppState>>,
    State(classroom_service): State<Arc<ClassroomService>>,
    State(subject_service): State<Arc<SubjectService>>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<CreateClassroomParams>, JsonRejection>,
) -> Result<Response, ClassroomControllerError> {
    let auth_user = auth_user?;
    let payload = {
        let Json(payload) = payload?;
        payload
    };

    if matches!(auth_user.user_role, UserRole::Administrator)
        || matches!(auth_user.user_role, UserRole::Staff)
    {
        todo!()
    }

    if matches!(auth_user.university_role, UserUniversityRole::Mahasiswa) {
        return Err(ClassroomControllerError::UnableCreateClass);
    }

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ClassroomControllerError::Unknown
    })?;

    let mut subject_classroom: Option<Subject> = None;

    if payload.subject_name.is_none() && payload.subject_id.is_none() {
        return Err(ClassroomControllerError::Other(anyhow!(
            "Insert either subject_name or subject_id"
        )));
    }

    if let Some(subject_name) = payload.subject_name {
        subject_classroom = Some(
            subject_service
                .get_subject_by_name(&mut transaction, subject_name.as_str())
                .await
                .map_err(|err| {
                    ClassroomControllerError::Other(anyhow!(
                        "Error to get subject, with an error: {}",
                        err.to_string()
                    ))
                })?,
        );
    }

    if let Some(subject_id) = payload.subject_id {
        subject_classroom = Some(
            subject_service
                .get_subject_by_id(&mut transaction, subject_id.as_str())
                .await
                .map_err(|err| {
                    ClassroomControllerError::Other(anyhow!(
                        "Error to get subject, with an error: {}",
                        err.to_string()
                    ))
                })?,
        );
    }

    let subject_classroom = match subject_classroom {
        None => {
            tracing::error!("subject_classroom is a none!");
            return Err(ClassroomControllerError::Unknown);
        }
        Some(v) => v,
    };

    let created_class_id = classroom_service
        .create_classroom(
            &mut transaction,
            &auth_user.user_role,
            &auth_user.university_role,
            subject_classroom.subject_id.as_str(),
            payload.students,
            payload.teachers,
        )
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            ClassroomControllerError::UnableCreateClass
        })?;

    transaction.commit().await.map_err(|err| {
        tracing::error!("{}", err.to_string());
        ClassroomControllerError::UnableCreateClass
    })?;

    let classroom = Classroom {
        is_active: true,
        class_id: created_class_id.to_owned(),
        subject_id: subject_classroom.subject_id.to_owned(),
        subject_name: subject_classroom.name.to_owned(),
    };

    let payload = json!({
        "message": "Class created successfully!",
        "data": classroom
    });
    Ok((StatusCode::CREATED, Json(payload)).into_response())
}

pub const UPDATE_CLASSROOM_WITH_ID_PATH: &str = "/:id";
pub async fn update_classroom(
    State(app_state): State<Arc<AppState>>,
    State(classroom_service): State<Arc<ClassroomService>>,
    State(teacher_service): State<Arc<TeacherService>>,
    Path(classroom_id): Path<String>,
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<UpdateClassroomParams>, JsonRejection>,
) -> Result<Response, ClassroomControllerError> {
    let auth_user = auth_user?;
    let payload = {
        let Json(payload) = payload?;
        payload
    };
    let classroom_id = Uuid::from_str(classroom_id.as_str()).map_err(|_| {
        tracing::error!("Unable to parse classroom_id");
        ClassroomControllerError::Other(anyhow!("Parse classroom_id to uuid failed"))
    })?;

    if matches!(auth_user.user_role, UserRole::Administrator)
        || matches!(auth_user.user_role, UserRole::Staff)
    {
        todo!()
    }

    if matches!(auth_user.university_role, UserUniversityRole::Mahasiswa) {
        return Err(ClassroomControllerError::UnableCreateClass);
    }
    let mut transaction = app_state.database.begin().await.map_err(|err| {
        tracing::error!(
            "Failed to acquire a Postgres Connection from the pool, reason: {}",
            err.to_string()
        );
        ClassroomControllerError::Unknown
    })?;

    // Check if user has a permission to edit the classroom
    let teacher = teacher_service
        .get_teacher_by_id(&mut transaction, auth_user.user_id.as_str())
        .await?;
    let classroom = classroom_service
        .get_teacher_classroom_by_id(
            &mut transaction,
            classroom_id.to_string().as_str(),
            teacher.teacher_id.as_str(),
        )
        .await?;

    if teacher.teacher_id != classroom.teacher_id {
        return Err(ClassroomControllerError::Unauthorized);
    }

    classroom_service
        .update_classroom(&mut transaction, classroom_id.to_string(), payload)
        .await
        .map_err(|err| {
            tracing::error!(
                "Error happened on update_classroom, with an error: {}",
                err.to_string()
            );
            ClassroomControllerError::Unknown
        })?;

    transaction.commit().await.map_err(|_| {
        tracing::error!("Unable to commit transaction into Postgres");
        ClassroomControllerError::Unknown
    })?;

    let response = json!({
        "status": true,
        "message": "Classroom has been updated successfully!"
    });

    Ok((StatusCode::CREATED, Json(response)).into_response())
}
