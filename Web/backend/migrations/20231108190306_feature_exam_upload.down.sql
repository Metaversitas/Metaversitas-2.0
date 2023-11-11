-- Add down migration script here
drop table student_exam_upload;
drop trigger check_exam_type on public.student_exam_upload;
drop function check_exam_type();
