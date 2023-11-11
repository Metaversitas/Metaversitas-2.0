-- Add up migration script here
INSERT INTO public.students (student_id, user_id) VALUES ('dcd3aaee-391b-4d72-9994-ba2a6409cb2c', 'de6a27a2-3cba-475a-a17e-70a512925d3d');
INSERT INTO public.students (student_id, user_id) VALUES ('2852ed68-c7e6-4655-9f1a-7baede899f06', '659074a0-22a8-47ba-9acc-6d55a9df84fb');

INSERT INTO public.student_schedule (schedule_id, student_id, start_time, end_time) VALUES ('6ad3f129-4a42-4e18-941c-e90f7e86760e', 'dcd3aaee-391b-4d72-9994-ba2a6409cb2c', '2023-10-20 21:00:00.229000 +00:00', '2023-10-20 22:00:00.388000 +00:00');

INSERT INTO public.subjects (subject_id, name) VALUES ('282f8c86-1432-4e8f-8f3c-a49bedf931dd', 'Mathematics');
INSERT INTO public.subjects (subject_id, name) VALUES ('d24a4a07-9576-48dd-ad6b-daad5fe05058', 'Physics');

INSERT INTO public.subject_secondary (secondary_subject_id, subject_id, name) VALUES ('a4e0510e-19f2-4c6b-81a6-5ac5c209b207', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', 'Ayunan Matematis');
INSERT INTO public.subject_secondary (secondary_subject_id, subject_id, name) VALUES ('385e658f-61df-4ddd-af1c-e7c3c5355d00', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', 'Archimedes');


INSERT INTO public.teachers (teacher_id, user_id) VALUES ('ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae', 'c3f04a30-38ac-474d-b7da-8fb889495d75');

INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', true, 'Test class #1', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('8292662f-4504-4472-a5c3-fa7d92107728', true, 'Lorem ipsum', null, 50, null, false, 'odd', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-11-04 02:17:22.683000 +00:00', '2023-11-05 02:17:28.686000 +00:00');

INSERT INTO public.class_meeting (meeting_id, class_id, name, description, start_time, end_time, topic_description, created_at, updated_at, meeting_number, is_active) VALUES ('5cdf362b-cb60-4e95-bed8-0142553ef58b', '06e5c7ac-801e-457d-a576-94e34c3b12cd', 'a', null, '2023-11-04 02:17:22.683000 +00:00', '2023-11-05 02:17:28.686000 +00:00', 'asd', '2023-10-30 06:48:44.117519 +00:00', '2023-10-30 06:48:44.117519 +00:00', 1, true);
INSERT INTO public.class_meeting (meeting_id, class_id, name, description, start_time, end_time, topic_description, created_at, updated_at, meeting_number, is_active) VALUES ('7279ef51-dc37-4da4-8b36-87b390da126e', '06e5c7ac-801e-457d-a576-94e34c3b12cd', 'b', 'asdsad', '2023-11-06 15:58:51.463000 +00:00', '2023-11-07 15:58:57.772000 +00:00', 'asdasdasd', '2023-10-30 06:49:11.196433 +00:00', '2023-10-30 06:49:11.196433 +00:00', 2, true);

UPDATE public.classes set current_meeting_id = '5cdf362b-cb60-4e95-bed8-0142553ef58b' where class_id = '06e5c7ac-801e-457d-a576-94e34c3b12cd';

INSERT INTO public.class_subjects (class_id, subject_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', 'd24a4a07-9576-48dd-ad6b-daad5fe05058');

INSERT INTO public.class_teachers (class_id, teacher_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');
INSERT INTO public.class_teachers (class_id, teacher_id) VALUES ('8292662f-4504-4472-a5c3-fa7d92107728', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');

INSERT INTO public.class_students (class_id, student_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', '2852ed68-c7e6-4655-9f1a-7baede899f06');

INSERT INTO public.exams (exam_id, name, description, created_by, created_at, updated_at, type) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', 'Test Exams', 'Lorem ipsum dolor sit amet assalamualaikum Jamet.', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-10-13 01:35:18.650000 +00:00', '2023-10-13 01:35:20.958000 +00:00', 'default');
INSERT INTO public.exams (exam_id, name, description, created_by, created_at, updated_at, type) VALUES ('b4bc8612-c3af-4c21-86d7-1d2a89949a6b', 'Test Exams', 'Lorem ipsum dolor sit amet assalamualaikum Jamet.', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-10-13 01:35:18.650000 +00:00', '2023-10-13 01:35:20.958000 +00:00', 'upload');

INSERT INTO public.exam_settings (exam_id, passing_score, multiple_attempts_allowed, randomize_question, time_limit) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', 70.00, false, false, 3600);

INSERT INTO public.exam_classes (exam_id, class_id) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', '06e5c7ac-801e-457d-a576-94e34c3b12cd');

INSERT INTO public.questions (question_id, question_text, question_type, table_question) VALUES ('68102cd3-e903-4b34-8de8-68192ce8939d', 'What is 2+2 is?', 'choice', null);
INSERT INTO public.questions (question_id, question_text, question_type, table_question) VALUES ('dfaad579-bcb9-4217-a231-1c00961f97cc', 'What is the square root of 16?', 'choice', null);
INSERT INTO public.questions (question_id, question_text, question_type, table_question) VALUES ('cbe238f0-12b9-4cf6-aded-e50a57c98ae0', 'Test??????', 'choice', null);

INSERT INTO public.question_key_answers (answer_id, question_id, choice_answer, text_answer, table_answer) VALUES ('0195b566-2685-4835-aabc-21fcb537ca5b', '68102cd3-e903-4b34-8de8-68192ce8939d', '76c778ed-d24b-4f52-a514-aa5203e410cd', null, null);
INSERT INTO public.question_key_answers (answer_id, question_id, choice_answer, text_answer, table_answer) VALUES ('24fc5091-be09-458f-80df-70e7d130694f', 'dfaad579-bcb9-4217-a231-1c00961f97cc', '204d57b4-4e3d-4147-9be4-0e1afb5afa1f', null, null);
INSERT INTO public.question_key_answers (answer_id, question_id, choice_answer, text_answer, table_answer) VALUES ('aea44a14-3d8e-4455-b6f7-781e39383a51', 'cbe238f0-12b9-4cf6-aded-e50a57c98ae0', 'e2f11fc9-28a6-49ab-8c53-c8b6a3b71a3c', null, null);

INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('e916a0ed-2e4f-4ee7-a27f-25847a5f3eb4', '68102cd3-e903-4b34-8de8-68192ce8939d', '2', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('e5622c74-8780-42f3-8309-72562dff6a4f', '68102cd3-e903-4b34-8de8-68192ce8939d', '3', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('76c778ed-d24b-4f52-a514-aa5203e410cd', '68102cd3-e903-4b34-8de8-68192ce8939d', '4', true);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('50444288-829d-47ff-a120-5e864e77d9c6', '68102cd3-e903-4b34-8de8-68192ce8939d', '5', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('204d57b4-4e3d-4147-9be4-0e1afb5afa1f', 'dfaad579-bcb9-4217-a231-1c00961f97cc', '4', true);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('907a4a17-7d69-47df-a32d-43ff55865e42', 'dfaad579-bcb9-4217-a231-1c00961f97cc', '1', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('2f19af60-cdc8-4dba-9de3-24a10a85e984', 'dfaad579-bcb9-4217-a231-1c00961f97cc', '2', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('19609867-bc63-4db5-a4b5-11bc0a3a1bb8', 'dfaad579-bcb9-4217-a231-1c00961f97cc', '3', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('85e600f4-c2c5-439f-905a-7134362c5191', 'cbe238f0-12b9-4cf6-aded-e50a57c98ae0', 'asdasd #2', false);
INSERT INTO public.question_choices (choice_id, question_id, choice_text, is_correct) VALUES ('e2f11fc9-28a6-49ab-8c53-c8b6a3b71a3c', 'cbe238f0-12b9-4cf6-aded-e50a57c98ae0', 'asdasd #1', true);

