use crate::backend::AppState;
use crate::helpers::errors::{AuthError, ClassroomControllerError};
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::model::classroom::{Classroom, CreateClassroomParams, SubjectClassroom};
use crate::model::user::{UserRole, UserUniversityRole};
use crate::service::classroom::ClassroomService;
use crate::service::user::UserService;
use anyhow::{anyhow, Context};
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::*;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use axum::extract::rejection::JsonRejection;

#[derive(Clone, FromRef)]
pub struct ClassroomRouterService {
    pub app_state: Arc<AppState>,
    pub classroom_service: Arc<ClassroomService>,
    pub user_service: Arc<UserService>,
}

pub const CLASSROOM_PATH_CONTROLLER: &str = "/classroom";
pub async fn classroom_router(
    app_state: Arc<AppState>,
    classroom_service: Arc<ClassroomService>,
    user_service: Arc<UserService>,
) -> Router {
    let classroom_router_service = ClassroomRouterService {
        app_state: Arc::clone(&app_state),
        classroom_service: Arc::clone(&classroom_service),
        user_service: Arc::clone(&user_service),
    };

    // 1. Get all current classes for current user
    // 2. Create classes only for lecturer
    // 3. Update classes only for lecturer
    // 4. Delete classes only for lecturer

    Router::new()
        .route(GET_CLASSROOM_PATH, get(get_available_classes))
        .route(CREATE_CLASSROOM_PATH, post(create_classes))
        .with_state(classroom_router_service.clone())
}

pub const GET_CLASSROOM_PATH: &str = "/";
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
    auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<CreateClassroomParams>, JsonRejection>
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

    let mut subject_classroom: Option<SubjectClassroom> = None;

    if payload.subject_name.is_none() && payload.subject_id.is_none() {
        return Err(ClassroomControllerError::Other(anyhow!("Insert either subject_name or subject_id")));
    }

    if let Some(subject_name) = payload.subject_name {
        subject_classroom = Some(classroom_service
            .get_subject_by_name(&mut transaction, subject_name.as_str())
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                ClassroomControllerError::Unknown
            })?);
    }

    if let Some(subject_id) = payload.subject_id {
        subject_classroom = Some(classroom_service.get_subject_by_id(&mut transaction, subject_id.as_str())
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                ClassroomControllerError::Unknown
            })?);
    }

    let subject_classroom = match subject_classroom {
        None => {
            tracing::error!("subject_classroom is a none!");
            return Err(ClassroomControllerError::Unknown)
        }
        Some(v) => v,
    };

    let created_class_id = classroom_service
        .create_classroom(
            &mut transaction,
            auth_user.user_id.as_str(),
            &auth_user.user_role,
            &auth_user.university_role,
            subject_classroom.subject_id.as_str(),
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
        subject_name: subject_classroom.subject_name.to_owned(),
    };

    let payload = json!({
        "message": "Class created successfully!",
        "data": classroom
    });
    Ok((StatusCode::CREATED, Json(payload)).into_response())
}
