-- Add up migration script here
CREATE OR REPLACE FUNCTION auto_change_updated_at_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER auto_update_users_modified_timestamp
    BEFORE UPDATE ON users
    FOR EACH ROW
EXECUTE PROCEDURE auto_change_updated_at_column();

CREATE TRIGGER auto_update_game_modified_timestamp
    BEFORE UPDATE ON game
    FOR EACH ROW
EXECUTE PROCEDURE  auto_change_updated_at_column();

CREATE TRIGGER auto_update_exams_modified_timestamp
    BEFORE UPDATE ON exams
    FOR EACH ROW
EXECUTE PROCEDURE auto_change_updated_at_column();

CREATE TRIGGER auto_update_classMeeting_modified_timestamp
    BEFORE UPDATE ON class_meeting
    FOR EACH ROW
EXECUTE PROCEDURE auto_change_updated_at_column();