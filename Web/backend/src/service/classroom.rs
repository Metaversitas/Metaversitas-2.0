use crate::backend::AppState;
use crate::helpers::errors::classroom::ClassroomServiceError;
use crate::helpers::extractor::AuthenticatedUserWithRole;
use crate::model::classroom::{
    Action, ActionType, ActionTypeUpdateClassMeeting, ActionTypeUpdateExam, BaseAction,
    ClassMeeting, ClassSemester, Classroom, CreateClassMeetingParams, CreateClassroomParams,
    ParamsActionUpdateClassMeeting, ParamsActionUpdateExam, QueryParamsClasses,
    QuerySemesterFilterClass, StudentClassroom, TeacherClassroom, UpcomingScheduled,
    UpcomingScheduledMeetingOrClass, UpdateClassMeetingParams, UpdateClassSubjectParams,
    UpdateClassroomParams,
};
use crate::model::subject::{SecondarySubject, Subject, SubjectWithSecondary};
use crate::model::user::UserUniversityRole;
use crate::r#const::PgTransaction;
use crate::service::exam::ExamService;
use crate::service::subject::SubjectService;
use crate::service::teacher::TeacherService;
use anyhow::anyhow;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use sqlx::{Execute, Postgres, QueryBuilder, Row};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct ClassroomService {
    app_state: Arc<AppState>,
    subject_service: Arc<SubjectService>,
    exam_service: Arc<ExamService>,
    teacher_service: Arc<TeacherService>,
}

// const DEFAULT_CACHE_TIME_EXIST: time::Duration = time::Duration::hours(1);

impl ClassroomService {
    pub fn new(
        app_state: Arc<AppState>,
        subject_service: Arc<SubjectService>,
        exam_service: Arc<ExamService>,
        teacher_service: Arc<TeacherService>,
    ) -> Self {
        Self {
            app_state,
            subject_service,
            exam_service,
            teacher_service,
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
        FROM classes AS new_class
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
        auth_user: &AuthenticatedUserWithRole,
        params: &CreateClassroomParams,
        subject: &Subject,
        secondary_subject: Option<&SecondarySubject>,
    ) -> Result<String, ClassroomServiceError> {
        if matches!(auth_user.university_role, UserUniversityRole::Mahasiswa) {
            return Err(ClassroomServiceError::UnauthorizedStudent);
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("insert into classes (");
        let mut separated = query_builder.separated(", ");

        separated.push_unseparated(
            "name, is_active, semester, year_start, year_end, created_by, have_multiple_meeting, capacity",
        );

        if params.description.is_some() {
            separated.push_unseparated(", description");
        }
        if params.start_time.is_some() {
            separated.push_unseparated(", start_time");
        }
        if params.end_time.is_some() {
            separated.push_unseparated(", end_time");
        }

        separated.push_unseparated(")");
        separated.push_unseparated(" values (");
        separated.push_bind(&params.class_name);
        separated.push_bind(true);
        separated.push_bind(&params.semester);
        let year_start = NaiveDate::from_str(params.year_start.as_str()).map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to parse year_start into date, with an error: {}",
                err.to_string()
            ))
        })?;
        let year_end = NaiveDate::from_str(params.year_end.as_str()).map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to parse year_end into date, with an error: {}",
                err.to_string()
            ))
        })?;
        separated.push_bind(year_start);
        separated.push_bind(year_end);
        separated.push_bind(&auth_user.user_id);
        separated.push_unseparated("::uuid");

        if let Some(meetings) = &params.meetings {
            if meetings.is_empty() {
                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                    "Got a meetings params but the length is 0"
                )));
            } else {
                separated.push_bind(true); // have_multiple_meeting
            }
        } else {
            separated.push_bind(false); // have_multiple_meeting
        }

        if let Some(capacity) = &params.capacity {
            separated.push_bind(capacity);
        } else {
            separated.push_bind(40);
        }

        if let Some(description) = &params.description {
            separated.push_bind(description);
        }
        if let Some(start_time) = &params.start_time {
            separated.push_bind(start_time);
        }
        if let Some(end_time) = &params.end_time {
            separated.push_bind(end_time);
        }
        separated.push_unseparated(r#") returning class_id::text;"#);

        let query = query_builder.build();
        let query = query.fetch_one(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to execute create class, with an error: {}",
                err.to_string()
            ))
        })?;
        let class_id = query.try_get::<String, &str>("class_id").map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to get class_id from fetched query, with an error: {}",
                err.to_string()
            ))
        })?;

        let secondary_subject_id = secondary_subject
            .map(|secondary_subject| secondary_subject.secondary_subject_id.as_str());

        self.create_class_subject(
            transaction,
            class_id.as_str(),
            subject.subject_id.as_str(),
            secondary_subject_id,
        )
        .await?;

        if let Some(meetings) = &params.meetings {
            //TODO: Should change the first_class_meeting to be more valid
            let mut first_class_meeting = true;
            for meeting in meetings {
                let meeting_id = self
                    .create_class_meeting(transaction, class_id.as_str(), meeting)
                    .await?;
                if first_class_meeting {
                    self.update_classes(transaction, class_id.as_str(), &UpdateClassroomParams {
                        class_name: None,
                        semester: None,
                        year_start: None,
                        year_end: None,
                        capacity: None,
                        description: None,
                        is_active: None,
                        current_meeting_id: Some(meeting_id.to_owned()),
                        subjects: None,
                        meetings: None,
                        exams: None,
                        start_time: None,
                        end_time: None,
                        subject_id: None,
                        subject_name: None,
                        students: None,
                        teachers: None,
                    }).await?;
                    first_class_meeting = false;
                }
                let class_meeting = self
                    .get_class_meeting_by_id(transaction, meeting_id.as_str())
                    .await?;
                if let Some(exams) = &meeting.exams {
                    for exam in exams {
                        self.exam_service
                            .insert_class_on_exam_by_id(
                                transaction,
                                exam.exam_id.as_str(),
                                class_id.as_str(),
                                Some(class_meeting.meeting_id.as_str()),
                            )
                            .await
                            .map_err(|err| {
                                ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to insert class_exam, with an error: {}",
                                    err.to_string()
                                ))
                            })?;
                    }
                }
            }
        }

        if params.meetings.is_none() {
            if let Some(exams) = &params.exams {
                for exam in exams {
                    self.exam_service
                        .insert_class_on_exam_by_id(
                            transaction,
                            exam.exam_id.as_str(),
                            class_id.as_str(),
                            None,
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to insert class_exam, with an error: {}",
                                err.to_string()
                            ))
                        })?;
                }
            }
        } else if params.meetings.is_some() && params.exams.is_some() {
            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "Parameter do have multiple meeting therefore exams on the class is not allowed"
            )));
        }

        if let Some(students) = &params.students {
            for student in students {
                self.insert_student_classroom_by_id(
                    &mut *transaction,
                    class_id.as_str(),
                    student.student_id.as_str(),
                )
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to insert student={} into classroom={}, with an error: {}",
                        student.student_id,
                        class_id,
                        err.to_string()
                    ))
                })?;
            }
        }

        if let Some(teachers) = &params.teachers {
            let current_teacher = self
                .teacher_service
                .get_teacher_by_id(transaction, auth_user.user_id.as_str())
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to get teacher_id, with an error: {}",
                        err.to_string()
                    ))
                })?;
            self.insert_teacher_classroom_by_id(
                transaction,
                class_id.as_str(),
                current_teacher.teacher_id.as_str(),
            )
            .await
            .map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!(
                    "Unable to insert teacher into classroom, with an error: {}",
                    err.to_string()
                ))
            })?;
            for teacher in teachers {
                if *teacher.teacher_id == current_teacher.teacher_id {
                    continue;
                }

                self.insert_teacher_classroom_by_id(
                    &mut *transaction,
                    class_id.as_str(),
                    teacher.teacher_id.as_str(),
                )
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to insert teacher={} into classroom={}, with an error: {}",
                        teacher.teacher_id,
                        class_id,
                        err.to_string()
                    ))
                })?;
            }
        } else {
            let current_teacher = self
                .teacher_service
                .get_teacher_by_id(transaction, auth_user.user_id.as_str())
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to get teacher_id, with an error: {}",
                        err.to_string()
                    ))
                })?;
            self.insert_teacher_classroom_by_id(
                &mut *transaction,
                class_id.as_str(),
                current_teacher.teacher_id.as_str(),
            )
            .await?;
        }

        Ok(class_id)
    }

    pub async fn update_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &UpdateClassroomParams,
    ) -> Result<(), ClassroomServiceError> {
        //Check if classroom exists
        let _current_classroom = self
            .get_classroom_by_id(&mut *transaction, class_id)
            .await?;

        if let Some(subjects) = &params.subjects {
            if let Err(err) = self
                .update_class_subject_by_id(transaction, class_id, subjects)
                .await
            {
                let err_msg = err.to_string();
                if !err_msg.contains("No value") {
                    return Err(err);
                }
            }
        }

        if let Some(students) = &params.students {
            match students {
                ActionType::All(list_students) => {
                    self.delete_student_classroom(&mut *transaction, class_id)
                        .await?;

                    for student in list_students {
                        match student {
                            Action::Add(student_id) => {
                                self.insert_student_classroom_by_id(
                                    &mut *transaction,
                                    class_id,
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
                                    class_id,
                                    student_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(student_id) => {
                                self.delete_student_classroom_by_id(
                                    &mut *transaction,
                                    class_id,
                                    student_id.as_str(),
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
        }

        if let Some(teachers) = &params.teachers {
            match teachers {
                ActionType::All(teachers) => {
                    self.delete_teacher_classroom(&mut *transaction, class_id)
                        .await?;

                    for teacher in teachers {
                        match teacher {
                            Action::Add(teacher_id) => {
                                self.insert_teacher_classroom_by_id(
                                    &mut *transaction,
                                    class_id,
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
                                    class_id,
                                    teacher_id.as_str(),
                                )
                                .await?;
                            }
                            Action::Delete(teacher_id) => {
                                self.delete_teacher_classroom_by_id(
                                    &mut *transaction,
                                    class_id,
                                    teacher_id.as_str(),
                                )
                                .await?;
                            }
                        }
                    }
                }
            }
        }

        if let Err(err) = self.update_classes(transaction, class_id, params).await {
            let err_msg = err.to_string();
            if !err_msg.contains("No value") {
                return Err(err);
            }
        };

        let class_meetings = self.get_class_meetings(transaction, class_id).await?;

        if let Some(exams) = &params.exams {
            if !class_meetings.is_empty() {
                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                    "Not able to update exams because of meetings is available."
                )));
            }

            self.update_exam_on_class(transaction, class_id, None, exams)
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to update exam on class={}, meeting_id={}, with an error={}",
                        class_id,
                        "NULL",
                        err.to_string()
                    ))
                })?;
        }

        if let Some(meetings) = &params.meetings {
            self.update_meeting_on_class(transaction, class_id, meetings)
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to update meeting on class={} and err={}",
                        class_id,
                        err.to_string()
                    ))
                })?;
        }

        Ok(())
    }

    async fn update_meeting_on_class(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &ActionTypeUpdateClassMeeting,
    ) -> Result<(), ClassroomServiceError> {
        match params {
            ActionTypeUpdateClassMeeting::All(action_meetings) => {
                self.delete_class_meeting_all(transaction, class_id).await?;
                for action_meeting in action_meetings {
                    match action_meeting.action {
                        BaseAction::Add => {
                            match &action_meeting.params {
                                ParamsActionUpdateClassMeeting::Create {
                                    create_meeting_name, create_meeting_number, create_topic_description, create_description, create_start_time, create_end_time, create_exams
                                } => {
                                    let create_meeting_params = CreateClassMeetingParams {
                                        meeting_name: create_meeting_name.to_owned(),
                                        meeting_number: create_meeting_number.to_owned(),
                                        topic_description: create_topic_description.to_owned(),
                                        description: create_description.to_owned(),
                                        start_time: create_start_time.to_owned(),
                                        end_time: create_end_time.to_owned(),
                                        exams: create_exams.to_owned(),
                                    };
                                    let class_meeting_id = self.create_class_meeting(transaction, class_id, &create_meeting_params)
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                                "Unable to create class_meeting on class={}, with an error: {}", class_id, err.to_string()))
                                    })?;

                                    if let Some(exams) = &create_meeting_params.exams {
                                        self.exam_service.delete_exam_on_class(transaction, class_id, None)
                                        .await
                                        .map_err(|err| {
                                            ClassroomServiceError::UnexpectedError(anyhow!(
                                                    "Unable to delete exam on class={} with error={}", class_id, err.to_string()))
                                        })?;

                                        for exam in exams {
                                            self.exam_service.insert_class_on_exam_by_id(transaction, exam.exam_id.as_str(), class_id, Some(class_meeting_id.as_str()))
                                            .await
                                            .map_err(|err| {
                                                ClassroomServiceError::UnexpectedError(anyhow!(
                                                        "Unable to insert exam on class={}, meeting_id={}, with error={}", class_id, class_meeting_id.as_str(), err.to_string()))
                                            })?;
                                        }
                                    }
                                }
                                ParamsActionUpdateClassMeeting::Update { .. } => {
                                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Update class_meeting is on All therefore params is on mode Update isn't available"
                                        )))
                                }
                            }
                        }
                        BaseAction::Delete => {
                            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                "Update class_meeting is on All therefore Delete isn't available"
                            )))
                        }
                        BaseAction::Edit => {
                            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                "Update class_meeting is on All therefore Edit isn't available"
                            )))
                        }
                    }
                }
            }
            ActionTypeUpdateClassMeeting::Single(action_meetings) => {
                for action_meeting in action_meetings {
                    match action_meeting.action {
                        BaseAction::Add => match &action_meeting.params {
                            ParamsActionUpdateClassMeeting::Create {
                                create_meeting_name, create_meeting_number, create_topic_description, create_description, create_start_time, create_end_time, create_exams
                            } => {
                                let create_meeting_params = CreateClassMeetingParams {
                                    meeting_name: create_meeting_name.to_owned(),
                                    meeting_number: create_meeting_number.to_owned(),
                                    topic_description: create_topic_description.to_owned(),
                                    description: create_description.to_owned(),
                                    start_time: create_start_time.to_owned(),
                                    end_time: create_end_time.to_owned(),
                                    exams: create_exams.to_owned(),
                                };
                                self.create_class_meeting(transaction, class_id, &create_meeting_params)
                                .await
                                .map_err(|err| {
                                    ClassroomServiceError::UnexpectedError(anyhow!(
                                                "Unable to create class_meeting with class_id={} and error={}", class_id, err.to_string()))
                                })?;
                            }
                            ParamsActionUpdateClassMeeting::Update { update_meeting_id, update_meeting_number, update_meeting_name, update_is_active, update_topic_description, update_description, update_start_time, update_end_time, update_exams} => {
                                let update_meeting_params = UpdateClassMeetingParams {
                                    meeting_id: update_meeting_id.to_owned(),
                                    meeting_number: update_meeting_number.to_owned(),
                                    meeting_name: update_meeting_name.to_owned(),
                                    topic_description: update_topic_description.to_owned(),
                                    description: update_description.to_owned(),
                                    start_time: update_start_time.to_owned(),
                                    end_time: update_end_time.to_owned(),
                                    exams: update_exams.to_owned(),
                                    is_active: update_is_active.to_owned(),
                                };
                                self.update_class_meeting_by_id(transaction, class_id, &update_meeting_params)
                                .await
                                .map_err(|err| {
                                    ClassroomServiceError::UnexpectedError(anyhow!(
                                                "Unable to update class_meeting with class_id={} and error={}", class_id, err.to_string()))
                                })?;
                            }
                        },
                        BaseAction::Delete => {
                            match &action_meeting.params {
                                ParamsActionUpdateClassMeeting::Create { .. } => {
                                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of delete therefore it isn't available"
                                        )))
                                }
                                ParamsActionUpdateClassMeeting::Update { update_meeting_id, update_meeting_number, update_meeting_name, update_is_active, update_topic_description, update_description, update_start_time, update_end_time, update_exams } => {
                                    let update_meeting_params = UpdateClassMeetingParams {
                                        meeting_id: update_meeting_id.to_owned(),
                                        meeting_number: update_meeting_number.to_owned(),
                                        meeting_name: update_meeting_name.to_owned(),
                                        topic_description: update_topic_description.to_owned(),
                                        description: update_description.to_owned(),
                                        start_time: update_start_time.to_owned(),
                                        end_time: update_end_time.to_owned(),
                                        exams: update_exams.to_owned(),
                                        is_active: update_is_active.to_owned(),
                                    };
                                    self.delete_class_meeting_by_id(transaction, update_meeting_params.meeting_id.as_str(), class_id)
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                                "Unable to delete class_meeting with meeting_id={}, class_id={}, and error={}",
                                                update_meeting_params.meeting_id,
                                                class_id,
                                                err.to_string()
                                            ))
                                    })?;
                                }
                            }
                        }
                        BaseAction::Edit => {
                            match &action_meeting.params {
                                ParamsActionUpdateClassMeeting::Create { .. } => {
                                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of edit therefore it isn't available"
                                        )))
                                }
                                ParamsActionUpdateClassMeeting::Update { update_meeting_id, update_meeting_number, update_meeting_name, update_is_active, update_topic_description, update_description, update_start_time, update_end_time, update_exams } => {
                                    let update_meeting_params = UpdateClassMeetingParams {
                                        meeting_id: update_meeting_id.to_owned(),
                                        meeting_number: update_meeting_number.to_owned(),
                                        meeting_name: update_meeting_name.to_owned(),
                                        topic_description: update_topic_description.to_owned(),
                                        description: update_description.to_owned(),
                                        start_time: update_start_time.to_owned(),
                                        end_time: update_end_time.to_owned(),
                                        exams: update_exams.to_owned(),
                                        is_active: update_is_active.to_owned(),
                                    };
                                    self.update_class_meeting_by_id(transaction, class_id, &update_meeting_params)
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                                "Unable to update class_meeting with meeting_id={}, class_id={}, and error={}",
                                                update_meeting_params.meeting_id,
                                                class_id,
                                                err.to_string(),
                                            ))
                                    })?;

                                    if let Some(exams) = &update_meeting_params.exams {
                                        self.update_exam_on_class(transaction, class_id, Some(update_meeting_params.meeting_id.as_str()), exams)
                                        .await
                                        .map_err(|err| {
                                            ClassroomServiceError::UnexpectedError(anyhow!("Unable to update exam on class={}, meeting_id={}, and error={}", class_id, update_meeting_params.meeting_id, err.to_string()))
                                        })?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    async fn update_exam_on_class(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        meeting_id: Option<&str>,
        params: &ActionTypeUpdateExam,
    ) -> Result<(), ClassroomServiceError> {
        match params {
            ActionTypeUpdateExam::All(action_exams) => {
                self.exam_service
                    .delete_exam_on_class(transaction, class_id, meeting_id)
                    .await
                    .map_err(|err| {
                        ClassroomServiceError::UnexpectedError(anyhow!(
                            "Unable to delete exam on class={}, with an error: {}",
                            class_id,
                            err.to_string()
                        ))
                    })?;
                for action_exam in action_exams {
                    match action_exam.action {
                        BaseAction::Add => match &action_exam.params {
                            ParamsActionUpdateExam::Create(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of add therefore it isn't available"
                                        )));
                            }
                            ParamsActionUpdateExam::Update(update_exam_params) => {
                                self.exam_service
                                    .insert_class_on_exam_by_id(
                                        transaction,
                                        update_exam_params.exam_id.as_str(),
                                        class_id,
                                        meeting_id,
                                    )
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Unable to insert exam on class={}, with an error: {}",
                                            class_id,
                                            err.to_string()
                                        ))
                                    })?;
                            }
                        },
                        BaseAction::Delete => {
                            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                "Update exam is on All therefore Delete isn't available"
                            )));
                        }
                        BaseAction::Edit => {
                            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                "Update exam is on All therefore Edit isn't available"
                            )));
                        }
                    }
                }
            }
            ActionTypeUpdateExam::Single(action_exams) => {
                for action_exam in action_exams {
                    match action_exam.action {
                        BaseAction::Add => match &action_exam.params {
                            ParamsActionUpdateExam::Create(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of add therefore it isn't available"
                                        )));
                            }
                            ParamsActionUpdateExam::Update(update_exam_params) => {
                                self.exam_service
                                    .insert_class_on_exam_by_id(
                                        transaction,
                                        update_exam_params.exam_id.as_str(),
                                        class_id,
                                        meeting_id,
                                    )
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Unable to insert exam on class={}, with an error: {}",
                                            class_id,
                                            err.to_string()
                                        ))
                                    })?;
                            }
                        },
                        BaseAction::Delete => match &action_exam.params {
                            ParamsActionUpdateExam::Create(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of delete therefore it isn't available"
                                        )));
                            }
                            ParamsActionUpdateExam::Update(update_exam_params) => {
                                self.exam_service.delete_exam_on_class_by_id(transaction, update_exam_params.exam_id.as_str(), class_id, meeting_id).await.map_err(|err| {
                                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to delete class={} on exam={}, with an error: {}", class_id, update_exam_params.exam_id, err.to_string()))
                                })?;
                            }
                        },
                        BaseAction::Edit => match &action_exam.params {
                            ParamsActionUpdateExam::Create(_) => {
                                return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Params is on create mode with action of edit therefore it isn't available"
                                        )));
                            }
                            ParamsActionUpdateExam::Update(update_exam_params) => {
                                self.exam_service
                                    .update_exam_on_class_by_id(
                                        transaction,
                                        update_exam_params.exam_id.as_str(),
                                        class_id,
                                        meeting_id,
                                    )
                                    .await
                                    .map_err(|err| {
                                        ClassroomServiceError::UnexpectedError(anyhow!(
                                            "Unable to edit exam={}, with an error: {}",
                                            update_exam_params.exam_id,
                                            err.to_string()
                                        ))
                                    })?;
                            }
                        },
                    }
                }
            }
        };
        Ok(())
    }

    async fn update_classes(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &UpdateClassroomParams,
    ) -> Result<(), ClassroomServiceError> {
        let mut count = 0;
        let mut _curr_count = 0;
        let mut _count_changed = 0;

        if params.class_name.is_some() {
            count += 1;
        }
        if params.semester.is_some() {
            count += 1;
        }
        if params.description.is_some() {
            count += 1;
        }
        if params.capacity.is_some() {
            count += 1;
        }
        if params.year_start.is_some() {
            count += 1;
        }
        if params.year_end.is_some() {
            count += 1;
        }
        if params.start_time.is_some() {
            count += 1;
        }
        if params.end_time.is_some() {
            count += 1;
        }
        if params.current_meeting_id.is_some() {
            count += 1;
        }
        if params.is_active.is_some() {
            count += 1;
        }

        if count == 0 {
            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("update classes set ");

        if let Some(class_name) = &params.class_name {
            query_builder.push("name = ");
            query_builder.push_bind(class_name);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(semester) = &params.semester {
            query_builder.push("semester = ");
            query_builder.push_bind(semester);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(year_start) = &params.year_start {
            query_builder.push("year_start = ");
            query_builder.push_bind(year_start);
            query_builder.push("::date");
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(year_end) = &params.year_end {
            query_builder.push("year_end = ");
            query_builder.push_bind(year_end);
            query_builder.push("::date");
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(capacity) = &params.capacity {
            query_builder.push("capacity = ");
            query_builder.push_bind(capacity);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(description) = &params.description {
            query_builder.push("description = ");
            query_builder.push_bind(description);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(start_time) = &params.start_time {
            query_builder.push("start_time = ");
            query_builder.push_bind(start_time);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(end_time) = &params.end_time {
            query_builder.push("end_time = ");
            query_builder.push_bind(end_time);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(is_active) = &params.is_active {
            query_builder.push("is_active = ");
            query_builder.push_bind(is_active);
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        if let Some(current_meeting_id) = &params.current_meeting_id {
            query_builder.push("current_meeting_id = ");
            query_builder.push_bind(current_meeting_id);
            query_builder.push("::uuid");
            if count > 1 && _curr_count != count - 1 {
                _curr_count += 1;
                query_builder.push(", ");
            }
            _count_changed += 1;
        }

        query_builder.push(" where class_id::text = ");
        query_builder.push_bind(class_id);

        let query = query_builder.build();

        query.execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Got an error while updating classroom, with an error: {}",
                err.to_string()
            ))
        })?;

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
    ) -> Result<(), ClassroomServiceError> {
        let _query = sqlx::query!(
            r#"
        insert into class_teachers (class_id, teacher_id)
        values ($1, $2);
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
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to insert class_teachers, with an error from database: {}",
                err.to_string()
            ))
        })?;

        Ok(())
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

    pub async fn get_list_teacher_by_classroom(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<Vec<TeacherClassroom>, ClassroomServiceError> {
        let query = sqlx::query_as!(
            TeacherClassroom,
            r#"
        select
            class_teachers.class_id::text as "class_id!",
            teachers.teacher_id::text as "teacher_id!",
            users_identity.full_name as teacher_name
        from class_teachers
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on users_identity.users_id = teachers.user_id
        where class_id::text = $1;
        "#,
            class_id
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch teachers on classroom, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(query)
    }

    pub async fn get_teacher_classroom_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        teacher_id: &str,
    ) -> Result<TeacherClassroom, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
        select
            class_teachers.class_id,
            class_teachers.teacher_id,
            users_identity.full_name
        from class_teachers
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on users_identity.users_id = teachers.user_id
        where class_teachers.class_id::text = $1 and class_teachers.teacher_id::text = $2;
        "#,
            class_id,
            teacher_id
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
            teacher_name: query.full_name,
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

    pub async fn create_class_subject(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        subject_id: &str,
        secondary_subject_id: Option<&str>,
    ) -> Result<(), ClassroomServiceError> {
        let mut query_builder =
            QueryBuilder::<Postgres>::new("insert into class_subjects (class_id, subject_id");
        let mut separated = query_builder.separated(", ");

        if secondary_subject_id.is_some() {
            separated.push_unseparated(", secondary_subject_id");
        }
        separated.push_unseparated(") ");
        separated.push_unseparated("values (");

        separated.push_bind(class_id);
        separated.push_unseparated("::uuid");
        separated.push_bind(subject_id);
        separated.push_unseparated("::uuid");
        if let Some(secondary_subject_id) = secondary_subject_id {
            separated.push_bind(secondary_subject_id);
            separated.push_unseparated("::uuid");
        }
        separated.push_unseparated(")");
        let query = query_builder.build();
        query.execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to create class_subject; \
            class_id={}; subject_id={}; secondary_subject_id={}, \
            with an error: {}",
                class_id,
                subject_id,
                secondary_subject_id.unwrap_or("NULL"),
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn delete_class_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        subject_id: &str,
        secondary_subject_id: Option<&str>,
    ) -> Result<(), ClassroomServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("delete from class_subjects where ");
        query_builder.push("class_id::text = ");
        query_builder.push_bind(class_id);
        query_builder.push(" and ");
        query_builder.push("subject_id::text = ");
        query_builder.push_bind(subject_id);

        if let Some(secondary_subject_id) = secondary_subject_id {
            query_builder.push(" and ");
            query_builder.push("secondary_subject_id::text = ");
            query_builder.push_bind(secondary_subject_id);
        }

        let query = query_builder.build();
        dbg!(&query.sql());
        query.execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!("Unalbe to delete class_subject with class_id={}, subject_id={}, secondary_subject_id={}, and err={}", class_id, subject_id, secondary_subject_id.unwrap_or("NULL"), err.to_string()))
        })?;

        Ok(())
    }

    pub async fn update_class_subject_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &UpdateClassSubjectParams,
    ) -> Result<(), ClassroomServiceError> {
        let mut count = 0;
        let mut _curr_count = 0;

        if params.subject_id.is_some() {
            count += 1;
        }
        if params.secondary_subject_id.is_some() {
            count += 1;
        }

        if count == 0 {
            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let mut query_builder = QueryBuilder::<Postgres>::new("update class_subjects set ");

        if let Some(subject_id) = &params.subject_id {
            query_builder.push("subject_id = ");
            query_builder.push_bind(subject_id);
            query_builder.push("::uuid");

            if count > 0 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        if let Some(secondary_subject_id) = &params.secondary_subject_id {
            query_builder.push("secondary_subject_id = ");
            query_builder.push_bind(secondary_subject_id);
            query_builder.push("::uuid");

            if count > 0 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }
        }

        query_builder.push(" where class_id::text = ");
        query_builder.push_bind(class_id);

        let query = query_builder.build();
        query.execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to update class_subject with class_id={} and error={}",
                class_id,
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn delete_class_meeting_all(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!(
            "delete from class_meeting where class_id::text = $1",
            class_id
        )
        .execute(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to delete class_meeting on class={}, with an error from database: {}",
                class_id,
                err.to_string()
            ))
        })?;

        Ok(())
    }

    pub async fn delete_class_meeting_by_id(
        &self,
        transaction: &mut PgTransaction,
        meeting_id: &str,
        class_id: &str,
    ) -> Result<(), ClassroomServiceError> {
        sqlx::query!("delete from class_meeting where meeting_id::text = $1 and class_id::text = $2", meeting_id, class_id).execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to delete class_meeting on class={} | meeting={}, with an error from database: {}",
                class_id,
                meeting_id,
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn create_class_meeting(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &CreateClassMeetingParams,
    ) -> Result<String, ClassroomServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new(
            "insert into class_meeting (class_id, name, topic_description, meeting_number",
        );

        let mut count = 0;
        let mut _curr_count = 0;

        if params.description.is_some() {
            query_builder.push(", description");
            count += 1;
        }
        if params.start_time.is_some() {
            query_builder.push(", start_time");
            count += 1;
        }
        if params.end_time.is_some() {
            query_builder.push(", end_time");
            count += 1;
        }
        query_builder.push(")");
        query_builder.push(" values (");

        query_builder.push_bind(class_id);
        query_builder.push("::uuid");
        query_builder.push(", ");
        query_builder.push_bind(&params.meeting_name);
        query_builder.push(", ");
        query_builder.push_bind(&params.topic_description);
        query_builder.push(", ");
        query_builder.push_bind(params.meeting_number);
        if count > 0 && _curr_count != count - 1 {
            query_builder.push(", ");
        }

        if let Some(description) = &params.description {
            query_builder.push_bind(description);
            if count > 0 && _curr_count != count - 1 {
                query_builder.push(", ");
            }
            _curr_count += 1;
        }
        if let Some(start_time) = &params.start_time {
            query_builder.push_bind(start_time);
            if count > 0 && _curr_count != count - 1 {
                query_builder.push(", ");
            }
            _curr_count += 1;
        }
        if let Some(end_time) = &params.end_time {
            query_builder.push_bind(end_time);
            if count > 0 && _curr_count != count - 1 {
                query_builder.push(", ");
            }
            _curr_count += 1;
        }
        query_builder.push(")");
        query_builder.push(r#" returning meeting_id::text"#);
        let query = query_builder.build();
        let query = query.fetch_one(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to create class_meeting, with an error: {}",
                err.to_string()
            ))
        })?;
        let meeting_id = query
            .try_get::<String, &str>("meeting_id")
            .map_err(|_err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Not found meeting_id"))
            })?;
        Ok(meeting_id)
    }

    pub async fn update_class_meeting_by_id(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
        params: &UpdateClassMeetingParams,
    ) -> Result<(), ClassroomServiceError> {
        let mut query_builder = QueryBuilder::<Postgres>::new("update class_meeting set ");
        let mut count = 0;
        let mut _curr_count = 0;
        let mut _count_changed = 0;

        if params.meeting_name.is_some() {
            count += 1;
        }
        if params.topic_description.is_some() {
            count += 1;
        }
        if params.description.is_some() {
            count += 1;
        }
        if params.start_time.is_some() {
            count += 1;
        }
        if params.end_time.is_some() {
            count += 1;
        }

        if let Some(meeting_name) = &params.meeting_name {
            query_builder.push("name = ");
            query_builder.push_bind(meeting_name);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }

            _count_changed += 1;
        }

        if let Some(topic_description) = &params.topic_description {
            query_builder.push("topic_description = ");
            query_builder.push_bind(topic_description);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }

            _count_changed += 1;
        }

        if let Some(description) = &params.description {
            query_builder.push("description = ");
            query_builder.push_bind(description);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }

            _count_changed += 1;
        }

        if let Some(start_time) = &params.start_time {
            query_builder.push("start_time = ");
            query_builder.push_bind(start_time);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }

            _count_changed += 1;
        }

        if let Some(end_time) = &params.end_time {
            query_builder.push("end_time = ");
            query_builder.push_bind(end_time);

            if count > 1 && _curr_count != count - 1 {
                query_builder.push(", ");
                _curr_count += 1;
            }

            _count_changed += 1;
        }

        query_builder.push(" where class_id::text = ");
        query_builder.push_bind(class_id);
        query_builder.push(" and meeting_id::text = ");
        query_builder.push_bind(&params.meeting_id);

        if _count_changed == 0 {
            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "No value to be updated"
            )));
        }

        let query = query_builder.build();

        query.execute(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to update class_meeting id={}, with an error: {}",
                params.meeting_id,
                err.to_string()
            ))
        })?;
        Ok(())
    }

    pub async fn get_class_meetings(
        &self,
        transaction: &mut PgTransaction,
        class_id: &str,
    ) -> Result<Vec<ClassMeeting>, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
        select
            meeting_id::text as "meeting_id!",
            class_id::text as "class_id!",
            meeting_number,
            name as meeting_name,
            topic_description,
            description,
            is_active as "is_active!",
            created_at,
            updated_at,
            start_time,
            end_time
        from class_meeting
        where class_id::text = $1;
        "#,
            class_id
        )
        .fetch_all(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch all the class_meeting from database, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut class_meetings = vec![];
        for class_meeting in query {
            class_meetings.push(ClassMeeting {
                meeting_id: class_meeting.meeting_id,
                class_id: class_meeting.class_id,
                meeting_number: class_meeting.meeting_number as i64,
                meeting_name: class_meeting.meeting_name,
                topic_description: class_meeting.topic_description,
                description: class_meeting.description,
                start_time: class_meeting.start_time,
                end_time: class_meeting.end_time,
                created_at: class_meeting.created_at,
                updated_at: class_meeting.updated_at,
                is_active: class_meeting.is_active,
            });
        }

        Ok(class_meetings)
    }

    pub async fn get_class_meeting_by_id(
        &self,
        transaction: &mut PgTransaction,
        meeting_id: &str,
    ) -> Result<ClassMeeting, ClassroomServiceError> {
        let query = sqlx::query!(
            r#"
        select
            meeting_id::text as "meeting_id!",
            class_id::text as "class_id!",
            name,
            meeting_number,
            topic_description,
            description,
            is_active,
            created_at,
            updated_at,
            start_time,
            end_time
        from class_meeting
        where meeting_id::text = $1
        "#,
            meeting_id
        )
        .fetch_one(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch data from database, with an error: {}",
                err.to_string()
            ))
        })?;

        Ok(ClassMeeting {
            meeting_id: query.meeting_id,
            class_id: query.class_id,
            meeting_number: query.meeting_number as i64,
            meeting_name: query.name,
            topic_description: query.topic_description,
            description: query.description,
            start_time: query.start_time,
            end_time: query.end_time,
            created_at: query.created_at,
            updated_at: query.updated_at,
            is_active: query.is_active,
        })
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
            classes.capacity,
            classes.semester as "semester!: ClassSemester",
            classes.year_start,
            classes.year_end,
            classes.have_multiple_meeting,
            classes.current_meeting_id,
            classes.start_time,
            classes.end_time,
            subjects.name as subject_name,
            subjects.subject_id,
            class_subjects.secondary_subject_id
        from classes
            inner join class_subjects on classes.class_id = class_subjects.class_id
            inner join subjects on class_subjects.subject_id = subjects.subject_id
        where classes.class_id::text = $1;
       "#,
            class_id
        )
        .fetch_optional(&mut **transaction)
        .await
        .map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Got an error from database: {}",
                err.to_string()
            ))
        })?
        .ok_or(ClassroomServiceError::UnexpectedError(anyhow!(
            "Not found a classroom with class_id: {}",
            class_id
        )))?;

        let secondary_subject = {
            if let Some(secondary_subject_id) = query.secondary_subject_id {
                Some(
                    self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                "Unable to fetch secondary_subject, with an error: {}",
                                err.to_string()
                            ))
                        })?,
                )
            } else {
                None
            }
        };
        let subject = SubjectWithSecondary {
            subject_id: query.subject_id.to_string(),
            subject_name: query.subject_name,
            secondary_subject,
        };
        let teachers = self
            .get_list_teacher_by_classroom(transaction, class_id)
            .await?;
        let class_meeting = self
            .get_class_meetings(transaction, class_id.to_string().as_str())
            .await?;
        let classroom = Classroom {
            is_active: query.is_active,
            capacity: query.capacity as i64,
            have_multiple_meeting: query.have_multiple_meeting,
            subject,
            meetings: {
                if !class_meeting.is_empty() {
                    Some(class_meeting)
                } else {
                    None
                }
            },
            semester: query.semester,
            class_name: query.name,
            class_id: query.class_id.to_string(),
            description: query.description,
            year_start: query.year_start.year().to_string(),
            year_end: query.year_end.year().to_string(),
            current_meeting_id: query.current_meeting_id.map(|val| val.to_string()),
            start_time: query.start_time,
            end_time: query.end_time,
            teachers: Some(teachers),
            is_enrolled: None,
        };
        Ok(classroom)
    }

    pub async fn get_student_enrolled_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
        select
            classes.class_id,
            classes.name,
            classes.description,
            classes.capacity::int8,
            classes.semester,
            classes.year_start,
            classes.year_end,
            classes.have_multiple_meeting,
            classes.current_meeting_id,
            classes.start_time,
            classes.end_time,
            class_subjects.subject_id,
            class_subjects.secondary_subject_id,
            subjects.name as subject_name
        from students
                 inner join class_students on students.student_id = class_students.student_id
                 inner join classes on class_students.class_id = classes.class_id and classes.is_active = true
                 inner join class_subjects on classes.class_id = class_subjects.class_id
                 inner join subjects on class_subjects.subject_id = subjects.subject_id
                 inner join search_subquery on classes.class_id = search_subquery.class_id
        where user_id::text = "#,
        );
        query_builder.push_bind(user_id);

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query
            .fetch_all(&self.app_state.database)
            .await
            .map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!(
                    "Got an error from database: {}",
                    err.to_string()
                ))
            })?;

        let mut list_classroom: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = tmp_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                            .get_secondary_subject_by_id(
                                transaction,
                                secondary_subject_id.to_string().as_str(),
                            )
                            .await
                            .map_err(|err| {
                                ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                            })?,
                    )
                } else {
                    None
                }
            };
            let subject = SubjectWithSecondary {
                subject_id: tmp_classroom.try_get::<Uuid, &str>("subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id when fetching to database, with an error: {}", err.to_string()))
                })?.to_string(),
                subject_name: tmp_classroom.try_get::<String, &str>("subject_name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                secondary_subject,
            };
            let tmp_class_id = tmp_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let class_meeting = self
                .get_class_meetings(transaction, tmp_class_id.as_str())
                .await?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let classroom = Classroom {
                is_active: true,
                capacity: tmp_classroom.try_get::<i64, &str>("capacity").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse capacity when fetching to database, with an error: {}", err.to_string()))
                })?,
                current_meeting_id: tmp_classroom.try_get::<Option<Uuid>, &str>("current_meeting_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse current_meeting_id when fetching to database, with an error: {}", err.to_string()))
                })?.map(|val| val.to_string()),
                class_name: tmp_classroom.try_get::<String, &str>("name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                description: tmp_classroom.try_get::<Option<String>, &str>("description").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse description when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_id: tmp_class_id,
                subject,
                meetings: Some(class_meeting),
                semester: tmp_classroom.try_get::<ClassSemester, &str>("semester").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse semester when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_start: tmp_classroom.try_get::<NaiveDate, &str>("year_start").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_start when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                have_multiple_meeting: tmp_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_end: tmp_classroom.try_get::<NaiveDate, &str>("year_end").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_end when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                start_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("start_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse start_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                end_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("end_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                teachers: Some(list_teacher),
                is_enrolled: Some(true),
            };
            list_classroom.push(classroom)
        }
        Ok(list_classroom)
    }

    pub async fn get_student_available_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
        select
        classes.is_active,
        classes.class_id,
        classes.name,
        classes.description,
        classes.capacity::int8,
        classes.semester,
        classes.year_start,
        classes.year_end,
        classes.have_multiple_meeting,
        classes.current_meeting_id,
        classes.start_time,
        classes.end_time,
        case when exists (
                select
                    1
                from students
                inner join class_students on students.student_id = class_students.student_id
                where students.user_id::text =
        "#,
        );
        query_builder.push_bind(user_id);
        query_builder.push(
            r#"
                 and classes.class_id = class_students.class_id
            ) then
                true
            else
                false
        end as is_enrolled,
        subjects.name as subject_name,
        subjects.subject_id,
        class_subjects.secondary_subject_id
    from classes
        inner join class_subjects on classes.class_id = class_subjects.class_id
        inner join subjects on class_subjects.subject_id = subjects.subject_id
        inner join search_subquery on classes.class_id = search_subquery.class_id
    where classes.is_active = true
        "#,
        );

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query.fetch_all(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch into database, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
        for fetched_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = fetched_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject = SubjectWithSecondary {
                subject_id: fetched_classroom.try_get::<Uuid, &str>("subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id when fetching to database, with an error: {}", err.to_string()))
                })?.to_string(),
                subject_name: fetched_classroom.try_get::<String, &str>("subject_name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                secondary_subject,
            };
            let tmp_class_id = fetched_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let class_meeting = self
                .get_class_meetings(transaction, tmp_class_id.as_str())
                .await?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let classroom = Classroom {
                is_active: fetched_classroom.try_get::<bool, &str>("is_active").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse is_active when fetching to database, with an error: {}", err.to_string()))
                })?,
                capacity: fetched_classroom.try_get::<i64, &str>("capacity").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse capacity when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_name: fetched_classroom.try_get::<String, &str>("name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                description: fetched_classroom.try_get::<Option<String>, &str>("description").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse description when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_id: tmp_class_id,
                subject,
                meetings: Some(class_meeting),
                semester: fetched_classroom.try_get::<ClassSemester, &str>("semester").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse semester when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_start: fetched_classroom.try_get::<NaiveDate, &str>("year_start").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_start when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                have_multiple_meeting: fetched_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_end: fetched_classroom.try_get::<NaiveDate, &str>("year_end").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_end when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                current_meeting_id: fetched_classroom.try_get::<Option<Uuid>, &str>("current_meeting_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse current_meeting_id when fetching to database, with an error: {}", err.to_string()))
                })?.map(|val| val.to_string()),
                start_time: fetched_classroom.try_get::<Option<DateTime<Utc>>, &str>("start_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse start_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                end_time: fetched_classroom.try_get::<Option<DateTime<Utc>>, &str>("end_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                teachers: Some(list_teacher),
                is_enrolled: Some(fetched_classroom.try_get::<bool, &str>("is_enrolled").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?),
            };
            classrooms.push(classroom);
        }
        Ok(classrooms)
    }

    pub async fn get_student_upcoming_scheduled_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<UpcomingScheduled>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(r#"
                select
            classes.class_id,
            classes.name,
            classes.have_multiple_meeting,
            classes.current_meeting_id,
            case
                when classes.have_multiple_meeting = true
                    then class_meeting.start_time
                else classes.start_time
            end as start_time,
            case
                when classes.have_multiple_meeting = true
                    then class_meeting.end_time
                else
                    classes.end_time
            end as end_time,
            class_meeting.meeting_id,
            class_meeting.meeting_number,
            class_subjects.subject_id,
            class_subjects.secondary_subject_id
        from students
            inner join class_students on students.student_id = class_students.student_id
            inner join classes on class_students.class_id = classes.class_id and classes.is_active = true
            inner join class_subjects on classes.class_id = class_subjects.class_id
            inner join subjects on class_subjects.subject_id = subjects.subject_id
            inner join search_subquery on classes.class_id = search_subquery.class_id
            left join class_meeting on classes.class_id = class_meeting.class_id and classes.have_multiple_meeting = true
        where students.user_id::text =
        "#);
        query_builder.push_bind(user_id);
        query_builder.push(
            r#"
                and (
                exists(
                        select 1
                        from classes
                                 inner join class_meeting on classes.class_id = class_meeting.class_id
                        where classes.have_multiple_meeting = true
                          and class_meeting.start_time is not null
                          and class_meeting.end_time is not null
                          and class_meeting.start_time > current_timestamp
                    )
                or exists
                    (
                        select 1
                        from classes
                        where classes.have_multiple_meeting = false
                          and classes.start_time is not null
                          and classes.end_time is not null
                          and classes.start_time > current_timestamp
                    )
            )
        order by
            case
                when classes.have_multiple_meeting = true then class_meeting.start_time
                else classes.start_time
            end,
            case
                when classes.have_multiple_meeting = true then class_meeting.end_time
                else classes.end_time
            end
        "#,
        );

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query.fetch_all(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch upcoming scheduled class for student, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut upcomings_scheduled: Vec<UpcomingScheduled> = Vec::with_capacity(query.len());
        for fetched_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = fetched_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject_id = fetched_classroom
                .try_get::<Uuid, &str>("subject_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse subject_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let subject = self
                .subject_service
                .get_subject_by_id(transaction, subject_id.as_str())
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to get a subject={}, with an error: {}",
                        subject_id,
                        err.to_string()
                    ))
                })?;
            let subject_with_secondary = SubjectWithSecondary {
                subject_id: subject.subject_id,
                subject_name: subject.subject_name,
                secondary_subject,
            };

            let tmp_class_id = fetched_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let tmp_class_name =
                fetched_classroom
                    .try_get::<String, &str>("name")
                    .map_err(|err| {
                        ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_name when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                    })?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let tmp_current_meeting_id = fetched_classroom
                .try_get::<Option<Uuid>, &str>("current_meeting_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?;
            let tmp_have_multiple_meeting = fetched_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetch to database, with an error: {}", err.to_string()))
            })?;
            let tmp_meeting_id = fetched_classroom
                .try_get::<Option<Uuid>, &str>("meeting_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?;

            let upcoming_scheduled: UpcomingScheduled;

            if tmp_have_multiple_meeting {
                let Some(_) = tmp_current_meeting_id else {
                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                        "current_meeting_id should be available"
                    )));
                };
                let Some(meeting_id) = tmp_meeting_id else {
                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                        "meeting_id should be available"
                    )));
                };
                let meeting = self
                    .get_class_meeting_by_id(transaction, meeting_id.to_string().as_str())
                    .await?;

                upcoming_scheduled = UpcomingScheduled {
                    class_id: tmp_class_id,
                    class_name: tmp_class_name,
                    teachers: Some(list_teacher),
                    subject: subject_with_secondary,
                    upcoming: UpcomingScheduledMeetingOrClass::Meeting(meeting),
                };
            } else {
                let classroom = self
                    .get_classroom_by_id(transaction, tmp_class_id.as_str())
                    .await?;

                upcoming_scheduled = UpcomingScheduled {
                    class_id: tmp_class_id,
                    class_name: tmp_class_name,
                    teachers: Some(list_teacher),
                    subject: subject_with_secondary,
                    upcoming: UpcomingScheduledMeetingOrClass::Class(classroom),
                };
            }

            upcomings_scheduled.push(upcoming_scheduled);
        }
        Ok(upcomings_scheduled)
    }

    pub async fn get_lecturer_enrolled_classroom(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
        with search_subquery as (
    select
        classes.class_id
    from classes
    inner join class_teachers on classes.class_id = class_teachers.class_id
    inner join teachers on class_teachers.teacher_id = teachers.teacher_id
    inner join users_identity on teachers.user_id = users_identity.users_id
    where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
            select
                classes.is_active,
                classes.class_id,
                classes.name,
                classes.description,
                classes.capacity::int8,
                classes.semester,
                classes.year_start,
                classes.year_end,
                classes.have_multiple_meeting,
                classes.current_meeting_id,
                classes.start_time,
                classes.end_time,
                class_subjects.subject_id,
                class_subjects.secondary_subject_id,
                subjects.name as subject_name
        from teachers
                 inner join class_teachers on teachers.teacher_id = class_teachers.teacher_id
                 inner join classes on class_teachers.class_id = classes.class_id and classes.is_active = true
                 inner join class_subjects on classes.class_id = class_subjects.class_id
                 inner join subjects on class_subjects.subject_id = subjects.subject_id
                 inner join search_subquery on search_subquery.class_id = classes.class_id
        where teachers.user_id::text =
        "#,
        );
        query_builder.push_bind(user_id);
        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query
            .fetch_all(&self.app_state.database)
            .await
            .map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!(
                    "Got an error from database: {}",
                    err.to_string()
                ))
            })?;

        let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = tmp_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                   ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject = SubjectWithSecondary {
                subject_id: tmp_classroom.try_get::<Uuid, &str>("subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id when fetching to database, with an error: {}", err.to_string()))
                })?.to_string(),
                subject_name: tmp_classroom.try_get::<String, &str>("subject_name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                secondary_subject,
            };
            let tmp_class_id = tmp_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let class_meeting = self
                .get_class_meetings(transaction, tmp_class_id.as_str())
                .await?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let classroom = Classroom {
                is_active: tmp_classroom.try_get::<bool, &str>("is_active").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse is_active when fetching to database, with an error: {}", err.to_string()))
                })?,
                capacity: tmp_classroom.try_get::<i64, &str>("capacity").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse capacity when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_name: tmp_classroom.try_get::<String, &str>("name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                description: tmp_classroom.try_get::<Option<String>, &str>("description").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse description when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_id: tmp_class_id,
                subject,
                meetings: Some(class_meeting),
                semester: tmp_classroom.try_get::<ClassSemester, &str>("semester").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse semester when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_start: tmp_classroom.try_get::<NaiveDate, &str>("year_start").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_start when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                have_multiple_meeting: tmp_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_end: tmp_classroom.try_get::<NaiveDate, &str>("year_end").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_end when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                current_meeting_id: tmp_classroom.try_get::<Option<Uuid>, &str>("current_meeting_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse current_meeting_id when fetching to database, with an error: {}", err.to_string()))
                })?.map(|val| val.to_string()),
                start_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("start_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse start_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                end_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("end_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                teachers: Some(list_teacher),
                is_enrolled: Some(true),
            };
            classrooms.push(classroom);
        }
        Ok(classrooms)
    }

    pub async fn get_lecturer_created_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
        select
            classes.is_active,
            classes.class_id,
            classes.name,
            classes.description,
            classes.capacity::int8,
            classes.semester,
            classes.year_start,
            classes.year_end,
            classes.have_multiple_meeting,
            classes.current_meeting_id,
            classes.start_time,
            classes.end_time,
            subjects.name as subject_name,
            subjects.subject_id,
            true as is_enrolled,
            class_subjects.secondary_subject_id
        from classes
            inner join class_subjects on classes.class_id = class_subjects.class_id
            inner join subjects on class_subjects.subject_id = subjects.subject_id
            inner join search_subquery on classes.class_id = search_subquery.class_id
        where classes.created_by::text =
        "#,
        );
        query_builder.push_bind(user_id);

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query.fetch_all(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to get lecturer created class from database, with an error: {}",
                err.to_string()
            ))
        })?;
        let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = tmp_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject = SubjectWithSecondary {
                subject_id: tmp_classroom.try_get::<Uuid, &str>("subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id when fetching to database, with an error: {}", err.to_string()))
                })?.to_string(),
                subject_name: tmp_classroom.try_get::<String, &str>("subject_name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                secondary_subject,
            };
            let tmp_class_id = tmp_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let class_meeting = self
                .get_class_meetings(transaction, tmp_class_id.as_str())
                .await?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let classroom = Classroom {
                is_active: tmp_classroom.try_get::<bool, &str>("is_active").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse is_active when fetching to database, with an error: {}", err.to_string()))
                })?,
                capacity: tmp_classroom.try_get::<i64, &str>("capacity").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse capacity when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_name: tmp_classroom.try_get::<String, &str>("name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                description: tmp_classroom.try_get::<Option<String>, &str>("description").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse description when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_id: tmp_class_id,
                subject,
                meetings: Some(class_meeting),
                semester: tmp_classroom.try_get::<ClassSemester, &str>("semester").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse semester when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_start: tmp_classroom.try_get::<NaiveDate, &str>("year_start").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_start when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                have_multiple_meeting: tmp_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_end: tmp_classroom.try_get::<NaiveDate, &str>("year_end").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_end when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                current_meeting_id: tmp_classroom.try_get::<Option<Uuid>, &str>("current_meeting_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse current_meeting_id when fetching to database, with an error: {}", err.to_string()))
                })?.map(|val| val.to_string()),
                start_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("start_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse start_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                end_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("end_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                teachers: Some(list_teacher),
                is_enrolled: Some(tmp_classroom.try_get::<bool, &str>("is_enrolled").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse is_enrolled when fetching to database, with an error: {}", err.to_string()))
                })?),
            };
            classrooms.push(classroom);
        }
        Ok(classrooms)
    }

    pub async fn get_lecturer_available_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<Classroom>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
        select
            classes.is_active,
            classes.class_id,
            classes.name,
            classes.description,
            classes.capacity::int8,
            classes.semester,
            classes.year_start,
            classes.year_end,
            classes.have_multiple_meeting,
            classes.current_meeting_id,
            classes.start_time,
            classes.end_time,
        "#,
        );
        query_builder.push(
            r#"
        case when exists (
            select
                1
            from teachers
            inner join class_teachers on teachers.teacher_id = class_teachers.teacher_id
            where teachers.user_id::text =
        "#,
        );
        query_builder.push_bind(user_id);
        query_builder.push(
            r#"
        and class_teachers.class_id = classes.class_id) then
            true
        else
            false
            end as is_enrolled,
            subjects.name as subject_name,
            subjects.subject_id,
            class_subjects.secondary_subject_id
        from classes
            inner join class_subjects on classes.class_id = class_subjects.class_id
            inner join subjects on class_subjects.subject_id = subjects.subject_id
            inner join search_subquery on classes.class_id = search_subquery.class_id
        "#,
        );

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query.fetch_all(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch data to database, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut classrooms: Vec<Classroom> = Vec::with_capacity(query.len());
        for tmp_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = tmp_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject = SubjectWithSecondary {
                subject_id: tmp_classroom.try_get::<Uuid, &str>("subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_id when fetching to database, with an error: {}", err.to_string()))
                })?.to_string(),
                subject_name: tmp_classroom.try_get::<String, &str>("subject_name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse subject_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                secondary_subject,
            };
            let tmp_class_id = tmp_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let class_meeting = self
                .get_class_meetings(transaction, tmp_class_id.as_str())
                .await?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let classroom = Classroom {
                is_active: tmp_classroom.try_get::<bool, &str>("is_active").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse is_active when fetching to database, with an error: {}", err.to_string()))
                })?,
                capacity: tmp_classroom.try_get::<i64, &str>("capacity").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse capacity when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_name: tmp_classroom.try_get::<String, &str>("name").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse class_name when fetching to database, with an error: {}", err.to_string()))
                })?,
                description: tmp_classroom.try_get::<Option<String>, &str>("description").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse description when fetching to database, with an error: {}", err.to_string()))
                })?,
                class_id: tmp_class_id,
                subject,
                meetings: Some(class_meeting),
                semester: tmp_classroom.try_get::<ClassSemester, &str>("semester").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse semester when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_start: tmp_classroom.try_get::<NaiveDate, &str>("year_start").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_start when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                have_multiple_meeting: tmp_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetching to database, with an error: {}", err.to_string()))
                })?,
                year_end: tmp_classroom.try_get::<NaiveDate, &str>("year_end").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse year_end when fetching to database, with an error: {}", err.to_string()))
                })?.year().to_string(),
                current_meeting_id: tmp_classroom.try_get::<Option<Uuid>, &str>("current_meeting_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse current_meeting_id when fetching to database, with an error: {}", err.to_string()))
                })?.map(|val| val.to_string()),
                start_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("start_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse start_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                end_time: tmp_classroom.try_get::<Option<DateTime<Utc>>, &str>("end_time").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?,
                teachers: Some(list_teacher),
                is_enrolled: Some(tmp_classroom.try_get::<bool, &str>("is_enrolled").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse end_time when fetching to database, with an error: {}", err.to_string()))
                })?),
            };
            classrooms.push(classroom);
        }
        Ok(classrooms)
    }

    pub async fn get_lecturer_upcoming_scheduled_classes(
        &self,
        transaction: &mut PgTransaction,
        user_id: &str,
        params: &QueryParamsClasses,
    ) -> Result<Vec<UpcomingScheduled>, ClassroomServiceError> {
        let mut search: String = "%%".to_string();
        if let Some(search_params) = &params.search {
            search = format!("%{}%", search_params)
        }
        let mut query_builder = QueryBuilder::<Postgres>::new(
            r#"
            with search_subquery as (
        select
            classes.class_id
        from classes
        inner join class_teachers on classes.class_id = class_teachers.class_id
        inner join teachers on class_teachers.teacher_id = teachers.teacher_id
        inner join users_identity on teachers.user_id = users_identity.users_id
        where (classes.name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(
            r#"
         or users_identity.full_name ilike
        "#,
        );
        query_builder.push_bind(&search);
        query_builder.push(r#"))"#);
        query_builder.push(
            r#"
    select
    classes.class_id,
    classes.name,
    classes.have_multiple_meeting,
    classes.current_meeting_id,
    case
        when classes.have_multiple_meeting = true
            then class_meeting.start_time
        else classes.start_time
    end as start_time,
    case
        when classes.have_multiple_meeting = true
            then class_meeting.end_time
        else
            classes.end_time
    end as end_time,
    class_meeting.meeting_id,
    class_meeting.meeting_number,
    class_subjects.subject_id,
    class_subjects.secondary_subject_id
from teachers
    inner join class_teachers on teachers.teacher_id = class_teachers.teacher_id
    inner join classes on class_teachers.class_id = classes.class_id and classes.is_active = true
    inner join class_subjects on classes.class_id = class_subjects.class_id
    inner join search_subquery on classes.class_id = search_subquery.class_id
    left join class_meeting on classes.class_id = class_meeting.class_id and classes.have_multiple_meeting = true
where teachers.user_id::text =
        "#,
        );
        query_builder.push_bind(user_id);
        query_builder.push(
            r#"
        and (
        exists(
                select 1
                from classes
                         inner join class_meeting on classes.class_id = class_meeting.class_id
                where classes.have_multiple_meeting = true
                  and class_meeting.start_time is not null
                  and class_meeting.end_time is not null
                  and class_meeting.start_time > current_timestamp
            )
        or exists
            (
                select 1
                from classes
                where classes.have_multiple_meeting = false
                  and classes.start_time is not null
                  and classes.end_time is not null
                  and classes.start_time > current_timestamp
            )
    )
order by
    case
        when classes.have_multiple_meeting = true then class_meeting.start_time
        else classes.start_time
    end,
    case
        when classes.have_multiple_meeting = true then class_meeting.end_time
        else classes.end_time
    end
        "#,
        );

        self.handle_query_params_filter_classes(params, &mut query_builder)
            .await?;

        let query = query_builder.build();
        let query = query.fetch_all(&mut **transaction).await.map_err(|err| {
            ClassroomServiceError::UnexpectedError(anyhow!(
                "Unable to fetch upcoming scheduled class for lecturer, with an error: {}",
                err.to_string()
            ))
        })?;

        let mut upcomings_scheduled: Vec<UpcomingScheduled> = Vec::with_capacity(query.len());

        for fetched_classroom in query {
            let secondary_subject = {
                if let Some(secondary_subject_id) = fetched_classroom.try_get::<Option<Uuid>, &str>("secondary_subject_id").map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse secondary_subject_id when fetching to database, with an error: {}", err.to_string()))
                })? {
                    Some(
                        self.subject_service
                        .get_secondary_subject_by_id(
                            transaction,
                            secondary_subject_id.to_string().as_str(),
                        )
                        .await
                        .map_err(|err| {
                            ClassroomServiceError::UnexpectedError(anyhow!(
                                    "Unable to fetch secondary_subject, with an error: {}",
                                    err.to_string()
                                ))
                        })?,
                    )
                } else {
                    None
                }
            };
            let subject_id = fetched_classroom
                .try_get::<Uuid, &str>("subject_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse subject_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let subject = self
                .subject_service
                .get_subject_by_id(transaction, subject_id.as_str())
                .await
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to get a subject={}, with an error: {}",
                        subject_id,
                        err.to_string()
                    ))
                })?;
            let subject_with_secondary = SubjectWithSecondary {
                subject_id: subject.subject_id,
                subject_name: subject.subject_name,
                secondary_subject,
            };

            let tmp_class_id = fetched_classroom
                .try_get::<Uuid, &str>("class_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?
                .to_string();
            let tmp_class_name =
                fetched_classroom
                    .try_get::<String, &str>("name")
                    .map_err(|err| {
                        ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_name when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                    })?;
            let list_teacher = self
                .get_list_teacher_by_classroom(transaction, tmp_class_id.as_str())
                .await?;
            let tmp_current_meeting_id = fetched_classroom
                .try_get::<Option<Uuid>, &str>("current_meeting_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?;
            let tmp_have_multiple_meeting = fetched_classroom.try_get::<bool, &str>("have_multiple_meeting").map_err(|err| {
                ClassroomServiceError::UnexpectedError(anyhow!("Unable to parse have_multiple_meeting when fetch to database, with an error: {}", err.to_string()))
            })?;
            let tmp_meeting_id = fetched_classroom
                .try_get::<Option<Uuid>, &str>("meeting_id")
                .map_err(|err| {
                    ClassroomServiceError::UnexpectedError(anyhow!(
                        "Unable to parse class_id when fetching to database, with an error: {}",
                        err.to_string()
                    ))
                })?;

            let upcoming_scheduled: UpcomingScheduled;

            if tmp_have_multiple_meeting {
                let Some(_) = tmp_current_meeting_id else {
                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                        "current_meeting_id should be available"
                    )));
                };
                let Some(meeting_id) = tmp_meeting_id else {
                    return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                        "meeting_id should be available"
                    )));
                };
                let meeting = self
                    .get_class_meeting_by_id(transaction, meeting_id.to_string().as_str())
                    .await?;

                upcoming_scheduled = UpcomingScheduled {
                    class_id: tmp_class_id,
                    class_name: tmp_class_name,
                    teachers: Some(list_teacher),
                    subject: subject_with_secondary,
                    upcoming: UpcomingScheduledMeetingOrClass::Meeting(meeting),
                };
            } else {
                let classroom = self
                    .get_classroom_by_id(transaction, tmp_class_id.as_str())
                    .await?;

                upcoming_scheduled = UpcomingScheduled {
                    class_id: tmp_class_id,
                    class_name: tmp_class_name,
                    teachers: Some(list_teacher),
                    subject: subject_with_secondary,
                    upcoming: UpcomingScheduledMeetingOrClass::Class(classroom),
                };
            }

            upcomings_scheduled.push(upcoming_scheduled);
        }
        Ok(upcomings_scheduled)
    }

    async fn handle_query_params_filter_classes(
        &self,
        params: &QueryParamsClasses,
        query_builder: &mut QueryBuilder<'_, Postgres>,
    ) -> Result<(), ClassroomServiceError> {
        if let Some(filter) = &params.filter {
            if let Some(semester_filter) = &filter.semester_filter {
                query_builder.push(" and classes.semester::text = ");
                match semester_filter {
                    QuerySemesterFilterClass::Odd => {
                        query_builder.push_bind("odd");
                    }
                    QuerySemesterFilterClass::Even => {
                        query_builder.push_bind("even");
                    }
                }
            }

            if let Some(subject_name_filter) = &filter.subject_name_filter {
                query_builder.push(" and subjects.name ilike ");
                query_builder.push_bind(format!("{}%", subject_name_filter.to_owned()));
            }

            if let Some(subject_id_filter) = &filter.subject_id_filter {
                query_builder.push(" and subjects.subject_id::text = ");
                query_builder.push_bind(subject_id_filter.to_owned());
            }
        }

        // limit params
        let Some(pagination) = &params.pagination else {
            return Err(ClassroomServiceError::UnexpectedError(anyhow!(
                "Query pagination is not found"
            )));
        };
        query_builder.push(" limit ");
        if let Some(limit) = &pagination.limit {
            query_builder.push_bind(*limit as i32);
        } else {
            query_builder.push_bind(10);
        }

        // offset params
        if let Some(offset) = &pagination.offset {
            query_builder.push(" offset ");
            query_builder.push_bind(*offset as i32);
        }

        Ok(())
    }
}
