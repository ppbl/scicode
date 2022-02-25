-- Your SQL goes here
ALTER TABLE users ALTER password DROP NOT NULL;
ALTER TABLE users ADD github_id BIGINT;
ALTER TABLE users ADD github_url VARCHAR;
ALTER TABLE users ADD avatar_url VARCHAR;
ALTER TABLE users ADD CONSTRAINT unique_users UNIQUE(github_id);