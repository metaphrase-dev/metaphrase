CREATE TABLE translations (
  id INTEGER PRIMARY KEY NOT NULL,
  key TEXT NOT NULL,
  locale TEXT NOT NULL,
  content TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ')),
  deleted_at TEXT DEFAULT NULL,
  user_id INTEGER DEFAULT NULL,
  validator_id INTEGER DEFAULT NULL,
  validated_at TEXT DEFAULT NULL,
  FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE RESTRICT,
  FOREIGN KEY(validator_id) REFERENCES user(id) ON DELETE RESTRICT
);

CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  email TEXT NOT NULL,
  hashed_password TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ'))
);

CREATE UNIQUE INDEX index_users_on_email ON users (email);

CREATE TABLE sessions (
    id INTEGER PRIMARY KEY NOT NULL,
    token TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ')),
    expired_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_sessions_on_token ON sessions (token);

CREATE INDEX index_sessions_on_user_id ON sessions (user_id);

CREATE TABLE settings (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ'))
);

CREATE INDEX index_settings_on_key ON settings (key);
