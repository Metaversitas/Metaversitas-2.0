use crate::backend::AppState;
use crate::helpers::errors::classroom::ClassroomServiceError;
use crate::model::classroom::{
    Action, ActionType, Classroom, StudentClassroom, TeacherClassroom, UpdateClassroomParams,
};
use crate::model::user::{UserRole, UserUniversityRole};
use crate::r#const::PgTransaction;
use crate::service::subject::SubjectService;
use anyhow::anyhow;
use redis::{AsyncCommands, JsonAsyncCommands};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct ClassroomService {
    pub app_state: Arc<AppState>,
    pub subject_service: Arc<SubjectService>,
}

const DEFAULT_CACHE_TIME_EXIST: time::Duration = time::Duration::hours(1);

impl ClassroomService {
    pub fn new(app_state: Arc<AppState>, subject_service: Arc<SubjectService>) -> Self {
        Self {
            app_state,
            subject_service,
        }
    }

    pub async fn get_available_classroom(
        &self,
        user_id: &str,
        user_role: &UserRole,
        university_role: &UserUniversityRole,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        match user_role {
            UserRole::Administrator | UserRole::Staff => {
                //TODO: If there is some admin panel, this would be useful to get all available classroom and try to edit on it
                todo!()
            }
            UserRole::User => match university_role {
                UserUniversityRole::Dosen => Ok(self.get_lecturer_classroom(user_id).await?),
                UserUniversityRole::Mahasiswa => Ok(self.get_student_classroom(user_id).await?),
            },
        }
    }

    pub async fn create_classroom(
        &self,
        transaction: &mut PgTransaction,
        user_role: &UserRole,
        university_role: &UserUniversityRole,
        subject_id: &str,
        students: Option<Vec<String>>,
        teachers: Option<Vec<String>>,
    ) -> Result<String, ClassroomServiceError> {
        //TODO: Admin access or support access.
        if matches!(user_role, UserRole::Administrator) || matches!(user_role, UserRole::Staff) {
            todo!()
        }

        if matches!(university_role, UserUniversityRole::Mahasiswa) {
            return Err(ClassroomServiceError::UnauthorizedStudent);
        }

        let query = sqlx::query!(
            r#"insert into classes (subject_id, is_active) values ($1, $2) returning class_id::text as "class_id!""#,
            Uuid::from_str(subject_id).map_err(|_|
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id into uuid"))
            )?,
            true
        )
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database, {}", err.to_string()))
            })?;

        let class_id = query.class_id;

        if students.is_some() || teachers.is_some() {
            if let Some(students) = students {
                for student in students {
                    sqlx::query!(
                        "insert into class_students (class_id, student_id) values ($1, $2)",
                        Uuid::from_str(class_id.as_str()).map_err(|_| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to parse class_id into uuid"
                            ))
                        })?,
                        Uuid::from_str(student.as_str()).map_err(|_| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to parse student into uuid"
                            ))
                        })?
                    )
                    .execute(&mut **transaction)
                    .await
                    .map_err(|err| {
                        ClassroomServiceError::UnexpectedError(anyhow!(
                            "Got an error from database: {}",
                            err.to_string()
                        ))
                    })?;
                }
            }

            if let Some(teachers) = teachers {
                for teacher in teachers {
                    sqlx::query!(
                        "insert into class_teachers (class_id, teacher_id) values ($1, $2)",
                        Uuid::from_str(class_id.as_str()).map_err(|_| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to parse class_id into uuid"
                            ))
                        })?,
                        Uuid::from_str(teacher.as_str()).map_err(|_| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to parse teacher_id into uuid"
                            ))
                        })?
                    )
                    .execute(&mut **transaction)
                    .await
                    .map_err(|err| {
                        ClassroomServiceError::UnexpectedError(anyhow!(
                            "Got an error from database: {}",
                            err.to_string()
                        ))
                    })?;
                }
            }
        }

        Ok(class_id)
    }

    pub async fn update_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: String,
        params: UpdateClassroomParams,
    ) -> Result<(), ClassroomServiceError> {
        //Check if classroom exists
        let _ = self
            .get_classroom_by_id(&mut *transaction, class_id.as_str())
            .await?;

        if let (Some(subject_id), Some(subject_name)) = (&params.subject_id, &params.subject_name) {
            self.subject_service
                .update_subject_by_id(
                    &mut *transaction,
                    subject_id.as_str(),
                    subject_name.as_str(),
                )
                .await?;
        }

        if let Some(students) = params.students {
            match students {
                ActionType::All(list_students) => {
                    self.delete_student_classroom(&mut *transaction, class_id.as_str())
                        .await?;

                    for student in list_students {
                        match student {
                            Action::Add(student_id) => {
                                self.insert_student_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    student_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Not an expected students type"
                                )));
                            }
                        }
                    }
                }
                ActionType::Single(list_students) => {
                    for student_action in list_students {
                        match student_action {
                            Action::Add(student_id) => {
                                self.insert_student_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    student_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(student_id) => {
                                self.delete_student_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    student_id.as_str(),
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(teachers) = params.teachers {
            match teachers {
                ActionType::All(teachers) => {
                    self.delete_teacher_classroom(&mut *transaction, class_id.as_str())
                        .await?;

                    for teacher in teachers {
                        match teacher {
                            Action::Add(teacher_id) => {
                                self.insert_teacher_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    teacher_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Not an expected teachers params type"
                                )));
                            }
                        }
                    }
                }
                ActionType::Single(teachers) => {
                    for teacher in teachers {
                        match teacher {
                            Action::Add(teacher_id) => {
                                self.insert_teacher_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    teacher_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(teacher_id) => {
                                self.delete_teacher_classroom_by_id(
                                    &mut *transaction,
                                    class_id.as_str(),
                                    teacher_id.as_str(),
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn delete_teacher_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!(
            r#"
        delete from class_teachers
        where class_id = $1;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn delete_teacher_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        teacher_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!(
            r#"
        delete from class_teachers
        where class_id = $1 and teacher_id = $2;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?,
            Uuid::from_str(teacher_id).map_err(|err| anyhow!(
                "Unable to parse teacher_id with error: {}",
                err.to_string()
            ))?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn insert_teacher_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        teacher_id: &str,
    ) -> Result<TeacherClassroom, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
        insert into class_teachers (class_id, teacher_id)
        values ($1, $2)
        returning class_id, teacher_id;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?,
            Uuid::from_str(teacher_id).map_err(|err| anyhow!(
                "Unable to parse teacher_id with error: {}",
                err.to_string()
            ))?
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(TeacherClassroom {
            class_id: query.class_id.to_string(),
            teacher_id: query.teacher_id.to_string(),
        })
    }

    pub async fn delete_student_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!(
            r#"
        delete from class_students
        where class_id = $1;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn insert_student_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        student_id: &str,
    ) -> Result<StudentClassroom, ClassroomServiceError> {
        sqlx::query!(
            r#"
        insert into class_students (class_id, student_id)
        values ($1, $2);
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?,
            Uuid::from_str(student_id).map_err(|err| anyhow!(
                "Unable to parse student_id with error: {}",
                err.to_string()
            ))?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(StudentClassroom {
            class_id: class_id.to_string(),
            student_id: student_id.to_string(),
        })
    }

    pub async fn delete_student_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        student_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!(
            r#"
        delete from class_students
        where class_id = $1 and student_id = $2;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?,
            Uuid::from_str(student_id).map_err(|err| anyhow!(
                "Unable to parse student_id with error: {}",
                err.to_string()
            ))?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn get_teacher_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        teacher_id: &str,
    ) -> Result<TeacherClassroom, ClassroomServiceError> {
        let query = sqlx::query!(
            "
            select
                class_id,
                teacher_id
            from class_teachers
            where class_id = $1 and teacher_id = $2;
        ",
            Uuid::from_str(class_id).map_err(|err| {
                anyhow!(
                    "Unable to parse class_id into uuid, with class_id: {}; err: {}",
                    class_id,
                    err.to_string()
                )
            })?,
            Uuid::from_str(teacher_id).map_err(|err| {
                anyhow!(
                    "Unable to parse teacher_id into uuid, with teacher_id: {}; err: {}",
                    teacher_id,
                    err.to_string()
                )
            })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?
        .ok_or(anyhow!(
            "Not found a classroom with teacher_id: {} and class_id: {}",
            teacher_id,
            class_id
        ))?;

        Ok(TeacherClassroom {
            class_id: query.class_id.to_string(),
            teacher_id: query.teacher_id.to_string(),
        })
    }

    pub async fn get_student_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        student_id: &str,
    ) -> Result<StudentClassroom, ClassroomServiceError> {
        let query = sqlx::query!(
            "
            select
                class_id,
                student_id
            from class_students
            where class_id = $1 and student_id = $2;
        ",
            Uuid::from_str(class_id).map_err(|err| {
                anyhow!(
                    "Unable to parse class_id into uuid, with class_id: {}; err: {}",
                    class_id,
                    err.to_string()
                )
            })?,
            Uuid::from_str(student_id).map_err(|err| {
                anyhow!(
                    "Unable to parse student_id into uuid, with student_id: {}; err: {}",
                    student_id,
                    err.to_string()
                )
            })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?
        .ok_or(anyhow!(
            "Not found a classroom with student_id: {} and class_id: {}",
            student_id,
            class_id
        ))?;

        let student_classroom = StudentClassroom {
            class_id: query.class_id.to_string(),
            student_id: query.student_id.to_string(),
        };
        Ok(student_classroom)
    }

    pub async fn get_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<Classroom, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
       select
        classes.is_active,
        classes.class_id,
        classes.subject_id,
        subjects.name as subject_name
       from classes
       inner join subjects on classes.subject_id = subjects.subject_id
       where class_id = $1
       "#,
            Uuid::from_str(class_id).map_err(|err| {
                tracing::error!("Unable to parse class_id with error: {}", err.to_string());
                anyhow!("Unable to parse class_id with error: {}", err.to_string())
            })?
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Got an error from database: {}", err.to_string());
            anyhow!("Got an error from database: {}", err.to_string())
        })?;

        let classroom = Classroom {
            is_active: query.is_active,
            class_id: query.class_id.to_string(),
            subject_id: query.subject_id.to_string(),
            subject_name: query.subject_name,
        };
        Ok(classroom)
    }

    async fn get_student_classroom(
        &self,
        user_id: &str,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let classroom_key = format!("classroom:{user_id}");
        let mut redis_conn = self
            .app_state
            .redis
            .get_async_connection()
            .await
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!(
                    "Unable to get connection from redis"
                ))
            })?;
        let is_exists = redis_conn
            .exists::<String, usize>(classroom_key.to_owned())
            .await
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to set exists key in redis"))
            })?;

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
            .map_err(|err| { ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string())) })?;

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

            redis_conn
                .json_set::<String, &str, Vec<Classroom>, ()>(
                    classroom_key.to_owned(),
                    "$",
                    &list_classroom,
                )
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to json set on redis"))
                })?;
            redis_conn
                .expire_at::<String, ()>(classroom_key.to_owned(), timestamp_expire as usize)
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to set an expire on redis"
                    ))
                })?;

            Ok(list_classroom)
        } else if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, String>(classroom_key.to_owned(), "$")
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to get a json data from redis"
                    ))
                })?;
            let classrooms = &serde_json::from_str::<Vec<Vec<Classroom>>>(
                query_from_redis.as_str(),
            )
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!("Got an unvalid data from redis"))
            })?[0];
            Ok(classrooms.to_vec())
        } else {
            Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "not a wanted response from redis"
            )))
        }
    }

    async fn get_lecturer_classroom(
        &self,
        user_id: &str,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let classroom_key = format!("classroom:{user_id}");
        let mut redis_conn = self
            .app_state
            .redis
            .get_async_connection()
            .await
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to get a redis connection"))
            })?;
        let is_exists = redis_conn
            .exists::<String, usize>(classroom_key.to_owned())
            .await
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!(
                    "Unable to get exists status from redis"
                ))
            })?;

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
            .map_err(|err| ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string())))?;

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

            redis_conn
                .json_set::<String, &str, Vec<Classroom>, ()>(
                    classroom_key.to_owned(),
                    "$",
                    &classrooms,
                )
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to do json_set on redis"
                    ))
                })?;
            redis_conn
                .expire_at::<String, ()>(classroom_key.to_owned(), timestamp_expire as usize)
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to do expire_at on redis"
                    ))
                })?;
            Ok(classrooms)
        } else if is_exists == 1 {
            let query_from_redis = redis_conn
                .json_get::<String, &str, String>(classroom_key.to_owned(), "$")
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to do json_get on redis"
                    ))
                })?;
            let classrooms = &serde_json::from_str::<Vec<Vec<Classroom>>>(
                query_from_redis.as_str(),
            )
            .map_err(|_| {
                ClassroomServiceError::UnexpectedError(anyhow!("Got an unexpected data from redis"))
            })?[0];
            Ok(classrooms.to_vec())
        } else {
            Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "not a wanted response from redis"
            )))
        }
    }
}
