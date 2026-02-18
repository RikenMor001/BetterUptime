-- Your SQL goes here
CREATE TABLE notification (
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES "user"(id),
    email TEXT NOT NULL,
    notify_down BOOLEAN NOT NULL DEFAULT true,
    notify_up BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
)