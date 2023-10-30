use crate::backend::AppState;
use crate::helpers::errors::auth::AuthError;
use crate::helpers::errors::question::QuestionControllerError;
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::model::question::{
    ChoiceAnswerBody, CreateQuestion, CreateQuestionParams, Question,
    QuestionType, UpdateQuestionParams,
};
use crate::model::user::{UserRole, UserUniversityRole};
use crate::service::question::QuestionService;
use crate::service::user::UserService;
use anyhow::anyhow;
use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{response::Response, Json, Router};
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, FromRef)]
pub struct QuestionServiceRouter {
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
    question_service: Arc<QuestionService>,
}

pub const QUESTION_ROUTER_PATH: &str = "/question";
pub async fn question_router(
    app_state: Arc<AppState>,
    user_service: Arc<UserService>,
    question_service: Arc<QuestionService>,
) -> Router {
    let question_service_router = QuestionServiceRouter {
        app_state,
        user_service,
        question_service,
    };

    Router::new()
        .route(HOME_QUESTION_PATH, post(create_question))
        .route(
            QUESTION_ID_PATH,
            get(get_question)
                .put(update_question)
                .delete(delete_question),
        )
        .with_state(question_service_router)
}

pub const HOME_QUESTION_PATH: &str = "/";
pub const QUESTION_ID_PATH: &str = "/:id";
pub async fn get_question(
    State(question_service): State<Arc<QuestionService>>,
    State(app_state): State<Arc<AppState>>,
    Path(question_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    let auth_user = is_auth_user?;

    if matches!(auth_user.user_role, UserRole::Administrator)
        || matches!(auth_user.user_role, UserRole::Staff)
    {
        todo!()
    }

    let question_id = Uuid::from_str(question_id.as_str()).map_err(|err| {
        QuestionControllerError::ErrorWithMessage(anyhow!(
            "Unable to parse question_id, with an error: {}",
            err.to_string()
        ))
    })?;

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        QuestionControllerError::Unknown
    })?;

    let question = question_service
        .get_question_by_id(&mut transaction, question_id.to_string().as_str())
        .await
        .map_err(|err| {
            let err_message = err.to_string();

            tracing::error!(err_message);
            if err_message.contains("Not found") {
                QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Not found a question with current id: {}",
                    question_id.to_string()
                ))
            } else {
                QuestionControllerError::Unknown
            }
        })?;
    let response = json!({"data": question});

    Ok((StatusCode::OK, Json(response)).into_response())
}

pub async fn update_question(
    State(app_state): State<Arc<AppState>>,
    State(question_service): State<Arc<QuestionService>>,
    Path(question_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<UpdateQuestionParams>, JsonRejection>,
) -> Result<Response, QuestionControllerError> {
    let auth_user = is_auth_user?;

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
            "Student not able to update questions"
        )));
    }

    let payload = {
        let Json(payload) = payload?;
        payload
    };

    if payload.table_question.is_none()
        && payload.question_text.is_none()
        && payload.key_answer.is_none()
        && payload.question_type.is_none()
    {
        return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
            "All params is unavailable, please check again"
        )));
    }

    let question_id = Uuid::parse_str(question_id.as_str()).map_err(|err| {
        tracing::error!("{}", err.to_string());
        QuestionControllerError::ErrorWithMessage(anyhow!(
            "Not able to parse question_id, with an error: {}",
            err.to_string()
        ))
    })?;

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        QuestionControllerError::Unknown
    })?;

    // question is exists
    let question = question_service
        .get_question_by_id(&mut transaction, question_id.to_string().as_str())
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionControllerError::ErrorWithMessage(anyhow!(
                "Unable to get a question with an id: {}",
                question_id.to_string()
            ))
        })?;

    if payload.table_question.is_some()
        || payload.question_text.is_some()
        || payload.question_type.is_some()
    {
        question_service
            .update_question_by_id(&mut transaction, question.question_id.as_str(), &payload)
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                QuestionControllerError::Unknown
            })?;
    }

    if let Some(key_answer) = payload.key_answer {
        let question = question_service
            .get_question_by_id(&mut transaction, question.question_id.as_str())
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                QuestionControllerError::Unknown
            })?;
        let previous_key_answer = question_service
            .get_key_answer_of_question(&mut transaction, question.question_id.as_str())
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                QuestionControllerError::Unknown
            })?;

        if let Some(choice_answer_id) = &key_answer.choice_answer_id {
            if !matches!(question.question_type, QuestionType::Choice) {
                return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Current question type is not a choice, change it first."
                )));
            }

            //pre
            let choice_answer = ChoiceAnswerBody {
                choice_id: previous_key_answer.choice_answer_id.clone(),
                text: None,
                is_correct: Some(false),
            };
            question_service
                .update_choice_question(
                    &mut transaction,
                    question.question_id.as_str(),
                    previous_key_answer
                        .choice_answer_id
                        .clone()
                        .ok_or(QuestionControllerError::Unknown)?
                        .as_str(),
                    &choice_answer,
                )
                .await
                .map_err(|err| {
                    tracing::error!("{}", err.to_string());
                    QuestionControllerError::Unknown
                })?;

            //post
            let choice_answer = ChoiceAnswerBody {
                choice_id: Some(choice_answer_id.to_string()),
                text: None,
                is_correct: Some(true),
            };
            question_service
                .update_choice_question(
                    &mut transaction,
                    question.question_id.as_str(),
                    choice_answer_id.as_str(),
                    &choice_answer,
                )
                .await
                .map_err(|err| {
                    tracing::error!("{}", err.to_string());
                    QuestionControllerError::Unknown
                })?;
        }

        if key_answer.text_answer.is_some() && !matches!(question.question_type, QuestionType::Descriptive) {
            return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Current question type is not a descriptive, change it first."
            )));
        }

        if key_answer.table_answer.is_some() && !matches!(question.question_type, QuestionType::Table) {
            return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                "Current question type is not a table, change it first."
            )));
        }

        question_service
            .update_key_answer_of_question(
                &mut transaction,
                question_id.to_string().as_str(),
                &key_answer,
            )
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                QuestionControllerError::Unknown
            })?;
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("{}", err.to_string());

        QuestionControllerError::Unknown
    })?;

    let response = json!({"message": "Question has been updated!"});

    Ok((StatusCode::CREATED, Json(response)).into_response())
}

pub async fn delete_question(
    State(app_state): State<Arc<AppState>>,
    State(question_service): State<Arc<QuestionService>>,
    Path(question_id): Path<String>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
) -> Result<Response, QuestionControllerError> {
    let auth_user = is_auth_user?;

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
            "Student not able to delete questions"
        )));
    }

    let mut transaction = app_state.database.begin().await.map_err(|err| {
        tracing::error!(
            "Failed to acquire a Postgres connection, with an error: {}",
            err.to_string()
        );
        QuestionControllerError::Unknown
    })?;

    let question_id = Uuid::parse_str(question_id.as_str()).map_err(|err| {
        tracing::error!("{}", err.to_string());
        QuestionControllerError::ErrorWithMessage(anyhow!("{}", err.to_string()))
    })?;

    question_service
        .delete_question(&mut transaction, question_id.to_string().as_str())
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionControllerError::Unknown
        })?;

    let response = json!({"message": "Question has been deleted."});

    Ok((StatusCode::NO_CONTENT, Json(response)).into_response())
}

pub async fn create_question(
    State(app_state): State<Arc<AppState>>,
    State(question_service): State<Arc<QuestionService>>,
    is_auth_user: Result<AuthenticatedUserWithRole, AuthError>,
    payload: Result<Json<CreateQuestionParams>, JsonRejection>,
) -> Result<Response, QuestionControllerError> {
    let auth_user = is_auth_user?;
    let create_question_params = {
        let Json(payload) = payload?;
        payload
    };

    if matches!(auth_user.user_role, UserRole::User)
        && matches!(auth_user.university_role, UserUniversityRole::Mahasiswa)
    {
        return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
            "Student not able to create questions"
        )));
    }

    let mut transaction = app_state.database.begin().await.map_err(|_| {
        tracing::error!("Failed to acquire a Postgres Connection from the pool");
        QuestionControllerError::Unknown
    })?;

    let create_question = CreateQuestion {
        question_type: create_question_params.question_type,
        question_text: create_question_params.question_text,
        table_question: create_question_params.table_question,
        key_answer: None,
        choice: None,
    };

    let question = question_service
        .create_question(&mut transaction, &create_question)
        .await
        .map_err(|err| {
            let error_msg = err.to_string();
            tracing::error!(error_msg);

            QuestionControllerError::Unknown
        })?;

    let response: Value;

    match create_question.question_type {
        QuestionType::Choice => match create_question_params.choice_answers {
            None => {
                return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Not found a list of choice answer provided."
                )))
            }
            Some(choice_answers) => {
                let created_choice_answers = question_service
                    .create_choice_answer_from_question(
                        &mut transaction,
                        question.question_id.as_str(),
                        choice_answers,
                    )
                    .await
                    .map_err(|err| {
                        tracing::error!("{}", err.to_string());
                        QuestionControllerError::Unknown
                    })?;

                let mut true_count = 0;
                let mut true_choice_id: String = String::new();
                for choice_answer in &created_choice_answers {
                    if let Some(is_correct) = choice_answer.is_correct {
                        if is_correct {
                            if let Some(choice_id) = &choice_answer.choice_id {
                                true_choice_id = choice_id.to_string();
                            }
                            true_count += 1;
                        }
                    }
                }

                let created_choice_answer_id = question_service
                    .create_key_choice_answer_from_question(
                        &mut transaction,
                        question.question_id.as_str(),
                        true_choice_id.as_str(),
                    )
                    .await
                    .map_err(|err| {
                        tracing::error!("{}", err.to_string());
                        QuestionControllerError::Unknown
                    })?;

                if true_count == 0 {
                    return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                        "Check the provided choice answer at least 1 true answer is required."
                    )));
                }

                let created_question = Question {
                    question_id: question.question_id,
                    question_type: question.question_type,
                    question_text: question.question_text,
                    table_question: question.table_question,
                    choice: Some(created_choice_answers),
                    key_answer: Some(created_choice_answer_id),
                };

                response =
                    json!({"message": "Question has been created", "data": created_question});
            }
        },
        QuestionType::Descriptive => match create_question_params.text_answer {
            None => {
                return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Not found a key of text answer on the provided params."
                )));
            }
            Some(text_answer) => {
                let created_text_answer = question_service
                    .create_key_text_answer_from_question(
                        &mut transaction,
                        question.question_id.as_str(),
                        text_answer.as_str(),
                    )
                    .await
                    .map_err(|err| {
                        tracing::error!("{}", err.to_string());
                        QuestionControllerError::Unknown
                    })?;
                response =
                    json!({"message": "Question has been created", "data": created_text_answer});
            }
        },
        QuestionType::Table => match create_question_params.table_answer {
            None => {
                return Err(QuestionControllerError::ErrorWithMessage(anyhow!(
                    "Not found an answer of table provided."
                )))
            }
            Some(table_answer) => {
                let created_table_answer = question_service
                    .create_key_table_answer_from_question(
                        &mut transaction,
                        question.question_id.as_str(),
                        table_answer,
                    )
                    .await
                    .map_err(|err| {
                        tracing::error!("{}", err.to_string());
                        QuestionControllerError::Unknown
                    })?;
                response =
                    json!({"message": "Question has been created", "data": created_table_answer});
            }
        },
    }

    transaction.commit().await.map_err(|err| {
        tracing::error!("{}", err.to_string());

        QuestionControllerError::Unknown
    })?;

    Ok((StatusCode::CREATED, Json(response)).into_response())
}
