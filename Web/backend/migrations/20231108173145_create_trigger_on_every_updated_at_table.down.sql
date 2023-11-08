-- Add down migration script here
DROP TRIGGER auto_update_classMeeting_modified_timestamp on class_meeting;
DROP TRIGGER auto_update_exams_modified_timestamp on exams;
DROP TRIGGER auto_update_game_modified_timestamp on game;
DROP TRIGGER auto_update_users_modified_timestamp on users;
DROP FUNCTION auto_change_updated_at_column();