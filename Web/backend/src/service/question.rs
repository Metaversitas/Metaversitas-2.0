use crate::helpers::errors::question::QuestionServiceError;
use crate::model::question::{
    ChoiceAnswerBody, CreateQuestion, KeyAnswerOfQuestion, Question, QuestionType,
    UpdateQuestionParams,
};
use anyhow::anyhow;
use sqlx::{PgConnection, Postgres, QueryBuilder, Row};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

pub struct QuestionService;

impl QuestionService {
    pub fn new() -> Self {
        Self
    }

    #[allow(unused_assignments)]
    pub async fn update_question_by_id(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        params: &UpdateQuestionParams,
    ) -> Result<(), QuestionServiceError> {
        if params.question_type.is_none()
            && params.table_question.is_none()
            && params.question_text.is_none()
        {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "All params are not setting a new value"
            )));
        }

        let retrieve_question = self.get_question_by_id(conn, question_id).await?;

        let mut query_builder = QueryBuilder::<Postgres>::new("update questions set");
        let mut separated = query_builder.separated(", ");

        let mut count = 0;
        let mut curr_count = 0;
        let mut count_changed = 0;

        if params.question_type.is_some() {
            count += 1;
        }
        if params.table_question.is_some() {
            count += 1;
        }
        if params.question_text.is_some() {
            count += 1;
        }

        if let Some(question_type) = &params.question_type {
            if retrieve_question.question_type != *question_type {
                separated.push_unseparated(" question_type = ");
                separated.push_bind_unseparated(question_type);
                if count > 1 && curr_count != count - 1 {
                    curr_count += 1;
                    separated.push_unseparated(",");
                }
                count_changed += 1;
            }
        }

        if let Some(question_text) = &params.question_text {
            if retrieve_question.question_text != *question_text {
                separated.push_unseparated(" question_text = ");
                separated.push_bind_unseparated(question_text);
                if count > 1 && curr_count != count - 1 {
                    curr_count += 1;
                    separated.push_unseparated(",");
                }
                count_changed += 1;
            }
        }

        if let Some(table_question) = &params.table_question {
            if let Some(retrieved_table_question) = &retrieve_question.table_question {
                if retrieved_table_question != table_question {
                    separated.push_unseparated(" table_question = ");
                    separated.push_bind_unseparated(table_question);
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(",");
                    }
                    count_changed += 1;
                }
            }
        }

        if count_changed == 0 {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "No new values to be set"
            )));
        }

        separated.push(" where question_id = ");
        separated.push_bind_unseparated(Uuid::from_str(question_id).map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Unable to parse question_id with error: {}",
                err.to_string()
            ))
        })?);

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn get_question_by_id(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<Question, QuestionServiceError> {
        let query = sqlx::query!(
            r#"
        select
            question_type as "question_type!: QuestionType",
            question_id,
            question_text,
            table_question
        from questions
        where question_id = $1;
        "#,
            Uuid::from_str(question_id).map_err(|err| {
                QuestionServiceError::UnexpectedError(anyhow!(
                    "Unable to parse question_id with error: {}",
                    err.to_string()
                ))
            })?
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?
        .ok_or(QuestionServiceError::UnexpectedError(anyhow!(
            "Not found a question by an id: {}",
            question_id
        )))?;

        Ok(Question {
            question_id: query.question_id.to_string(),
            question_type: query.question_type,
            question_text: query.question_text,
            table_question: query.table_question,
            choice: None,
            key_answer: None,
        })
    }

    pub async fn get_question_full_no_answer(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<Question, QuestionServiceError> {
        let question = self.get_question_by_id(conn, question_id).await?;

        if matches!(question.question_type, QuestionType::Choice) {
            let query = sqlx::query!(
                r#"
        select
            qc.choice_id,
            qc.choice_text
        from questions
                 inner join question_choices qc on questions.question_id = qc.question_id
        where questions.question_id = $1;
        "#,
                Uuid::from_str(question_id).map_err(|err| {
                    QuestionServiceError::UnexpectedError(anyhow!(
                        "Unable to parse question_id with error: {}",
                        err.to_string()
                    ))
                })?
            )
            .fetch_all(&mut *conn)
            .await
            .map_err(|err| {
                QuestionServiceError::UnexpectedError(anyhow!(
                    "Got an error from database: {}",
                    err.to_string()
                ))
            })?;

            let mut choice_vec = Vec::new();
            for choice in query {
                choice_vec.push(ChoiceAnswerBody {
                    choice_id: Some(choice.choice_id.to_string()),
                    text: Some(choice.choice_text),
                    is_correct: None,
                })
            }

            Ok(Question {
                question_id: question.question_id,
                question_type: question.question_type,
                question_text: question.question_text,
                table_question: question.table_question,
                choice: Some(choice_vec),
                key_answer: None,
            })
        } else {
            Ok(Question {
                question_id: question.question_id,
                question_type: question.question_type,
                question_text: question.question_text,
                table_question: question.table_question,
                choice: None,
                key_answer: None,
            })
        }
    }

    pub async fn delete_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<(), QuestionServiceError> {
        sqlx::query!(
            r#"
        delete from questions
        where question_id = $1
        "#,
            Uuid::from_str(question_id).map_err(|err| {
                QuestionServiceError::UnexpectedError(anyhow!(
                    "Unable to parse question_id with error: {}",
                    err.to_string()
                ))
            })?
        )
        .execute(&mut *conn)
        .await
        .map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn create_question(
        &self,
        conn: &mut PgConnection,
        params: &CreateQuestion,
    ) -> Result<Question, QuestionServiceError> {
        let query = sqlx::query!(
            r#"
        insert into
        questions
        (question_text, question_type, table_question)
        values
        ($1, $2, $3)
        returning
        question_id, question_text, question_type as "question_type!: QuestionType", table_question;
        "#,
            params.question_text,
            params.question_type.clone() as QuestionType,
            params.table_question,
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?;

        Ok(Question {
            question_id: query.question_id.to_string(),
            question_type: query.question_type,
            question_text: query.question_text,
            table_question: query.table_question,
            choice: None,
            key_answer: None,
        })
    }

    pub async fn get_question_full_with_key_answer(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<Question, QuestionServiceError> {
        let question = self.get_question_by_id(&mut *conn, question_id).await?;

        if matches!(question.question_type, QuestionType::Choice) {
            let query = sqlx::query!(
                r#"
        select
            qc.choice_id,
            qc.choice_text,
            qc.is_correct
        from questions
        inner join question_choices qc on questions.question_id = qc.question_id
        where questions.question_id = $1;
        "#,
                Uuid::from_str(question_id).map_err(|err| {
                    QuestionServiceError::UnexpectedError(anyhow!(
                        "Unable to parse question_id with error: {}",
                        err.to_string()
                    ))
                })?
            )
            .fetch_all(&mut *conn)
            .await
            .map_err(|err| {
                QuestionServiceError::UnexpectedError(anyhow!(
                    "Got an error from database: {}",
                    err.to_string()
                ))
            })?;

            let mut choice_vec = vec![];

            for choice in query {
                choice_vec.push(ChoiceAnswerBody {
                    text: Some(choice.choice_text),
                    choice_id: Some(choice.choice_id.to_string()),
                    is_correct: Some(choice.is_correct),
                });
            }

            Ok(Question {
                question_id: question.question_id,
                question_type: question.question_type,
                question_text: question.question_text,
                table_question: question.table_question,
                choice: Some(choice_vec),
                key_answer: None,
            })
        } else {
            let query = sqlx::query!(
                r#"
            select
                qa.text_answer,
                qa.table_answer
            from questions
            inner join question_key_answers qa on questions.question_id = qa.question_id
            where questions.question_id = $1;
            "#,
                Uuid::from_str(question_id).map_err(|err| {
                    QuestionServiceError::UnexpectedError(anyhow!(
                        "Unable to parse question_id with error: {}",
                        err.to_string()
                    ))
                })?
            )
            .fetch_optional(&mut *conn)
            .await
            .map_err(|err| {
                QuestionServiceError::UnexpectedError(anyhow!(
                    "Got an error from database: {}",
                    err.to_string()
                ))
            })?
            .ok_or(QuestionServiceError::UnexpectedError(anyhow!(
                "Not found an answer with question id: {}",
                question_id
            )))?;

            Ok(Question {
                question_id: question.question_id,
                question_type: question.question_type,
                question_text: question.question_text,
                table_question: question.table_question,
                choice: None,
                key_answer: Some(KeyAnswerOfQuestion {
                    answer_id: None,
                    choice_answer_id: None,
                    text_answer: query.text_answer,
                    table_answer: query.table_answer,
                }),
            })
        }
    }

    pub async fn create_choice_answer_from_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        choice_answers: Vec<ChoiceAnswerBody>,
    ) -> Result<Vec<ChoiceAnswerBody>, QuestionServiceError> {
        if choice_answers.len() != 4 {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "Choice answer should be 4"
            )));
        }

        let mut true_count = 0;

        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
        insert into question_choices (question_id, choice_text, is_correct) "#,
        );

        for choice_answer in &choice_answers {
            if choice_answer.is_correct.is_none() {
                return Err(QuestionServiceError::UnexpectedError(anyhow!(
                    "Not found a is_correct bool params"
                )));
            }

            if true_count > 1 {
                return Err(QuestionServiceError::UnexpectedError(anyhow!(
                    "Found a multiple correct answer, check it again."
                )));
            }

            if let Some(is_correct) = choice_answer.is_correct {
                if is_correct {
                    true_count += 1;
                }
            }
        }

        let question_id = Uuid::parse_str(question_id).map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Unable to parse question_id, with err: {}",
                err.to_string()
            ))
        })?;

        query_builder.push_values(choice_answers, |mut separated, choice_answer| {
            separated.push_bind(question_id);
            separated.push_bind(choice_answer.text);
            if let Some(is_correct) = choice_answer.is_correct {
                separated.push_bind(is_correct);
            }
        });

        query_builder.push(r#" returning choice_id::text, choice_text, is_correct"#);

        let query = query_builder.build();

        let res = query.fetch_all(&mut *conn).await.map_err(|err| {
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got a database error, with err: {}",
                err.to_string()
            ))
        })?;

        let mut choice_answers = vec![];

        for choice_answer in res {
            choice_answers.push(ChoiceAnswerBody {
                choice_id: Some(choice_answer.get("choice_id")),
                text: choice_answer.get("choice_text"),
                is_correct: Some(choice_answer.get("is_correct")),
            })
        }

        Ok(choice_answers)
    }

    pub async fn create_key_choice_answer_from_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        choice_id: &str,
    ) -> Result<KeyAnswerOfQuestion, QuestionServiceError> {
        let query = sqlx::query!(
            r#"
      insert into question_key_answers
      (question_id, choice_answer)
      values
      ($1::text::uuid, $2::text::uuid)
      returning *;
      "#,
            question_id,
            choice_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(KeyAnswerOfQuestion {
            answer_id: None,
            choice_answer_id: query.choice_answer.map(|id| id.to_string()),
            text_answer: None,
            table_answer: None,
        })
    }

    pub async fn create_key_text_answer_from_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        text_answer: &str,
    ) -> Result<KeyAnswerOfQuestion, QuestionServiceError> {
        if text_answer.is_empty() {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "Got an empty text_answer"
            )));
        }

        let query = sqlx::query!(
            r#"
        insert into question_key_answers
        (question_id, text_answer)
        values
        ($1::text::uuid, $2)
        returning *
        "#,
            question_id,
            text_answer
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with err: {}",
                err.to_string()
            ))
        })?;

        Ok(KeyAnswerOfQuestion {
            answer_id: None,
            choice_answer_id: None,
            text_answer: query.text_answer,
            table_answer: None,
        })
    }

    pub async fn create_key_table_answer_from_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        table_answer: serde_json::Value,
    ) -> Result<KeyAnswerOfQuestion, QuestionServiceError> {
        if table_answer.is_null() {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "Got an empty table answer object"
            )));
        }

        let query = sqlx::query!(
            r#"
        insert into question_key_answers
            (question_id, table_answer)
        values
            ($1::text::uuid, $2::jsonb)
        returning *;
        "#,
            question_id,
            table_answer
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(KeyAnswerOfQuestion {
            answer_id: None,
            choice_answer_id: None,
            text_answer: None,
            table_answer: query.table_answer,
        })
    }

    pub async fn get_key_answer_of_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<KeyAnswerOfQuestion, QuestionServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from question_key_answers
        where question_id::text = $1;
        "#,
            question_id
        )
        .fetch_optional(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?
        .ok_or(QuestionServiceError::UnexpectedError(anyhow!(
            "Not found a key answer of question with a question id: {}",
            question_id
        )))?;

        Ok(KeyAnswerOfQuestion {
            answer_id: Some(query.answer_id.to_string()),
            choice_answer_id: query.choice_answer.map(|val| val.to_string()),
            text_answer: query.text_answer,
            table_answer: query.table_answer,
        })
    }

    #[allow(unused_assignments)]
    pub async fn update_key_answer_of_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        key_answer: &KeyAnswerOfQuestion,
    ) -> Result<(), QuestionServiceError> {
        if key_answer.table_answer.is_none()
            && key_answer.text_answer.is_none()
            && key_answer.choice_answer_id.is_none()
        {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "All params are not set a new value"
            )));
        }

        let retrieve_key_answer = self.get_key_answer_of_question(conn, question_id).await?;

        let mut query_builder = QueryBuilder::<Postgres>::new("update question_key_answers set");
        let mut separated = query_builder.separated(", ");

        let mut count = 0;
        let mut curr_count = 0;
        let mut count_changed = 0;

        if key_answer.table_answer.is_some() {
            count += 1;
        }

        if key_answer.text_answer.is_some() {
            count += 1;
        }

        if key_answer.choice_answer_id.is_some() {
            count += 1;
        }

        if let Some(table_answer) = &key_answer.table_answer {
            if let Some(retrieved_table_answer) = retrieve_key_answer.table_answer {
                if retrieved_table_answer != *table_answer {
                    separated.push_unseparated(" table_answer = ");
                    separated.push_bind_unseparated(table_answer);
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(", ");
                    }
                    count_changed += 1;
                }
            }
        }

        if let Some(choice_answer_id) = &key_answer.choice_answer_id {
            if let Some(retrieved_choice_answer_id) = retrieve_key_answer.choice_answer_id {
                if *choice_answer_id != retrieved_choice_answer_id {
                    separated.push_unseparated(" choice_answer = ");
                    separated.push_bind_unseparated(choice_answer_id);
                    separated.push_unseparated("::text::uuid");
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(", ");
                    }
                    count_changed += 1;
                }
            }
        }

        if let Some(text_answer) = &key_answer.text_answer {
            if let Some(retrieved_text_answer) = retrieve_key_answer.text_answer {
                if *text_answer != retrieved_text_answer {
                    separated.push_unseparated(" text_answer = ");
                    separated.push_bind_unseparated(text_answer);
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(", ");
                    }
                    count_changed += 1;
                }
            }
        }

        if count_changed == 0 {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "No new values to be set"
            )));
        }

        separated.push(" where question_id::text = ");
        separated.push_bind_unseparated(question_id);

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn get_choice_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
    ) -> Result<Vec<ChoiceAnswerBody>, QuestionServiceError> {
        let query = sqlx::query!(
            r#"
        select
        *
        from question_choices
        where question_id::text = $1;
        "#,
            question_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        if query.is_empty() {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "Not found any choice question"
            )));
        }

        let mut choices_answer_question = vec![];

        for choice_answer in query {
            choices_answer_question.push(ChoiceAnswerBody {
                choice_id: Some(choice_answer.choice_id.to_string()),
                text: Some(choice_answer.choice_text),
                is_correct: Some(choice_answer.is_correct),
            });
        }

        Ok(choices_answer_question)
    }

    #[allow(unused_assignments)]
    pub async fn update_choice_question(
        &self,
        conn: &mut PgConnection,
        question_id: &str,
        choice_id: &str,
        params: &ChoiceAnswerBody,
    ) -> Result<(), QuestionServiceError> {
        if params.is_correct.is_none() && params.text.is_none() {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "All params are not setting a new value"
            )));
        }

        let mut choice_question_map = HashMap::new();
        let retrieve_choices_question = self.get_choice_question(conn, question_id).await?;

        for choice_question in retrieve_choices_question {
            let choice_id =
                choice_question
                    .choice_id
                    .as_ref()
                    .ok_or(QuestionServiceError::UnexpectedError(anyhow!(
                        "Not found a choice_id"
                    )))?;
            choice_question_map.insert(choice_id.to_string(), choice_question);
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("update question_choices set");
        let mut separated = query_builder.separated(", ");

        let mut count = 0;
        let mut curr_count = 0;
        let mut count_changed = 0;

        if params.is_correct.is_some() {
            count += 1;
        }
        if params.text.is_some() {
            count += 1;
        }

        let retrieved_choice_question =
            choice_question_map
                .get(choice_id)
                .ok_or(QuestionServiceError::UnexpectedError(anyhow!(
                    "Not found choice question"
                )))?;

        if let Some(text) = &params.text {
            if let Some(retrieved_text) = &retrieved_choice_question.text {
                if text != retrieved_text {
                    separated.push_unseparated(" choice_text = ");
                    separated.push_bind_unseparated(text);
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(",");
                    }
                    count_changed += 1;
                }
            }
        }

        if let Some(is_correct) = &params.is_correct {
            if let Some(retrieved_is_correct) = &retrieved_choice_question.is_correct {
                if is_correct != retrieved_is_correct {
                    separated.push_unseparated(" is_correct = ");
                    separated.push_bind_unseparated(is_correct);
                    if count > 1 && curr_count != count - 1 {
                        curr_count += 1;
                        separated.push_unseparated(",");
                    }
                    count_changed += 1;
                }
            }
        }

        if count_changed == 0 {
            return Err(QuestionServiceError::UnexpectedError(anyhow!(
                "No new values to bet set"
            )));
        }

        separated.push(" where question_id::text = ");
        separated.push_bind_unseparated(question_id);
        separated.push_unseparated(" and choice_id::text = ");
        separated.push_bind_unseparated(choice_id);

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            tracing::error!("{}", err.to_string());
            QuestionServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }
}

impl Default for QuestionService {
    fn default() -> Self {
        Self::new()
    }
}
