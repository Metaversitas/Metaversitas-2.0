-- Add up migration script here
INSERT INTO public.students (student_id, user_id) VALUES ('dcd3aaee-391b-4d72-9994-ba2a6409cb2c', 'de6a27a2-3cba-475a-a17e-70a512925d3d');
INSERT INTO public.students (student_id, user_id) VALUES ('2852ed68-c7e6-4655-9f1a-7baede899f06', '659074a0-22a8-47ba-9acc-6d55a9df84fb');

INSERT INTO public.student_schedule (schedule_id, student_id, start_time, end_time) VALUES ('6ad3f129-4a42-4e18-941c-e90f7e86760e', 'dcd3aaee-391b-4d72-9994-ba2a6409cb2c', '2023-10-20 21:00:00.229000 +00:00', '2023-10-20 22:00:00.388000 +00:00');

INSERT INTO public.subjects (subject_id, name) VALUES ('282f8c86-1432-4e8f-8f3c-a49bedf931dd', 'Mathematics');
INSERT INTO public.subjects (subject_id, name) VALUES ('d24a4a07-9576-48dd-ad6b-daad5fe05058', 'Physics');

INSERT INTO public.teachers (teacher_id, user_id) VALUES ('ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae', 'c3f04a30-38ac-474d-b7da-8fb889495d75');

INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('3405e8dd-5458-4e6d-a24b-007751caaf01', true, 'Test class #6', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', true, 'Test class #1', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('4b3cd71b-6e28-41d4-b620-5a9ced60f452', true, 'Test class #4', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('5b681f91-ff2a-4b39-89d9-c0ad76a7305d', true, 'Test class #8', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('349e16c4-4801-4f6a-a850-a971f30af3da', true, 'Test class #7', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('c7895bf8-a712-4e7a-81bb-8c3ba7d69072', true, 'Test class #5', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('a08f5d29-af34-490c-884f-18e73a50393f', true, 'Test class #10', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('103470f8-64be-445c-9048-8512392ae8c3', true, 'Test class #3', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', true, 'Test class #2', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);
INSERT INTO public.classes (class_id, is_active, name, description, capacity, current_meeting_id, have_multiple_meeting, semester, year_start, year_end, created_by, start_time, end_time) VALUES ('9b7aaf07-c88e-40c5-97ef-d6afd25fd767', true, 'Test class #9', 'Lorem ipsum dolor sit amet', 40, null, true, 'even', '2023-01-01', '2024-01-01', 'c3f04a30-38ac-474d-b7da-8fb889495d75', null, null);


INSERT INTO public.class_subjects (class_id, subject_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', 'd24a4a07-9576-48dd-ad6b-daad5fe05058');

INSERT INTO public.class_teachers (class_id, teacher_id) VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');
INSERT INTO public.class_teachers (class_id, teacher_id) VALUES ('9b7aaf07-c88e-40c5-97ef-d6afd25fd767', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');
INSERT INTO public.class_teachers (class_id, teacher_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');

INSERT INTO public.class_students (class_id, student_id) VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', 'dcd3aaee-391b-4d72-9994-ba2a6409cb2c');
INSERT INTO public.class_students (class_id, student_id) VALUES ('a08f5d29-af34-490c-884f-18e73a50393f', '2852ed68-c7e6-4655-9f1a-7baede899f06');
INSERT INTO public.class_students (class_id, student_id) VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', '2852ed68-c7e6-4655-9f1a-7baede899f06');

INSERT INTO public.exams (exam_id, name, description, created_by, created_at, updated_at, type) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', 'Test Exams', 'Lorem ipsum dolor sit amet assalamualaikum Jamet.', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-10-13 01:35:18.650000 +00:00', '2023-10-13 01:35:20.958000 +00:00', 'default');
INSERT INTO public.exams (exam_id, name, description, created_by, created_at, updated_at, type) VALUES ('b4bc8612-c3af-4c21-86d7-1d2a89949a6b', 'Test Exams', 'Lorem ipsum dolor sit amet assalamualaikum Jamet.', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-10-13 01:35:18.650000 +00:00', '2023-10-13 01:35:20.958000 +00:00', 'upload');

INSERT INTO public.exam_settings (exam_id, passing_score, multiple_attempts_allowed, randomize_question, time_limit) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', 70.00, false, false, 3600);

INSERT INTO public.exam_classes (exam_id, class_id) VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', '06e5c7ac-801e-457d-a576-94e34c3b12cd');

INSERT INTO public.game (version, description, installed_on, is_live, updated_at, created_at) VALUES (1, 'Alpha Version', '2023-09-21 13:09:49.033000 +00:00', true, '2023-09-21 13:09:49.033000 +00:00', '2023-09-21 13:09:49.033000 +00:00');

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

