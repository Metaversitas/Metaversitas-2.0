use crate::helpers::errors::subject::SubjectServiceError;
use crate::model::subject::{SecondarySubject, Subject, SubjectWithSecondaryList};
use crate::r#const::PgTransaction;
use anyhow::anyhow;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

pub struct SubjectService;

impl SubjectService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        subject_id: &str,
    ) -> Result<Subject, SubjectServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from subjects
        where subject_id = $1;
        "#,
            Uuid::from_str(subject_id).map_err(|err| {
                anyhow!(
                    "Unable to parse subject_id into uuid, with subject_id: {}; err: {}",
                    subject_id,
                    err.to_string()
                )
            })?
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?
        .ok_or(anyhow!("Subject isn't exists!"))?;

        Ok(Subject {
            subject_id: query.subject_id.to_string(),
            subject_name: query.name,
        })
    }

    pub async fn get_all_subject_with_secondary(
        &self,
        transaction: &mut PgTransaction,
    ) -> Result<Vec<SubjectWithSecondaryList>, SubjectServiceError> {
        let query = sqlx::query!(
            r#"
            select
                subjects.subject_id as "subject_id!",
                subjects.name as "subject_name!",
                subject_secondary.secondary_subject_id as "subject_secondary_id?",
                subject_secondary.name as "subject_secondary_name?"
            from subjects
            left join subject_secondary on subjects.subject_id = subject_secondary.subject_id;
        "#
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            SubjectServiceError::UnexpectedError(anyhow!(
                "Unable to get all subject from database, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut subjects: Vec<SubjectWithSecondaryList> = vec![];
        let mut subject_map: HashMap<Subject, Vec<SecondarySubject>> = HashMap::new();

        for subject in query {
            let tmp_subject = Subject {
                subject_id: subject.subject_id.to_string(),
                subject_name: subject.subject_name,
            };

            if let Some(list_secondary_subject) = subject_map.get_mut(&tmp_subject) {
                if let Some(subject_secondary_id) = subject.subject_secondary_id {
                    if let Some(subject_secondary_name) = subject.subject_secondary_name {
                        list_secondary_subject.push(SecondarySubject {
                            secondary_subject_id: subject_secondary_id.to_string(),
                            secondary_subject_name: subject_secondary_name,
                            subject_id: subject.subject_id.to_string(),
                        })
                    }
                }
            } else {
                let mut list_secondary_subject = vec![];
                if let Some(subject_secondary_id) = subject.subject_secondary_id {
                    if let Some(subject_secondary_name) = subject.subject_secondary_name {
                        list_secondary_subject.push(SecondarySubject {
                            secondary_subject_id: subject_secondary_id.to_string(),
                            secondary_subject_name: subject_secondary_name,
                            subject_id: subject.subject_id.to_string(),
                        })
                    }
                }
                subject_map.insert(tmp_subject, list_secondary_subject);
            }
        }

        for (subject, secondary_list) in subject_map {
            subjects.push(SubjectWithSecondaryList {
                subject_id: subject.subject_id,
                subject_name: subject.subject_name,
                secondary_subjects: Some(secondary_list),
            })
        }

        Ok(subjects)
    }

    pub async fn delete_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        subject_id: &str,
    ) -> Result<(), SubjectServiceError> {
        sqlx::query!(
            r#"
        delete from subjects
        where subject_id = $1;
        "#,
            Uuid::from_str(subject_id).map_err(|err| {
                anyhow!(
                    "Unable to parse subject_id into uuid, with subject_id: {}; err: {}",
                    subject_id,
                    err.to_string()
                )
            })?
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| anyhow!("Got an error from database: {}", err.to_string()))?;

        Ok(())
    }

    pub async fn create_subject(
        &self,
        transaction: &mut PgTransaction,
        subject_name: &str,
    ) -> Result<Subject, SubjectServiceError> {
        //Check if it is exists
        if self
            .get_subject_by_name(&mut *transaction, subject_name)
            .await
            .is_ok()
        {
            return Err(SubjectServiceError::UnexpectedError(anyhow!(
                "Subject with name: {} already exists!",
                subject_name
            )));
        }

        // Insert
        let query = sqlx::query!(
            r#"
        insert into subjects (name)
        values ($1)
        returning subject_id::text as "subject_id!", name;
        "#,
            subject_name
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            tracing::error!("Unable to create a subject with error: {}", err.to_string());
            SubjectServiceError::UnexpectedError(anyhow!(
                "Unable to create a subject with error: {}",
                err.to_string()
            ))
        })?;

        let subject = Subject {
            subject_id: query.subject_id,
            subject_name: query.name,
        };

        Ok(subject)
    }

    pub async fn get_subjects_by_name(
        &self,
        transaction: &mut PgTransaction,
        name: &str,
    ) -> Result<Vec<Subject>, SubjectServiceError> {
        let query = sqlx::query!(
            "
        select subject_id, name
        from subjects
        where name ilike $1",
            format!("{name}%")
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| SubjectServiceError::UnexpectedError(anyhow!(err.to_string())))?;
        let mut classroom_subjects = Vec::with_capacity(query.len());

        for subject in query {
            let subject = Subject {
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
    ) -> Result<Subject, SubjectServiceError> {
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
        .map_err(|err| {
            SubjectServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?
        .ok_or(SubjectServiceError::NotFound)?;
        let subject = Subject {
            subject_id: query.subject_id.to_string(),
            subject_name: query.name.to_string(),
        };
        Ok(subject)
    }

    pub async fn update_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        subject_id: &str,
        subject_name: &str,
    ) -> Result<Subject, SubjectServiceError> {
        // Check if it is exists
        let _ = &self
            .get_subject_by_id(&mut *transaction, subject_id)
            .await?;

        let query_update = sqlx::query!(
            "
            update subjects
            set name = $1
            where subject_id::text = $2
            returning subject_id, name
            ",
            subject_name,
            subject_id
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            SubjectServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?;
        let subject = Subject {
            subject_id: query_update.subject_id.to_string(),
            subject_name: query_update.name.to_string(),
        };

        Ok(subject)
    }

    pub async fn get_secondary_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        secondary_subject_id: &str,
    ) -> Result<SecondarySubject, SubjectServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from subject_secondary
        where secondary_subject_id::text = $1
        "#,
            secondary_subject_id
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            SubjectServiceError::UnexpectedError(anyhow!(
                "Unable to fetch secondary subject from database, with an error: {}",
                err.to_string()
            ))
        })?
        .ok_or(SubjectServiceError::NotFound)?;

        Ok(SecondarySubject {
            secondary_subject_id: query.secondary_subject_id.to_string(),
            secondary_subject_name: query.name,
            subject_id: query.subject_id.to_string(),
        })
    }

    pub async fn get_secondary_subject_by_name(
        &self,
        transaction: &mut PgTransaction,
        secondary_subject_name: &str,
    ) -> Result<SecondarySubject, SubjectServiceError> {
        let query = sqlx::query!(
            r#"
        select
            *
        from subject_secondary
        where name ilike $1
        limit 1
        "#,
            format!("{secondary_subject_name}%")
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            SubjectServiceError::UnexpectedError(anyhow!(
                "Unable to fetch secondary subject from database, with an error: {}",
                err.to_string()
            ))
        })?
        .ok_or(SubjectServiceError::NotFound)?;

        Ok(SecondarySubject {
            secondary_subject_id: query.secondary_subject_id.to_string(),
            secondary_subject_name: query.name,
            subject_id: query.subject_id.to_string(),
        })
    }
}

impl Default for SubjectService {
    fn default() -> Self {
        Self::new()
    }
}
