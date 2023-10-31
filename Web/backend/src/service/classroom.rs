use crate::backend::AppState;
use crate::helpers::errors::classroom::ClassroomServiceError;
use crate::model::classroom::{Action, ActionType, Classroom, CreateClassroomParams, StudentClassroom, TeacherClassroom, UpdateClassroomParams};
use crate::model::user::{UserRole, UserUniversityRole};
use crate::r#const::PgTransaction;
use crate::service::subject::SubjectService;
use anyhow::anyhow;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct ClassroomService {
    app_state: Arc<AppState>,
    subject_service: Arc<SubjectService>,
}

// const DEFAULT_CACHE_TIME_EXIST: time::Duration = time::Duration::hours(1);

impl ClassroomService {
    pub fn new(app_state: Arc<AppState>, subject_service: Arc<SubjectService>) -> Self {
        Self {
            app_state,
            subject_service,
        }
    }

    pub async fn is_student_schedule_conflict(
        &self,
        transaction: &mut PgTransaction,
        student_id: &str,
        class_id: &str,
    ) -> Result<bool, ClassroomServiceError> {
        let query = sqlx::query!(r#"
        SELECT COUNT(*) as "count!"
        FROM class_schedule AS new_class
        JOIN student_schedule AS existing_student ON
            (
                (new_class.start_time BETWEEN existing_student.start_time AND existing_student.end_time) OR
                (new_class.end_time BETWEEN existing_student.start_time AND existing_student.end_time) OR
                (existing_student.start_time BETWEEN new_class.start_time AND new_class.end_time) OR
                (existing_student.end_time BETWEEN new_class.start_time AND new_class.end_time)
            )
        WHERE existing_student.student_id = $1 AND new_Class.class_id = $2;
        "#, Uuid::from_str(student_id).map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse student_id with error: {}", err.to_string()))
            })?,
            Uuid::from_str(class_id).map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_id with error: {}", err.to_string()))
            })?).fetch_one(&mut **transaction).await.map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string()))
            })?;

        if query.count > 0 {
            return Ok(false);
        }
        Ok(true)
    }

    pub async fn is_seat_classroom_available(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<bool, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
        select
            classes.capacity,
            count(class_students.class_id) as "count!"
        from classes
        left join class_students on classes.class_id = class_students.class_id
        where classes.class_id = $1
        group by classes.capacity;
        "#,
            Uuid::from_str(class_id).map_err(|err| anyhow!(
                "Unable to parse class_id with error: {}",
                err.to_string()
            ))?
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?;

        if query.count > query.capacity.into() {
            return Ok(false);
        }

        Ok(true)
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

    pub async fn delete_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        // Check if exists
        let _ = self
            .get_classroom_by_id(&mut *transaction, class_id)
            .await?;

        sqlx::query!(
            r#"
        delete from classes
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

    pub async fn create_classroom(
        &self,
        transaction: &mut PgTransaction,
        user_role: &UserRole,
        university_role: &UserUniversityRole,
        params: &CreateClassroomParams,
    ) -> Result<String, ClassroomServiceError> {
        //TODO: Admin access or support access.
        if matches!(user_role, UserRole::Administrator) || matches!(user_role, UserRole::Staff) {
            todo!()
        }

        if matches!(university_role, UserUniversityRole::Mahasiswa) {
            return Err(ClassroomServiceError::UnauthorizedStudent);
        }

        let query = sqlx::query!(
            r#"insert into classes (is_active, name, description) values ($1, $2, $3) returning class_id::text as "class_id!""#,
            true,
            params.name,
            params.description
        )
            .fetch_one(&mut **transaction)
            .await
            .map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database, {}", err.to_string()))
            })?;

        let class_id = query.class_id;

        if params.students.is_some() || params.teachers.is_some() {
            if let Some(students) = &params.students {
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

            if let Some(teachers) = &params.teachers {
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
                .await
                .map_err(|_| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Not found a subject"))
                })?;
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
            classes.name,
            classes.description,
            subjects.name as subject_name,
            subjects.subject_id,
            class_schedule.start_time,
            class_schedule.end_time
        from classes
            inner join class_subjects on classes.class_id = class_subjects.class_id
            inner join subjects on class_subjects.subject_id = subjects.subject_id
            inner join class_schedule on classes.class_id = class_schedule.class_id
        where classes.class_id::text = $1;
       "#,
            class_id,
            // Uuid::from_str(class_id).map_err(|err| {
            //     tracing::error!("Unable to parse class_id with error: {}", err.to_string());
            //     anyhow!("Unable to parse class_id with error: {}", err.to_string())
            // })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string()))
        })?
            .ok_or(ClassroomServiceError::UnexpectedError(anyhow!("Not found a classroom with class_id: {}", class_id)))?;

        let classroom = Classroom {
            is_active: query.is_active,
            name: query.name,
            description: query.description,
            class_id: query.class_id.to_string(),
            subject_id: query.subject_id.to_string(),
            subject_name: query.subject_name,
            start_time: query.start_time.to_rfc3339(),
            end_time: query.end_time.map(|time| time.to_rfc3339()),
        };
        Ok(classroom)
    }

    async fn get_student_classroom(
        &self,
        user_id: &str,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let query = sqlx::query!("
            select
                classes.is_active,
                classes.class_id,
                classes.name,
                classes.description,
                class_subjects.subject_id,
                subjects.name as subject_name,
                class_schedule.start_time,
                class_schedule.end_time
            from students
                     inner join class_students on students.student_id = class_students.student_id
                     inner join classes on class_students.class_id = classes.class_id and classes.is_active = true
                     inner join class_subjects on classes.class_id = class_subjects.class_id
                     inner join subjects on class_subjects.subject_id = subjects.subject_id
                     inner join class_schedule on classes.class_id = class_schedule.class_id
            where user_id::text = $1;
            ", &user_id)
            .fetch_all(&self.app_state.database)
            .await
            .map_err(|err| { ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string())) })?;

        let mut list_classroom: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let classroom = Classroom {
                is_active: tmp_classroom.is_active,
                name: tmp_classroom.name,
                description: tmp_classroom.description,
                class_id: tmp_classroom.class_id.to_string(),
                subject_id: tmp_classroom.subject_id.to_string(),
                subject_name: tmp_classroom.subject_name.to_string(),
                start_time: tmp_classroom.start_time.to_rfc3339(),
                end_time: tmp_classroom.end_time.map(|time| time.to_rfc3339()),
            };
            list_classroom.push(classroom)
        }
        Ok(list_classroom)
    }

    async fn get_lecturer_classroom(
        &self,
        user_id: &str,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let query = sqlx::query!(r#"
        select
            subjects.name as subject_name,
            classes.class_id, subjects.subject_id, is_active, classes.name, description,
            cs.start_time, cs.end_time
        from teachers
                 inner join class_teachers on teachers.teacher_id = class_teachers.teacher_id
                 inner join classes on class_teachers.class_id = classes.class_id and classes.is_active = true
                 inner join class_subjects on classes.class_id = class_subjects.class_id
                 inner join subjects on class_subjects.subject_id = subjects.subject_id
                 inner join class_schedule cs on classes.class_id = cs.class_id
        where user_id::text = $1;
            "#, &user_id)
            .fetch_all(&self.app_state.database)
            .await
            .map_err(|err| ClassroomServiceError::UnexpectedError(anyhow!("Got an error from database: {}", err.to_string())))?;

        let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let classroom = Classroom {
                is_active: tmp_classroom.is_active,
                name: tmp_classroom.name,
                description: tmp_classroom.description,
                class_id: tmp_classroom.class_id.to_string(),
                subject_id: tmp_classroom.subject_id.to_string(),
                subject_name: tmp_classroom.subject_name.to_string(),
                start_time: tmp_classroom.start_time.to_rfc3339(),
                end_time: tmp_classroom.end_time.map(|time| time.to_rfc3339()),
            };
            classrooms.push(classroom);
        }
        Ok(classrooms)
    }
}
