-- Add up migration script here
create table public.student_exam_upload (
                                            exam_id uuid not null,
                                            student_id uuid not null,
                                            file_url text not null,
                                            uploaded_at timestamp with time zone not null default now(),
                                            primary key (exam_id, student_id),
                                            foreign key (exam_id) references public.exams (exam_id)
                                                match simple on update cascade on delete cascade,
                                            foreign key (student_id) references public.students (student_id)
                                                match simple on update cascade on delete cascade
);

CREATE FUNCTION check_exam_type() RETURNS TRIGGER AS
$$
BEGIN
    IF (SELECT type FROM exams WHERE exam_id = NEW.exam_id) != 'upload' THEN
        RAISE EXCEPTION 'Invalid exam type';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER check_exam_type
    BEFORE INSERT OR UPDATE ON student_exam_upload
    FOR EACH ROW EXECUTE PROCEDURE check_exam_type();