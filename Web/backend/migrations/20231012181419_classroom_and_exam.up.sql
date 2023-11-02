create type question_types as enum ('choice', 'descriptive', 'table');
create table public.questions (
                                  question_id uuid primary key not null default gen_random_uuid(),
                                  question_text text not null,
                                  question_type question_types not null,
                                  table_question jsonb
);

create table public.question_choices (
                                         choice_id uuid primary key not null default gen_random_uuid(),
                                         question_id uuid not null,
                                         choice_text text not null,
                                         is_correct boolean not null,
                                         foreign key (question_id) references public.questions (question_id)
                                             match simple on update cascade on delete cascade
);

create table public.students (
                                 student_id uuid primary key not null default gen_random_uuid(),
                                 user_id uuid not null,
                                 foreign key (user_id) references public.users (user_id)
                                     match simple on update cascade on delete cascade
);
create unique index students_pk2 on students using btree (student_id);
create unique index students_unique_student_id_user_id on students using btree (student_id, user_id);


create table public.student_answers (
                                        answer_id uuid primary key not null default gen_random_uuid(),
                                        user_id uuid not null,
                                        question_id uuid not null,
                                        choice_answer_id uuid,
                                        text_answer text,
                                        table_answer jsonb,
                                        foreign key (choice_answer_id) references public.question_choices (choice_id)
                                            match simple on update cascade on delete cascade,
                                        foreign key (question_id) references public.questions (question_id)
                                            match simple on update cascade on delete cascade,
                                        foreign key (user_id) references public.users (user_id)
                                            match simple on update cascade on delete cascade
);

create table public.student_schedule (
                                         schedule_id uuid not null default gen_random_uuid(),
                                         student_id uuid not null,
                                         start_time timestamp with time zone not null,
                                         end_time timestamp with time zone,
                                         primary key (schedule_id, student_id),
                                         foreign key (student_id) references public.students (student_id)
                                             match simple on update cascade on delete cascade
);

create table public.teachers (
                                 teacher_id uuid primary key not null default gen_random_uuid(),
                                 user_id uuid not null,
                                 foreign key (user_id) references public.users (user_id)
                                     match simple on update cascade on delete cascade
);
create unique index teachers_pk2 on teachers using btree (teacher_id);
create unique index teachers_unique_teacher_id_user_id on teachers using btree (teacher_id, user_id);


create table public.teacher_schedule (
                                         schedule_id uuid not null default gen_random_uuid(),
                                         teacher_id uuid not null,
                                         start_time timestamp with time zone not null,
                                         end_time timestamp with time zone,
                                         primary key (schedule_id, teacher_id),
                                         foreign key (teacher_id) references public.teachers (teacher_id)
                                             match simple on update cascade on delete cascade
);

create table public.subjects (
                                 subject_id uuid primary key not null default gen_random_uuid(),
                                 name text not null
);
create unique index subjects_pk2 on subjects using btree (subject_id);

create table public.subject_secondary (
                                          secondary_subject_id uuid not null default gen_random_uuid(),
                                          subject_id uuid not null,
                                          name text not null,
                                          primary key (secondary_subject_id, subject_id)
);
create unique index subject_unique_secondary_pk on subject_secondary using btree (secondary_subject_id);


create type semester as enum ('odd', 'even');
create table public.classes (
                                class_id uuid primary key not null default gen_random_uuid(),
                                is_active boolean not null,
                                name text not null,
                                description text,
                                capacity integer not null,
                                current_meeting_id uuid,
                                have_multiple_meeting boolean not null,
                                semester semester not null,
                                year_start date not null,
                                year_end date not null,
                                created_by uuid not null,
                                start_time timestamp with time zone,
                                end_time timestamp with time zone,
                                foreign key (current_meeting_id) references public.class_meeting (meeting_id)
                                    match simple on update cascade on delete set null,
                                foreign key (created_by) references public.users (user_id)
                                    match simple on update no action on delete no action
);

create table public.class_grades (
                                     grade_id uuid not null,
                                     student_id uuid not null,
                                     class_id uuid not null,
                                     score numeric(5,2) not null,
                                     meeting_id uuid,
                                     primary key (grade_id, student_id, class_id),
                                     foreign key (class_id) references public.classes (class_id)
                                         match simple on update cascade on delete cascade,
                                     foreign key (meeting_id) references public.class_meeting (meeting_id)
                                         match simple on update cascade on delete set null,
                                     foreign key (student_id) references public.students (student_id)
                                         match simple on update cascade on delete cascade
);

create table public.class_meeting (
                                      meeting_id uuid not null default gen_random_uuid(),
                                      class_id uuid not null,
                                      name text not null,
                                      description text,
                                      start_time timestamp with time zone,
                                      end_time timestamp with time zone,
                                      topic_description text not null,
                                      created_at timestamp with time zone not null default now(),
                                      updated_at timestamp with time zone not null default now(),
                                      meeting_number integer not null,
                                      primary key (meeting_id, class_id),
                                      foreign key (class_id) references public.classes (class_id)
                                          match simple on update cascade on delete cascade
);
create unique index class_meeting_pk on class_meeting using btree (meeting_id);

create table public.class_students (
                                       class_id uuid not null,
                                       student_id uuid not null,
                                       primary key (class_id, student_id),
                                       foreign key (class_id) references public.classes (class_id)
                                           match simple on update cascade on delete cascade,
                                       foreign key (student_id) references public.students (student_id)
                                           match simple on update cascade on delete cascade
);

create table public.class_subjects (
                                       class_id uuid not null,
                                       subject_id uuid not null,
                                       secondary_subject_id uuid,
                                       foreign key (class_id) references public.classes (class_id)
                                           match simple on update cascade on delete cascade,
                                       foreign key (subject_id) references public.subjects (subject_id)
                                           match simple on update cascade on delete cascade,
                                       foreign key (secondary_subject_id) references public.subject_secondary (secondary_subject_id)
                                           match simple on update cascade on delete set null
);

create table public.class_teachers (
                                       class_id uuid not null,
                                       teacher_id uuid not null,
                                       primary key (class_id, teacher_id),
                                       foreign key (class_id) references public.classes (class_id)
                                           match simple on update cascade on delete cascade,
                                       foreign key (teacher_id) references public.teachers (teacher_id)
                                           match simple on update cascade on delete cascade
);

create type exam_type as enum ('upload', 'default');
create table public.exams (
                              exam_id uuid primary key not null default gen_random_uuid(),
                              name text not null,
                              description text,
                              created_by uuid not null,
                              created_at timestamp with time zone not null default now(),
                              updated_at timestamp with time zone not null default now(),
                              type exam_type not null,
                              foreign key (created_by) references public.users (user_id)
                                  match simple on update cascade on delete cascade
);

create table public.exam_classes (
                                     exam_id uuid not null,
                                     class_id uuid not null,
                                     meeting_id uuid,
                                     foreign key (class_id) references public.classes (class_id)
                                         match simple on update cascade on delete cascade,
                                     foreign key (meeting_id) references public.class_meeting (meeting_id)
                                         match simple on update cascade on delete set null,
                                     foreign key (exam_id) references public.exams (exam_id)
                                         match simple on update cascade on delete cascade
);
create unique index exam_classes_pk on exam_classes using btree (meeting_id, exam_id, class_id);

create table public.exam_sessions (
                                      session_id uuid not null default gen_random_uuid(),
                                      exam_id uuid not null,
                                      user_id uuid not null,
                                      start_time timestamp with time zone not null,
                                      end_time timestamp with time zone not null,
                                      primary key (session_id, exam_id, user_id),
                                      foreign key (exam_id) references public.exams (exam_id)
                                          match simple on update no action on delete no action,
                                      foreign key (user_id) references public.users (user_id)
                                          match simple on update no action on delete no action
);

create table public.exam_settings (
                                      exam_id uuid primary key not null,
                                      passing_score numeric(5,2) not null,
                                      multiple_attempts_allowed boolean not null default false,
                                      randomize_question boolean not null default false,
                                      time_limit integer not null, -- in seconds
                                      foreign key (exam_id) references public.exams (exam_id)
                                          match simple on update no action on delete no action
);
comment on column public.exam_settings.time_limit is 'in seconds';

create table public.exam_subject (
                                     subject_id uuid not null,
                                     exam_id uuid not null,
                                     secondary_subject_id uuid,
                                     primary key (subject_id, exam_id),
                                     foreign key (exam_id) references public.exams (exam_id)
                                         match simple on update no action on delete no action,
                                     foreign key (secondary_subject_id) references public.subject_secondary (secondary_subject_id)
                                         match simple on update no action on delete no action,
                                     foreign key (subject_id) references public.subjects (subject_id)
                                         match simple on update no action on delete no action
);

create table public.question_exams (
                                       question_id uuid not null,
                                       exam_id uuid not null,
                                       primary key (question_id, exam_id),
                                       foreign key (exam_id) references public.exams (exam_id)
                                           match simple on update no action on delete no action
);

create table public.question_key_answers (
                                             answer_id uuid not null default gen_random_uuid(),
                                             question_id uuid not null,
                                             choice_answer uuid,
                                             text_answer text,
                                             table_answer jsonb,
                                             primary key (answer_id, question_id),
                                             foreign key (question_id) references public.questions (question_id)
                                                 match simple on update cascade on delete cascade
);