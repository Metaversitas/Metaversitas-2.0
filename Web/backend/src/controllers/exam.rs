use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::exam::ExamControllerError;
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::model::exam::{CreateExamParams, Exam, QueryParamsExam, UpdateExamParams};
use crate::model::user::{UserRole, UserUniversityRole};
use crate::service::exam::ExamService;
use crate::service::user::UserService;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRef, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, FromRef)]
pub struct ExamServiceRouter {
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
    exam_service: Arc<ExamService>,
}

pub const EXAM_CONTROLLER_PATH: &str = "/exam";
pub async fn exam_router(
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
    exam_service: Arc<ExamService>,
) -> Router {
    let exam_service_router = ExamServiceRouter {
        app_state,
        user_service,
        exam_service,
    };
    Router::new()
        .route(HOME_EXAM_PATH, get(get_available_exams).post(create_exam))
        .route(
            EXAM_ID_PATH,
            get(get_exam_with_id).put(update_exam).delete(delete_exam),
        )
        .with_state(exam_service_router)
}

pub const HOME_EXAM_PATH: &str = "/";
pub const EXAM_ID_PATH: &str = "/:id";

pub async fn get_exam_with_id(
    State(app_state): State<Arc<AppState>>,
    State(exam_service): State<Arc<ExamService>>,
    Path(exam_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, ExamControllerError> {
    let _auth_user = is_auth_user?;
    let exam_id = Uuid::parse_str(exam_id.as_str()).map_err(|err| {
        ExamControllerError::ErrorWithMessage(anyhow!(
            "Failed to parse exam_id, with an error: {}",
            err.to_string()
        ))
    })?;
    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ExamControllerError::Unknown
    })?;

    let exam = exam_service
        .get_exam_by_id(&mut transaction, exam_id.to_string().as_str())
        .await
        .map_err(|err| {
            let error_msg = err.to_string();
            tracing::error!("Got an error while do get_exam_by_id, err: {}", error_msg);

            if error_msg.contains("Not found") {
                return ExamControllerError::ErrorWithMessage(anyhow!(
                    "Not found an exam with id: {}",
                    exam_id.to_string()
                ));
            }
            ExamControllerError::Unknown
        })?;

    let response = json!({"data": exam});

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn get_available_exams(
    State(app_state): State<Arc<AppState>>,
    State(exam_service): State<Arc<ExamService>>,
    params: Option<Query<QueryParamsExam>>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, ExamControllerError> {
    let _auth_user = is_auth_user?;
    let Query(params) = params.unwrap_or_default();

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ExamControllerError::Unknown
    })?;

    let exam: Vec<Exam>;

    if let Some(subject_id) = params.subject_id {
        exam = exam_service
            .get_exams_by_subject_id(&mut transaction, subject_id.as_str())
            .await
            .map_err(|err| {
                let error_msg = err.to_string();
                tracing::error!("Got an error while getting exams, err: {}", error_msg);

                if error_msg.contains("Not found") {
                    return ExamControllerError::ErrorWithMessage(anyhow!(
                        "Not found any available exam",
                    ));
                }
                ExamControllerError::Unknown
            })?;
    } else {
        exam = exam_service
            .get_available_exams(&mut transaction)
            .await
            .map_err(|err| {
                let error_msg = err.to_string();
                tracing::error!("Got an error while getting exams, err: {}", error_msg);

                if error_msg.contains("Not found") {
                    return ExamControllerError::ErrorWithMessage(anyhow!(
                        "Not found any available exam",
                    ));
                }
                ExamControllerError::Unknown
            })?;
    }

    let response = json!({"data": exam});

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn create_exam(
    State(app_state): State<Arc<AppState>>,
    State(exam_service): State<Arc<ExamService>>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<CreateExamParams>, JsonRejection>,
) -> Result<Response, ExamControllerError> {
    let auth_user = is_auth_user?;
    let payload = {
        let Json(payload) = payload
            .map_err(|err| ExamControllerError::ErrorWithMessage(anyhow!(err.to_string())))?;
        payload
    };

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(ExamControllerError::ErrorWithMessage(anyhow!(
            "Student not able to create an exam"
        )));
    }

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ExamControllerError::Unknown
    })?;

    exam_service
        .create_exam(&mut transaction, auth_user.user_id.as_str(), &payload)
        .await
        .map_err(|err| {
            tracing::error!(
                "Unable to create an exam, with an error: {}",
                err.to_string()
            );
            ExamControllerError::Unknown
        })?;

    transaction.commit().await.map_err(|err| {
        tracing::error!(
            "Unable to commit transaction to database, with an error: {}",
            err.to_string()
        );
        ExamControllerError::Unknown
    })?;

    let response = json!({"message": "Exam has been created"});
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

pub async fn delete_exam(
    State(app_state): State<Arc<AppState>>,
    State(exam_service): State<Arc<ExamService>>,
    Path(exam_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, ExamControllerError> {
    let auth_user = is_auth_user?;
    let exam_id = Uuid::parse_str(exam_id.as_str()).map_err(|err| {
        ExamControllerError::ErrorWithMessage(anyhow!(
            "Failed to parse exam_id, with an error: {}",
            err.to_string()
        ))
    })?;

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(ExamControllerError::ErrorWithMessage(anyhow!(
            "Student should not be able to delete an exam"
        )));
    }

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ExamControllerError::Unknown
    })?;

    // get data
    let exam = exam_service
        .get_exam_by_id(&mut transaction, exam_id.to_string().as_str())
        .await
        .map_err(|err| {
            tracing::error!(
                "not able to get an exam by id: {}, with an error: {}",
                exam_id.to_string(),
                err.to_string()
            );
            ExamControllerError::Unknown
        })?;

    if exam.created_by != auth_user.user_id {
        return Err(ExamControllerError::ErrorWithMessage(anyhow!(
            "Unable to delete exam user isn't the one who create the exam."
        )));
    }

    exam_service
        .delete_exam_by_id(&mut transaction, exam.exam_id.as_str())
        .await
        .map_err(|err| {
            tracing::error!(
                "not able to delete an exam by id: {}, with an error: {}",
                exam_id.to_string(),
                err.to_string()
            );
            ExamControllerError::Unknown
        })?;

    let response = json!({"message": "Exam has been deleted"});
    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn update_exam(
    State(app_state): State<Arc<AppState>>,
    State(exam_service): State<Arc<ExamService>>,
    Path(exam_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<UpdateExamParams>, JsonRejection>,
) -> Result<Response, ExamControllerError> {
    let auth_user = is_auth_user?;
    let exam_id = Uuid::parse_str(exam_id.as_str()).map_err(|err| {
        ExamControllerError::ErrorWithMessage(anyhow!(
            "Failed to parse exam_id, with an error: {}",
            err.to_string()
        ))
    })?;
    let payload = {
        let Json(payload) = payload
            .map_err(|err| ExamControllerError::ErrorWithMessage(anyhow!("{}", err.to_string())))?;
        payload
    };

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(ExamControllerError::ErrorWithMessage(anyhow!(
            "Student should not be able to update an exam"
        )));
    }

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        ExamControllerError::Unknown
    })?;

    // get exam
    let exam = exam_service
        .get_exam_by_id(&mut transaction, exam_id.to_string().as_str())
        .await
        .map_err(|err| {
            tracing::error!(
                "Not able to get an exam by id: {}, with an error: {}",
                exam_id.to_string(),
                err.to_string()
            );
            ExamControllerError::Unknown
        })?;

    exam_service
        .update_exam_by_id(&mut transaction, exam.exam_id.as_str(), &payload)
        .await
        .map_err(|err| {
            let err_msg = err.to_string();
            tracing::error!(
                "Unable to update an exam by id: {}; Error: {}",
                exam_id.to_string(),
                err_msg
            );

            if err_msg.contains("No value to be updated") {
                return ExamControllerError::ErrorWithMessage(anyhow!("No value to be updated"));
            }

            ExamControllerError::Unknown
        })?;

    transaction.commit().await.map_err(|err| {
        tracing::error!(
            "Unable to commit transaction, error from database: {}",
            err.to_string()
        );
        ExamControllerError::Unknown
    })?;

    let response = json!({"message": "Successfully update an exam"});

    Ok((StatusCode::OK, Json(response)).into_response())
}
