-- Add up migration script here
INSERT INTO public.teachers VALUES ('ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae', 'Nama Lengkap Dosen', 'c3f04a30-38ac-474d-b7da-8fb889495d75');

INSERT INTO public.subjects VALUES ('d24a4a07-9576-48dd-ad6b-daad5fe05058', 'Physics');
INSERT INTO public.subjects VALUES ('282f8c86-1432-4e8f-8f3c-a49bedf931dd', 'Mathematics');

INSERT INTO public.students VALUES ('dcd3aaee-391b-4d72-9994-ba2a6409cb2c', 'Nama 2 Mahasiswa', 'de6a27a2-3cba-475a-a17e-70a512925d3d');
INSERT INTO public.students VALUES ('2852ed68-c7e6-4655-9f1a-7baede899f06', 'Nama Lengkap Mahasiswa', '659074a0-22a8-47ba-9acc-6d55a9df84fb');

INSERT INTO public.questions VALUES ('68102cd3-e903-4b34-8de8-68192ce8939d', 'What is 2+2 is?', 'choice', '60fff264-2e9e-476e-9d05-10e38d1c8bca', NULL);
INSERT INTO public.questions VALUES ('dfaad579-bcb9-4217-a231-1c00961f97cc', 'What is the square root of 16?', 'choice', '60fff264-2e9e-476e-9d05-10e38d1c8bca', NULL);

INSERT INTO public.game VALUES (1, 'Alpha Version', '2023-09-21 13:09:49.033+00', true, '2023-09-21 13:09:49.033+00', '2023-09-21 13:09:49.033+00');

INSERT INTO public.exams_categories VALUES ('3aeac3de-891c-4796-b8de-09c6d19ad137', 'Mathematics');
INSERT INTO public.exams_categories VALUES ('e24ea3d1-f246-48cb-a0cc-003dbccfcfd2', 'Physics');

INSERT INTO public.exams VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', '06e5c7ac-801e-457d-a576-94e34c3b12cd', '282f8c86-1432-4e8f-8f3c-a49bedf931dd', 'Test Exams', 'Lorem ipsum dolor sit amet assalamualaikum Jamet.', 'c3f04a30-38ac-474d-b7da-8fb889495d75', '2023-10-13 01:35:18.65+00', '2023-10-13 01:35:20.958+00');

INSERT INTO public.exam_settings VALUES ('60fff264-2e9e-476e-9d05-10e38d1c8bca', 70.00, false, false, 3600);

INSERT INTO public.classes VALUES ('06e5c7ac-801e-457d-a576-94e34c3b12cd', '282f8c86-1432-4e8f-8f3c-a49bedf931dd', true);
INSERT INTO public.classes VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', true);
INSERT INTO public.classes VALUES ('103470f8-64be-445c-9048-8512392ae8c3', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', true);
INSERT INTO public.classes VALUES ('4b3cd71b-6e28-41d4-b620-5a9ced60f452', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', true);
INSERT INTO public.classes VALUES ('c7895bf8-a712-4e7a-81bb-8c3ba7d69072', 'd24a4a07-9576-48dd-ad6b-daad5fe05058', true);
INSERT INTO public.classes VALUES ('3405e8dd-5458-4e6d-a24b-007751caaf01', '282f8c86-1432-4e8f-8f3c-a49bedf931dd', true);

INSERT INTO public.class_teachers VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', 'ac0b6326-0ee3-4acb-bc17-a2f43a5e74ae');

INSERT INTO public.class_students VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', 'dcd3aaee-391b-4d72-9994-ba2a6409cb2c');
INSERT INTO public.class_students VALUES ('5b099ac5-5b17-4614-b7dd-4dfb26734eec', '2852ed68-c7e6-4655-9f1a-7baede899f06');

INSERT INTO public.choices VALUES ('e916a0ed-2e4f-4ee7-a27f-25847a5f3eb4', '68102cd3-e903-4b34-8de8-68192ce8939d', '2', false);
INSERT INTO public.choices VALUES ('e5622c74-8780-42f3-8309-72562dff6a4f', '68102cd3-e903-4b34-8de8-68192ce8939d', '3', false);
INSERT INTO public.choices VALUES ('76c778ed-d24b-4f52-a514-aa5203e410cd', '68102cd3-e903-4b34-8de8-68192ce8939d', '4', true);
INSERT INTO public.choices VALUES ('50444288-829d-47ff-a120-5e864e77d9c6', '68102cd3-e903-4b34-8de8-68192ce8939d', '5', false);
