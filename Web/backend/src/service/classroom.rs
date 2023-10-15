use crate::backend::AppState;
use crate::model::classroom::{Classroom, SubjectClassroom};
use crate::model::user::{UserRole, UserUniversityRole};
use anyhow::anyhow;
use garde::rules::AsStr;
use redis::{AsyncCommands, JsonAsyncCommands};
use sqlx::{Postgres, Transaction};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;
use crate::r#const::PgTransaction;

pub struct ClassroomService {
    pub app_state: Arc<AppState>,
}

const DEFAULT_CACHE_TIME_EXIST: time::Duration = time::Duration::hours(1);

impl ClassroomService {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    pub async fn get_available_classroom(
        &self,
        user_id: &str,
        user_role: &UserRole,
        university_role: &UserUniversityRole,
    ) -> Result<Vec<Classroom>, anyhow::Error> {
        match user_role {
            UserRole::Administrator | UserRole::Staff => {
                //TODO: If there is some admin panel, this would be useful to get all available classroom and try to edit on it
                todo!()
            }
            UserRole::User => match university_role {
                UserUniversityRole::Dosen => {
                    Ok(self.get_lecturer_classroom(user_id).await.unwrap())
                }
                UserUniversityRole::Mahasiswa => {
                    Ok(self.get_student_classroom(user_id).await.unwrap())
                }
            },
        }
    }

    pub async fn create_classroom(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        user_role: &UserRole,
        university_role: &UserUniversityRole,
        subject_id: &str,
    ) -> Result<String, anyhow::Error> {
        //TODO: Admin access or support access.
        if matches!(user_role, UserRole::Administrator) || matches!(user_role, UserRole::Staff) {
            todo!()
        }

        if matches!(university_role, UserUniversityRole::Mahasiswa) {
            return Err(anyhow!("Unable to create a classroom with student access."));
        }

        let query = sqlx::query!(
            r#"insert into classes (subject_id, is_active) values ($1, $2) returning class_id::text as "class_id!""#,
            Uuid::from_str(subject_id).map_err(|_| anyhow!("Unable to parse subject_id into Uuid"))?,
            true
        )
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| {
                anyhow!(err.to_string())
            })?;
        Ok(query.class_id)
    }

    pub async fn get_subjects_by_name(
        &self,
        transaction: &mut PgTransaction,
        name: &str,
    ) -> Result<Vec<SubjectClassroom>, anyhow::Error> {
        let query = sqlx::query!(
            "
        select subject_id, name
        from subjects
        where name ilike $1",
            format!("{name}%")
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| anyhow!(err.to_string()))?;
        let mut classroom_subjects = Vec::with_capacity(query.len());

        for subject in query {
            let subject = SubjectClassroom {
                subject_id: subject.subject_id.to_string(),
                subject_name: subject.name.to_string(),
            };

            classroom_subjects.push(subject);
        }
        Ok(classroom_subjects)
    }

    pub async fn get_subject_by_name(
        &self,
        transaction: &mut PgTransaction,
        name: &str,
    ) -> Result<SubjectClassroom, anyhow::Error> {
        let query = sqlx::query!(
            "
        select subject_id, name
        from subjects
        where name ilike $1
        limit 1",
            format!("{name}%")
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| anyhow!("error from database: {}", err.to_string()))?
        .ok_or(anyhow!("not found any subject"))?;
        let subject = SubjectClassroom {
            subject_id: query.subject_id.to_string(),
            subject_name: query.name.to_string(),
        };
        Ok(subject)
    }

    pub async fn get_subject_by_id(&self, transaction: &mut PgTransaction, subject_id: &str) -> Result<SubjectClassroom, anyhow::Error> {
        let query = sqlx::query!("
        select subject_id, name
        from subjects
        where subject_id::text = $1
        ", subject_id)
            .fetch_optional(&mut **transaction)
            .await
            .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?
            .ok_or(anyhow!("Not found any subject classroom"))?;
        let subject = SubjectClassroom {
            subject_id: query.subject_id.to_string(),
            subject_name: query.name.to_string(),
        };
        Ok(subject)
    }

    async fn get_student_classroom(&self, user_id: &str) -> Result<Vec<Classroom>, anyhow::Error> {
        let classroom_key = format!("classroom:{user_id}");
        let mut redis_conn = self.app_state.redis.get_async_connection().await.unwrap();
        let is_exists = redis_conn
            .exists::<String, usize>(classroom_key.to_owned())
            .await
            .unwrap();

        if is_exists == 0 {
            let query = sqlx::query!("
            select
                classes.is_active,
                classes.class_id,
                classes.subject_id,
                subjects.name as subject_name
            from students
            inner join class_students on students.student_id = class_students.student_id
            inner join classes on class_students.class_id = classes.class_id and classes.is_active = true
            inner join subjects on classes.subject_id = subjects.subject_id
            where user_id::text = $1
            ", &user_id)
                .fetch_all(&self.app_state.database)
                .await
                .unwrap();

            let mut list_classroom: Vec<Classroom> = Vec::with_capacity(query.len());
            for tmp_classroom in query {
                let classroom = Classroom {
                    is_active: tmp_classroom.is_active,
                    class_id: tmp_classroom.class_id.to_string(),
                    subject_id: tmp_classroom.subject_id.to_string(),
                    subject_name: tmp_classroom.subject_name.to_string(),
                };
                list_classroom.push(classroom)
            }

            let timestamp_expire =
                (time::OffsetDateTime::now_utc() + DEFAULT_CACHE_TIME_EXIST).unix_timestamp();

            let _ = redis_conn
                .json_set::<String, &str, Vec<Classroom>, ()>(
                    classroom_key.to_owned(),
                    "$",
                    &list_classroom,
                )
                .await
                .unwrap();
            let _ = redis_conn
                .expire_at::<String, ()>(classroom_key.to_owned(), timestamp_expire as usize)
                .await
                .unwrap();

            Ok(list_classroom)
        } else if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, String>(classroom_key.to_owned(), "$")
                .await
                .unwrap();
            let classrooms =
                &serde_json::from_str::<Vec<Vec<Classroom>>>(query_from_redis.as_str()).unwrap()[0];
            Ok(classrooms.to_vec())
        } else {
            Err(anyhow!("not a wanted response from redis"))
        }
    }

    async fn get_lecturer_classroom(&self, user_id: &str) -> Result<Vec<Classroom>, anyhow::Error> {
        let classroom_key = format!("classroom:{user_id}");
        let mut redis_conn = self.app_state.redis.get_async_connection().await.unwrap();
        let is_exists = redis_conn
            .exists::<String, usize>(classroom_key.to_owned())
            .await
            .unwrap();

        if is_exists == 0 {
            let query = sqlx::query!(r#"
            select
                subjects.name as subject_name,
                classes.class_id, subjects.subject_id, is_active
            from teachers
            inner join class_teachers on teachers.teacher_id = class_teachers.teacher_id
            inner join classes on class_teachers.class_id = classes.class_id and classes.is_active = true
            inner join subjects on classes.subject_id = subjects.subject_id
            where user_id::text = $1"#, &user_id)
                .fetch_all(&self.app_state.database)
                .await
                .unwrap();

            let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
            for tmp_classroom in query {
                let classroom = Classroom {
                    is_active: tmp_classroom.is_active,
                    class_id: tmp_classroom.class_id.to_string(),
                    subject_id: tmp_classroom.subject_id.to_string(),
                    subject_name: tmp_classroom.subject_name.to_string(),
                };
                classrooms.push(classroom);
            }
            let timestamp_expire =
                (time::OffsetDateTime::now_utc() + DEFAULT_CACHE_TIME_EXIST).unix_timestamp();

            let _ = redis_conn
                .json_set::<String, &str, Vec<Classroom>, ()>(
                    classroom_key.to_owned(),
                    "$",
                    &classrooms,
                )
                .await
                .unwrap();
            let _ = redis_conn
                .expire_at::<String, ()>(classroom_key.to_owned(), timestamp_expire as usize)
                .await
                .unwrap();
            Ok(classrooms)
        } else if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, String>(classroom_key.to_owned(), "$")
                .await
                .unwrap();
            let classrooms =
                &serde_json::from_str::<Vec<Vec<Classroom>>>(query_from_redis.as_str()).unwrap()[0];
            Ok(classrooms.to_vec())
        } else {
            Err(anyhow!("not a wanted response from redis"))
        }
    }
}
