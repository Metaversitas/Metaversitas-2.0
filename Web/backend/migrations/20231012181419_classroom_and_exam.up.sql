create table teachers
(
    teacher_id uuid default gen_random_uuid() not null
        constraint teachers_pk
            primary key
        constraint teachers_pk2
            unique,
    name       text                           not null
        constraint teachers_users_identity_full_name_fk
            references users_identity (full_name),
    user_id    uuid                           not null
        constraint teachers_users_user_id_fk
            references users
);

alter table teachers
    owner to admin;

create table subjects
(
    subject_id uuid default gen_random_uuid() not null
        constraint subjects_pk
            primary key
        constraint subjects_pk2
            unique,
    name       text                           not null
);

alter table subjects
    owner to admin;

create table students
(
    student_id uuid default gen_random_uuid() not null
        constraint students_pk
            primary key
        constraint students_pk2
            unique,
    name       text                           not null
        constraint students_users_identity_full_name_fk
            references users_identity (full_name),
    user_id    uuid                           not null
        constraint students_users_user_id_fk
            references users
);

alter table students
    owner to admin;

create table classes
(
    class_id   uuid default gen_random_uuid() not null
        constraint classes_pk
            primary key
        constraint classes_pk3
            unique,
    subject_id uuid                           not null
        constraint classes_subjects_subject_id_fk
            references subjects,
    is_active  boolean                        not null
);

alter table classes
    owner to admin;

create table class_teachers
(
    class_id   uuid not null
        constraint class_teachers_pk
            unique
        constraint class_teachers_classes_class_id_fk
            references classes,
    teacher_id uuid not null
        constraint class_teachers_pk2
            unique
        constraint class_teachers_teachers_teacher_id_fk
            references teachers
);

alter table class_teachers
    owner to admin;

create table class_students
(
    class_id   uuid not null
        constraint class_students_classes_class_id_fk
            references classes,
    student_id uuid not null
        constraint class_students_pk
            unique
        constraint class_students_students_student_id_fk
            references students
);

alter table class_students
    owner to admin;

create table exams
(
    exam_id     uuid                     default gen_random_uuid() not null
        constraint exams_pk
            primary key,
    class_id    uuid                                               not null
        constraint exams_pk3
            unique
        constraint exams_classes_class_id_fk
            references classes,
    subject_id  uuid                                               not null
        constraint exams_pk4
            unique
        constraint exams_subjects_subject_id_fk
            references subjects,
    name        text                                               not null,
    description text,
    created_by  uuid                                               not null
        constraint exams_users_user_id_fk
            references users,
    created_at  timestamp with time zone default now()             not null,
    updated_at  timestamp with time zone default now()             not null
);

alter table exams
    owner to admin;

create table questions
(
    question_id    uuid default gen_random_uuid() not null
        constraint questions_pk
            primary key,
    question_text  text                           not null,
    question_type  question_types                 not null,
    exam_id        uuid                           not null
        constraint questions_exams_exam_id_fk
            references exams,
    question_level question_levels
);

alter table questions
    owner to admin;

create table choices
(
    choice_id   uuid default gen_random_uuid() not null
        constraint choices_pk
            primary key,
    question_id uuid                           not null
        constraint choices_questions_question_id_fk
            references questions,
    choice_text text                           not null,
    is_correct  boolean                        not null
);

alter table choices
    owner to admin;

create table answers
(
    answer_id        uuid default gen_random_uuid() not null
        constraint answers_pk
            primary key,
    user_id          uuid                           not null
        constraint answers_users_user_id_fk
            references users,
    question_id      uuid                           not null
        constraint answers_questions_question_id_fk
            references questions,
    choice_answer_id uuid
        constraint answers_choices_choice_id_fk
            references choices,
    text_answer      text,
    table_answer_id  uuid
);

alter table answers
    owner to admin;

create table answered_tables
(
    answered_table_id uuid default gen_random_uuid() not null
        constraint answered_tables_pk
            primary key,
    answer_id         uuid default gen_random_uuid() not null
        constraint answered_tables_answers_answer_id_fk
            references answers
);

alter table answered_tables
    owner to admin;

alter table answers
    add constraint answers_answered_tables_answered_table_id_fk
        foreign key (table_answer_id) references answered_tables;

create table answered_tables_rows
(
    answered_table_row_id uuid default gen_random_uuid() not null
        constraint answered_tables_rows_pk
            primary key,
    answered_table_id     uuid                           not null
        constraint answered_tables_rows_answered_tables_answered_table_id_fk
            references answered_tables
);

alter table answered_tables_rows
    owner to admin;

create table answered_tables_cells
(
    cell_id      uuid default gen_random_uuid() not null
        constraint answered_tables_cells_pk
            primary key,
    table_row_id uuid                           not null
        constraint table_row_id_constraint
            references answered_tables_rows,
    column_name  text                           not null,
    value        text                           not null
);

alter table answered_tables_cells
    owner to admin;

create table exams_categories
(
    category_id uuid default gen_random_uuid() not null
        constraint exams_categories_pk
            primary key,
    name        text                           not null
);

alter table exams_categories
    owner to admin;

create table exams_users_score
(
    user_id uuid          not null
        constraint exams_users_score_users_user_id_fk
            references users,
    exam_id uuid          not null
        constraint exams_users_score_exams_exam_id_fk
            references exams,
    score   numeric(5, 2) not null
);

alter table exams_users_score
    owner to admin;

create table exam_sessions
(
    session_id uuid default gen_random_uuid() not null
        constraint exam_sessions_pk
            primary key,
    exam_id    uuid                           not null
        constraint exam_sessions_exams_exam_id_fk
            references exams,
    user_id    uuid                           not null
        constraint exam_sessions_users_user_id_fk
            references users,
    start_time timestamp with time zone       not null,
    end_time   timestamp with time zone       not null
);

alter table exam_sessions
    owner to admin;

create table exam_settings
(
    exam_id                   uuid                  not null
        constraint exam_settings_pk
            primary key
        constraint exam_settings_exams_exam_id_fk
            references exams,
    passing_score             numeric(5, 2)         not null,
    multiple_attempts_allowed boolean default false not null,
    randomize_question        boolean default false not null,
    time_limit                integer               not null
);

comment on column exam_settings.time_limit is 'in seconds';

alter table exam_settings
    owner to admin;