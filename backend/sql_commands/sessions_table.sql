CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    session_id BYTEA NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '30 minutes',
    last_seen_at TIMESTAMPTZ DEFAULT NOW(),

    CONSTRAINT fk_sessions_user
        FOREIGN KEY (user_id)
        REFERENCES useraccounts(id)
        ON DELETE CASCADE
);