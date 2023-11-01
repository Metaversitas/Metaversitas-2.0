use crate::helpers::errors::teacher::TeacherServiceError;
use crate::model::teacher::Teacher;
use crate::r#const::PgTransaction;
use anyhow::anyhow;
use std::str::FromStr;
use uuid::Uuid;

pub struct TeacherService;

impl TeacherService {
    pub fn new() -> Self {
        Self
    }

    pub async fn delete_teacher_by_id(
        &self,
        transaction: &mut PgTransaction,
        teacher_id: &str,
    ) -> Result<(), TeacherServiceError> {
        sqlx::query!(
            r#"
        delete from teachers
        where teacher_id = $1;
        "#,
            Uuid::from_str(teacher_id).map_err(|err| {
                anyhow!(
                    "Unable to parse teacher_id into uuid, with teacher_id: {}; err: {}",
                    teacher_id,
                    err.to_string()
                )
            })?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;
        Ok(())
    }

    pub async fn insert_teacher_by_id(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
    ) -> Result<Teacher, TeacherServiceError> {
        let query = sqlx::query!(
            r#"
        insert into teachers (user_id)
        values ($1)
        returning teacher_id, user_id;
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

        Ok(Teacher {
            teacher_id: query.teacher_id.to_string(),
            user_id: query.user_id.to_string(),
        })
    }

    pub async fn get_teacher_by_id(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
    ) -> Result<Teacher, TeacherServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from teachers
        where user_id = $1;
        "#,
            Uuid::from_str(user_id).map_err(|err| { TeacherServiceError::UuidParseFailed(err) })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            TeacherServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?
        .ok_or(TeacherServiceError::UnexpectedError(anyhow!(
            "Teacher isn't exists!"
        )))?;

        Ok(Teacher {
            teacher_id: query.teacher_id.to_string(),
            user_id: query.user_id.to_string(),
        })
    }
}

impl Default for TeacherService {
    fn default() -> Self {
        Self::new()
    }
}
