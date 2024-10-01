-- Add up migration script here
CREATE TABLE IF NOT EXISTS meetups(
    meetup_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title TEXT NOT NULL,
    body_text TEXT NOT NULL
);