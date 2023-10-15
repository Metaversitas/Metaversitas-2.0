CREATE TABLE public.teachers (
     teacher_id uuid DEFAULT gen_random_uuid() NOT NULL,
     name text NOT NULL,
     user_id uuid NOT NULL
);

ALTER TABLE public.teachers OWNER TO admin;

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_pk PRIMARY KEY (teacher_id);

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_pk2 UNIQUE (teacher_id);

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_users_identity_full_name_fk FOREIGN KEY (name) REFERENCES public.users_identity(full_name);

ALTER TABLE ONLY public.teachers
    ADD CONSTRAINT teachers_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);

CREATE TABLE public.subjects (
                                 subject_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                 name text NOT NULL
);

ALTER TABLE public.subjects OWNER TO admin;

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_pk PRIMARY KEY (subject_id);

ALTER TABLE ONLY public.subjects
    ADD CONSTRAINT subjects_pk2 UNIQUE (subject_id);

CREATE TABLE public.students (
                                 student_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                 name text NOT NULL,
                                 user_id uuid NOT NULL
);

ALTER TABLE public.students OWNER TO admin;

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_pk PRIMARY KEY (student_id);

ALTER TABLE ONLY public.students
    ADD CONSTRAINT students_pk2 UNIQUE (student_id);

CREATE TABLE public.classes (
                                class_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                subject_id uuid NOT NULL,
                                is_active boolean NOT NULL
);

ALTER TABLE public.classes OWNER TO admin;

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_pk PRIMARY KEY (class_id);

ALTER TABLE ONLY public.classes
    ADD CONSTRAINT classes_pk3 UNIQUE (class_id);

CREATE TABLE public.class_teachers (
                                       class_id uuid NOT NULL,
                                       teacher_id uuid NOT NULL
);

ALTER TABLE public.class_teachers OWNER TO admin;

ALTER TABLE ONLY public.class_teachers
    ADD CONSTRAINT class_teachers_pk UNIQUE (class_id);

ALTER TABLE ONLY public.class_teachers
    ADD CONSTRAINT class_teachers_pk2 UNIQUE (teacher_id);

ALTER TABLE ONLY public.class_teachers
    ADD CONSTRAINT class_teachers_classes_class_id_fk FOREIGN KEY (class_id) REFERENCES public.classes(class_id);

ALTER TABLE ONLY public.class_teachers
    ADD CONSTRAINT class_teachers_teachers_teacher_id_fk FOREIGN KEY (teacher_id) REFERENCES public.teachers(teacher_id);


CREATE TABLE public.class_students (
   class_id uuid NOT NULL,
   student_id uuid NOT NULL
);

ALTER TABLE public.class_students OWNER TO admin;
ALTER TABLE ONLY public.class_students
    ADD CONSTRAINT class_students_pk UNIQUE (student_id);
ALTER TABLE ONLY public.class_students
    ADD CONSTRAINT class_students_classes_class_id_fk FOREIGN KEY (class_id) REFERENCES public.classes(class_id);

ALTER TABLE ONLY public.class_students
    ADD CONSTRAINT class_students_students_student_id_fk FOREIGN KEY (student_id) REFERENCES public.students(student_id);


CREATE TABLE public.exams (
                              exam_id uuid DEFAULT gen_random_uuid() NOT NULL,
                              class_id uuid NOT NULL,
                              subject_id uuid NOT NULL,
                              name text NOT NULL,
                              description text,
                              created_by uuid NOT NULL,
                              created_at timestamp with time zone DEFAULT now() NOT NULL,
                              updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.exams OWNER TO admin;
ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_pk PRIMARY KEY (exam_id);

ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_pk3 UNIQUE (class_id);

ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_pk4 UNIQUE (subject_id);


ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_classes_class_id_fk FOREIGN KEY (class_id) REFERENCES public.classes(class_id);

ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_subjects_subject_id_fk FOREIGN KEY (subject_id) REFERENCES public.subjects(subject_id);

ALTER TABLE ONLY public.exams
    ADD CONSTRAINT exams_users_user_id_fk FOREIGN KEY (created_by) REFERENCES public.users(user_id);


CREATE TABLE public.questions (
                                  question_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                  question_text text NOT NULL,
                                  question_type public.question_types NOT NULL,
                                  exam_id uuid NOT NULL,
                                  question_level public.question_levels
);


ALTER TABLE public.questions OWNER TO admin;
ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_pk PRIMARY KEY (question_id);
ALTER TABLE ONLY public.questions
    ADD CONSTRAINT questions_exams_exam_id_fk FOREIGN KEY (exam_id) REFERENCES public.exams(exam_id);

CREATE TABLE public.choices (
                                choice_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                question_id uuid NOT NULL,
                                choice_text text NOT NULL,
                                is_correct boolean NOT NULL
);

ALTER TABLE public.choices OWNER TO admin;
ALTER TABLE ONLY public.choices
    ADD CONSTRAINT choices_pk PRIMARY KEY (choice_id);
ALTER TABLE ONLY public.choices
    ADD CONSTRAINT choices_questions_question_id_fk FOREIGN KEY (question_id) REFERENCES public.questions(question_id);


CREATE TABLE public.answers (
                                answer_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                user_id uuid NOT NULL,
                                question_id uuid NOT NULL,
                                choice_answer_id uuid,
                                text_answer text,
                                table_answer_id uuid
);

ALTER TABLE public.answers OWNER TO admin;
ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_pk PRIMARY KEY (answer_id);

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_answered_tables_answered_table_id_fk FOREIGN KEY (table_answer_id) REFERENCES public.answered_tables(answered_table_id);

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_choices_choice_id_fk FOREIGN KEY (choice_answer_id) REFERENCES public.choices(choice_id);

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_questions_question_id_fk FOREIGN KEY (question_id) REFERENCES public.questions(question_id);

ALTER TABLE ONLY public.answers
    ADD CONSTRAINT answers_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);

CREATE TABLE public.answered_tables (
                                        answered_table_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                        answer_id uuid DEFAULT gen_random_uuid() NOT NULL
);


ALTER TABLE public.answered_tables OWNER TO admin;

ALTER TABLE ONLY public.answered_tables
    ADD CONSTRAINT answered_tables_pk PRIMARY KEY (answered_table_id);
ALTER TABLE ONLY public.answered_tables
    ADD CONSTRAINT answered_tables_answers_answer_id_fk FOREIGN KEY (answer_id) REFERENCES public.answers(answer_id);


CREATE TABLE public.answered_tables_rows (
                                             answered_table_row_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                             answered_table_id uuid NOT NULL
);


ALTER TABLE public.answered_tables_rows OWNER TO admin;
ALTER TABLE ONLY public.answered_tables_rows
    ADD CONSTRAINT answered_tables_rows_pk PRIMARY KEY (answered_table_row_id);
ALTER TABLE ONLY public.answered_tables_rows
    ADD CONSTRAINT answered_tables_rows_answered_tables_answered_table_id_fk FOREIGN KEY (answered_table_id) REFERENCES public.answered_tables(answered_table_id);


CREATE TABLE public.answered_tables_cells (
                                              cell_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                              table_row_id uuid NOT NULL,
                                              column_name text NOT NULL,
                                              value text NOT NULL
);


ALTER TABLE public.answered_tables_cells OWNER TO admin;
ALTER TABLE ONLY public.answered_tables_cells
    ADD CONSTRAINT answered_tables_cells_pk PRIMARY KEY (cell_id);
ALTER TABLE ONLY public.answered_tables_cells
    ADD CONSTRAINT table_row_id_constraint FOREIGN KEY (table_row_id) REFERENCES public.answered_tables_rows(answered_table_row_id);


CREATE TABLE public.exams_categories (
                                         category_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                         name text NOT NULL
);


ALTER TABLE public.exams_categories OWNER TO admin;
ALTER TABLE ONLY public.exams_categories
    ADD CONSTRAINT exams_categories_pk PRIMARY KEY (category_id);

CREATE TABLE public.exams_users_score (
                                          user_id uuid NOT NULL,
                                          exam_id uuid NOT NULL,
                                          score numeric(5,2) NOT NULL
);


ALTER TABLE public.exams_users_score OWNER TO admin;
ALTER TABLE ONLY public.exams_users_score
    ADD CONSTRAINT exams_users_score_exams_exam_id_fk FOREIGN KEY (exam_id) REFERENCES public.exams(exam_id);
ALTER TABLE ONLY public.exams_users_score
    ADD CONSTRAINT exams_users_score_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);


CREATE TABLE public.exam_sessions (
                                      session_id uuid DEFAULT gen_random_uuid() NOT NULL,
                                      exam_id uuid NOT NULL,
                                      user_id uuid NOT NULL,
                                      start_time timestamp with time zone NOT NULL,
                                      end_time timestamp with time zone NOT NULL
);


ALTER TABLE public.exam_sessions OWNER TO admin;
ALTER TABLE ONLY public.exam_sessions
    ADD CONSTRAINT exam_sessions_pk PRIMARY KEY (session_id);
ALTER TABLE ONLY public.exam_sessions
    ADD CONSTRAINT exam_sessions_exams_exam_id_fk FOREIGN KEY (exam_id) REFERENCES public.exams(exam_id);
ALTER TABLE ONLY public.exam_sessions
    ADD CONSTRAINT exam_sessions_users_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(user_id);


CREATE TABLE public.exam_settings (
                                      exam_id uuid NOT NULL,
                                      passing_score numeric(5,2) NOT NULL,
                                      multiple_attempts_allowed boolean DEFAULT false NOT NULL,
                                      randomize_question boolean DEFAULT false NOT NULL,
                                      time_limit integer NOT NULL
);


ALTER TABLE public.exam_settings OWNER TO admin;
COMMENT ON COLUMN public.exam_settings.time_limit IS 'in seconds';
ALTER TABLE ONLY public.exam_settings
    ADD CONSTRAINT exam_settings_pk PRIMARY KEY (exam_id);
ALTER TABLE ONLY public.exam_settings
    ADD CONSTRAINT exam_settings_exams_exam_id_fk FOREIGN KEY (exam_id) REFERENCES public.exams(exam_id);
