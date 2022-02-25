-- This file should undo anything in `up.sql`
ALTER TABLE users DROP CONSTRAINT unique_users;
ALTER TABLE users DROP COLUMN github_id;
ALTER TABLE users DROP COLUMN github_url;
ALTER TABLE users DROP COLUMN avatar_url;
ALTER TABLE users ALTER password SET NOT NULL;