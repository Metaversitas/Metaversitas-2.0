use crate::helpers::errors::exam::ExamServiceError;
use crate::model::exam::{CreateExamParams, Exam, ExamType, ExamWithQuestion, UpdateExamParams};
use anyhow::anyhow;
use sqlx::{Execute, PgConnection, Postgres, QueryBuilder};

pub struct ExamService;

impl ExamService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_available_exams(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Exam>, ExamServiceError> {
        let query = sqlx::query!(
            r#"
        select
            type as "type!: ExamType",
            name,
            exam_id,
            description,
            created_by,
            updated_at
        from exams
        limit 10;
        "#
        )
        .fetch_all(&mut *conn)
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
                r#type: exam.r#type,
                exam_name: exam.name,
                description: exam.description,
                created_by: exam.created_by.to_string(),
                created_at: None,
                updated_at: Some(exam.updated_at),
            });
        }

        Ok(exams)
    }

    pub async fn get_exam_by_id(
        &self,
        conn: &mut PgConnection,
        exam_id: &str,
    ) -> Result<Exam, ExamServiceError> {
        let query = sqlx::query!(
            r#"
        select
            name,
            description,
            type as "type!: ExamType",
            exam_id,
            created_by,
            updated_at
        from exams
        where exam_id::text = $1;
        "#,
            exam_id
        )
        .fetch_optional(&mut *conn)
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
            r#type: ExamType::Default,
            exam_name: query.name,
            description: query.description,
            created_by: query.created_by.to_string(),
            created_at: None,
            updated_at: Some(query.updated_at),
        })
    }

    pub async fn create_exam(
        &self,
        conn: &mut PgConnection,
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
        separated.push_bind(&params.exam_name);
        if params.description.is_some() {
            separated.push_bind(&params.description);
        }
        separated.push_bind(user_id);
        separated.push_unseparated(")");

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn delete_exam_by_id(
        &self,
        conn: &mut PgConnection,
        exam_id: &str,
    ) -> Result<(), ExamServiceError> {
        sqlx::query!(
            r#"
        delete from exams
        where exam_id::text = $1;
        "#,
            exam_id
        )
        .execute(&mut *conn)
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
        conn: &mut PgConnection,
        exam_id: &str,
        params: &UpdateExamParams,
    ) -> Result<(), ExamServiceError> {
        if params.description.is_none() && params.exam_name.is_none() {
            return Err(ExamServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("update exams set ");
        let mut separated = query_builder.separated(", ");

        let mut count = 0;
        let mut curr_count = 0;
        let mut count_changed = 0;

        if params.exam_name.is_some() {
            count += 1;
        }
        if params.description.is_some() {
            count += 1;
        }

        if let Some(name) = &params.exam_name {
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

        query.execute(&mut *conn).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Got an error from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn insert_class_on_exam_by_id(
        &self,
        conn: &mut PgConnection,
        exam_id: &str,
        class_id: &str,
        meeting_id: Option<&str>,
    ) -> Result<(), ExamServiceError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("insert into exam_classes (exam_id, class_id");

        if meeting_id.is_some() {
            query_builder.push(", meeting_id");
        }
        query_builder.push(")");
        query_builder.push(" values (");

        query_builder.push_bind(exam_id);
        query_builder.push("::uuid");
        query_builder.push(", ");
        query_builder.push_bind(class_id);
        query_builder.push("::uuid");
        if meeting_id.is_some() {
            query_builder.push(", ");
        }

        if let Some(meeting_id) = meeting_id {
            query_builder.push_bind(meeting_id);
            query_builder.push("::uuid");
        }
        query_builder.push(")");

        let query = query_builder.build();
        query.execute(&mut *conn)
        .await
        .map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!("Unable to add exam_id={}; class_id={}; meeting_id={} into databases, with an error: {}", exam_id, class_id, meeting_id.unwrap_or("NULL"),err.to_string()))
        })?;

        Ok(())
    }

    pub async fn delete_exam_on_class_by_id(
        &self,
        conn: &mut PgConnection,
        exam_id: &str,
        class_id: &str,
        meeting_id: Option<&str>,
    ) -> Result<(), ExamServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("delete from exam_classes where ");
        query_builder.push("exam_id::uuid = ");
        query_builder.push_bind(exam_id);
        query_builder.push(" and ");
        query_builder.push("class_id::uuid = ");
        query_builder.push_bind(class_id);

        if let Some(meeting_id) = meeting_id {
            query_builder.push(" and ");
            query_builder.push_bind(meeting_id);
        }

        let query = query_builder.build();
        dbg!(&query.sql());

        query.execute(&mut *conn).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Unable to delete exam={} on class={}, with an error from database: {}",
                exam_id,
                class_id,
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn delete_exam_on_class(
        &self,
        conn: &mut PgConnection,
        class_id: &str,
        meeting_id: Option<&str>,
    ) -> Result<(), ExamServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("delete from exam_classes where ");
        query_builder.push("class_id::text = ");
        query_builder.push_bind(class_id);

        if let Some(meeting_id) = meeting_id {
            query_builder.push(" and ");
            query_builder.push_bind(meeting_id);
        }

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Unable to delete exam on class={}, with an error from database: {}",
                class_id,
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn update_exam_on_class_by_id(
        &self,
        conn: &mut PgConnection,
        exam_id: &str,
        class_id: &str,
        meeting_id: Option<&str>,
    ) -> Result<(), ExamServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("update exam_classes set exam_id = ");
        query_builder.push_bind(exam_id);
        query_builder.push("::uuid");
        query_builder.push(" where class_id::text = ");
        query_builder.push_bind(class_id);

        if let Some(meeting_id) = meeting_id {
            query_builder.push(" and meeting_id::text = ");
            query_builder.push_bind(meeting_id);
        }

        let query = query_builder.build();

        query.execute(&mut *conn).await.map_err(|err| {
            ExamServiceError::UnexpectedError(anyhow!(
                "Unable to update exam on class={}, with an error from database: {}",
                class_id,
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn get_exam_by_id_with_questions(
        &self,
        _conn: &mut PgConnection,
        _exam_id: &str,
    ) -> Result<ExamWithQuestion, ExamServiceError> {
        // let query = sqlx::query!(r#"
        // select
        //     *
        // from question_exams
        // inner join questions q on question_exams.question_id = q.question_id
        // where exam_id::text = $1
        // "#, exam_id).fetch_all();
        todo!();
    }

    pub async fn get_exams_by_subject_id(
        &self,
        conn: &mut PgConnection,
        subject_id: &str,
    ) -> Result<Vec<Exam>, ExamServiceError> {
        let query = sqlx::query!(r#"
        select
            exam_subject.subject_id, exam_subject.secondary_subject_id,
            exams.exam_id::text as "exam_id!: String", exams.name as exam_name, description, created_by::text as "created_by!: String", created_at, updated_at, exams.type as "exam_type!: ExamType",
            s.name as subject_name,
            ss.name as subject_secondary_name
        from exam_subject
        inner join exams on exam_subject.exam_id = exams.exam_id
        inner join subjects s on exam_subject.subject_id = s.subject_id
        left join subject_secondary ss on exam_subject.secondary_subject_id = ss.secondary_subject_id
        where exam_subject.subject_id::text = $1
        "#, subject_id)
        .fetch_all(&mut *conn)
        .await
        .map_err(|err| {
           ExamServiceError::UnexpectedError(anyhow!("Unable to get list of exam by subject_id, with an error: {}", err.to_string()))
        })?;

        let mut exams = vec![];

        for tmp_exam in query {
            exams.push(Exam {
                exam_id: tmp_exam.exam_id,
                r#type: tmp_exam.exam_type,
                exam_name: tmp_exam.exam_name,
                description: tmp_exam.description,
                created_by: tmp_exam.created_by,
                created_at: Some(tmp_exam.created_at),
                updated_at: Some(tmp_exam.updated_at),
            })
        }

        Ok(exams)
    }
}

impl Default for ExamService {
    fn default() -> Self {
        Self::new()
    }
}
