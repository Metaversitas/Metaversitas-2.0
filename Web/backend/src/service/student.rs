use crate::backend::AppState;
use crate::helpers::errors::student::StudentServiceError;
use crate::model::student::Student;
use crate::r#const::PgTransaction;
use anyhow::anyhow;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct StudentService {
    app_state: Arc<AppState>,
}

impl StudentService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn delete_student_by_id(
        &self,
        transaction: &mut PgTransaction,
        student_id: &str,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
        delete from students
        where student_id = $1;
        "#,
            Uuid::from_str(student_id).map_err(|err| {
                anyhow!(
                    "Unable to parse student_id into uuid, with student_id: {}; err: {}",
                    student_id,
                    err.to_string()
                )
            })?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn insert_student_by_id(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
    ) -> Result<Student, StudentServiceError> {
        let query = sqlx::query!(
            r#"
        insert into students (user_id)
        values ($1)
        returning student_id, user_id;
        "#,
            Uuid::from_str(user_id).map_err(|err| {
                anyhow!(
                    "Unable to parse user_id into uuid, with user_id: {}; err: {}",
                    user_id,
                    err.to_string()
                )
            })?
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;
        Ok(Student {
            student_id: query.student_id.to_string(),
            user_id: query.user_id.to_string(),
        })
    }

    pub async fn get_student_by_id(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
    ) -> Result<Student, StudentServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from students
        where user_id = $1;
        "#,
            Uuid::from_str(user_id).map_err(|err| {
                anyhow!(
                    "Unable to parse user_id into uuid, with user_id: {}; err: {}",
                    user_id,
                    err.to_string()
                )
            })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?
        .ok_or(anyhow!("Student isn't exists!"))?;

        Ok(Student {
            student_id: query.student_id.to_string(),
            user_id: query.user_id.to_string(),
        })
    }
}
