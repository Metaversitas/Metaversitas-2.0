use crate::helpers::errors::exam::ExamServiceError;
use crate::model::exam::{CreateExamParams, Exam, UpdateExamParams};
use crate::r#const::PgTransaction;
use anyhow::anyhow;
use sqlx::{Postgres, QueryBuilder};

pub struct ExamService;

impl ExamService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_available_exams(
        &self,
        transaction: &mut PgTransaction,
    ) -> Result<Vec<Exam>, ExamServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from exams
        limit 10;
        "#
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got a database error, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut exams = vec![];
        for exam in query {
            exams.push(Exam {
                exam_id: exam.exam_id.to_string(),
                name: exam.name,
                description: exam.description,
                created_by: exam.created_by.to_string(),
                created_at: None,
                updated_at: None,
            });
        }

        Ok(exams)
    }

    pub async fn get_exam_by_id(
        &self,
        transaction: &mut PgTransaction,
        exam_id: &str,
    ) -> Result<Exam, ExamServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from exams
        where exam_id::text = $1;
        "#,
            exam_id
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?
        .ok_or(ExamServiceError::UnexpectedError(anyhow!(
            "Not found an exam with given id: {}",
            exam_id
        )))?;
        Ok(Exam {
            exam_id: query.exam_id.to_string(),
            name: query.name,
            description: query.description,
            created_by: query.created_by.to_string(),
            created_at: None,
            updated_at: None,
        })
    }

    pub async fn create_exam(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &CreateExamParams,
    ) -> Result<(), ExamServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("insert into exams (name, ");
        let mut separated = query_builder.separated(", ");

        if params.description.is_some() {
            separated.push("description");
        }
        separated.push("created_by");
        separated.push_unseparated(")");
        separated.push_unseparated(" values (");
        separated.push_bind(&params.name);
        if params.description.is_some() {
            separated.push_bind(&params.description);
        }
        separated.push_bind(user_id);
        separated.push_unseparated(")");

        let query = query_builder.build();

        query.execute(&mut **transaction).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn delete_exam_by_id(
        &self,
        transaction: &mut PgTransaction,
        exam_id: &str,
    ) -> Result<(), ExamServiceError> {
        sqlx::query!(
            r#"
        delete from exams
        where exam_id::text = $1;
        "#,
            exam_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got a database error, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }

    #[allow(unused_assignments)]
    pub async fn update_exam_by_id(
        &self,
        transaction: &mut PgTransaction,
        exam_id: &str,
        params: &UpdateExamParams,
    ) -> Result<(), ExamServiceError> {
        if params.description.is_none() && params.name.is_none() {
            return Err(ExamServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("update exams set ");
        let mut separated = query_builder.separated(", ");

        let mut count = 0;
        let mut curr_count = 0;
        let mut count_changed = 0;

        if params.name.is_some() {
            count += 1;
        }
        if params.description.is_some() {
            count += 1;
        }

        if let Some(name) = &params.name {
            separated.push_unseparated("name = ");
            separated.push_bind_unseparated(name);

            if count > 1 && curr_count != count - 1 {
                curr_count += 1;
                separated.push_unseparated(", ");
            }

            count_changed += 1;
        }

        if let Some(description) = &params.description {
            separated.push_unseparated("description = ");
            separated.push_bind_unseparated(description);

            if count > 1 && curr_count != count - 1 {
                curr_count += 1;
                separated.push_unseparated(", ");
            }
            count_changed += 1;
        }

        if count_changed == 0 {
            return Err(ExamServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        separated.push(" where exam_id::text = ");
        separated.push_bind_unseparated(exam_id);

        let query = query_builder.build();

        query.execute(&mut **transaction).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }
}

impl Default for ExamService {
    fn default() -> Self {
        Self::new()
    }
}