CREATE TABLE sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    token TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_sessions_on_token ON sessions (token);

CREATE INDEX index_sessions_on_user_id ON sessions (user_id);
