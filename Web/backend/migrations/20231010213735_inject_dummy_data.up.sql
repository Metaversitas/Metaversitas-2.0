
--
-- Data for Name: blockchain_authentication; Type: TABLE DATA; Schema: public; Owner: admin
--



--
-- Data for Name: game; Type: TABLE DATA; Schema: public; Owner: admin
--



--
-- Data for Name: university; Type: TABLE DATA; Schema: public; Owner: admin
--

INSERT INTO public.university VALUES (1, 'Universitas Gunadarma');


--
-- Data for Name: university_faculty; Type: TABLE DATA; Schema: public; Owner: admin
--

INSERT INTO public.university_faculty VALUES (40000, 1, 'Sistem Informasi');


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: admin
--

INSERT INTO public.users VALUES ('de6a27a2-3cba-475a-a17e-70a512925d3d', 'mahasiswa2@dikti.go.id', '$argon2id$v=19$m=19456,t=2,p=1$h0CM9qGfJDs5KhagNdn5fQ$UaoLAHE2a/28FjlKacFk4Ll5aA0p45jSVXSgmVbT7vQ', 'IniMahasiswa2', false, '2023-09-18 03:16:55.667156+00', 'user', '2023-09-18 03:16:55.667156+00');
INSERT INTO public.users VALUES ('c3f04a30-38ac-474d-b7da-8fb889495d75', 'dosen@dikti.go.id', '$argon2id$v=19$m=19456,t=2,p=1$S0+9eG3WoW8BXYYyBmUdJA$LhqA6r3EdfX/aEg5qEN24+ofkjdl43dF93+wCqtn0Qk', 'IniDosen', false, '2023-09-18 03:16:45.424017+00', 'user', '2023-09-18 03:16:45.424017+00');
INSERT INTO public.users VALUES ('659074a0-22a8-47ba-9acc-6d55a9df84fb', 'mahasiswa@dikti.go.id', '$argon2id$v=19$m=19456,t=2,p=1$cKV3nHlHNpUiv/OsyIQT6A$XjdcDv1jWq3IJBVX1gv5UfOruXr0b0jVCCyoO6orHHQ', 'IniMahasiswa', false, '2023-09-18 03:16:35.539074+00', 'user', '2023-09-18 03:16:35.539074+00');
INSERT INTO public.users VALUES ('bd390205-c344-449c-8493-d8f6021cbe71', 'mahasiswi@dikti.go.id', '$argon2id$v=19$m=19456,t=2,p=1$S0+9eG3WoW8BXYYyBmUdJA$LhqA6r3EdfX/aEg5qEN24+ofkjdl43dF93+wCqtn0Qk', 'IniMahasiswaPerempuan', false, '2023-10-05 06:13:28.572121+00', 'user', '2023-10-05 06:13:28.572121+00');
INSERT INTO public.users VALUES ('ad05b191-7d8e-4fba-8009-d7615bdab6bd', 'dosen_perempuan@dikti.go.id', '$argon2id$v=19$m=19456,t=2,p=1$S0+9eG3WoW8BXYYyBmUdJA$LhqA6r3EdfX/aEg5qEN24+ofkjdl43dF93+wCqtn0Qk', 'IniDosenPerempuan', false, '2023-10-05 06:13:13.603895+00', 'user', '2023-10-05 06:13:13.603895+00');


--
-- Data for Name: users_external_authentication; Type: TABLE DATA; Schema: public; Owner: admin
--



--
-- Data for Name: users_identity; Type: TABLE DATA; Schema: public; Owner: admin
--

INSERT INTO public.users_identity VALUES (2, 'c3f04a30-38ac-474d-b7da-8fb889495d75', 'Nama Lengkap Dosen', 'male');
INSERT INTO public.users_identity VALUES (0, '659074a0-22a8-47ba-9acc-6d55a9df84fb', 'Nama Lengkap Mahasiswa', 'male');
INSERT INTO public.users_identity VALUES (4, 'bd390205-c344-449c-8493-d8f6021cbe71', 'Nama Lengkap Mahasiswi', 'female');
INSERT INTO public.users_identity VALUES (3, 'ad05b191-7d8e-4fba-8009-d7615bdab6bd', 'Nama Lengkap Dosen Cewe', 'female');
INSERT INTO public.users_identity VALUES (1, 'de6a27a2-3cba-475a-a17e-70a512925d3d', 'Nama 2 Mahasiswa', 'male');


--
-- Data for Name: users_university_identity; Type: TABLE DATA; Schema: public; Owner: admin
--

INSERT INTO public.users_university_identity VALUES (111200031, '659074a0-22a8-47ba-9acc-6d55a9df84fb', 0, 'mahasiswa', 1, 40000);
INSERT INTO public.users_university_identity VALUES (111200032, 'de6a27a2-3cba-475a-a17e-70a512925d3d', 1, 'mahasiswa', 1, 40000);
INSERT INTO public.users_university_identity VALUES (111120033, 'c3f04a30-38ac-474d-b7da-8fb889495d75', 2, 'dosen', 1, 40000);
INSERT INTO public.users_university_identity VALUES (111120034, 'ad05b191-7d8e-4fba-8009-d7615bdab6bd', 3, 'dosen', 1, 40000);
INSERT INTO public.users_university_identity VALUES (111120035, 'bd390205-c344-449c-8493-d8f6021cbe71', 4, 'mahasiswa',1, 40000);

INSERT INTO public.game (version, description, installed_on, is_live, updated_at, created_at) VALUES (1, 'Alpha Version', '2023-09-21 13:09:49.033000 +00:00', true, '2023-09-21 13:09:49.033000 +00:00', '2023-09-21 13:09:49.033000 +00:00');
