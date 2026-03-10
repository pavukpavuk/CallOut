CREATE TYPE belt_rank AS ENUM ('white', 'blue', 'purple', 'brown', 'black');

CREATE TABLE useraccounts (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    verified BOOLEAN DEFAULT false NOT NULL,
    email_nonce BYTEA NOT NULL,
    user_key BYTEA NOT NULL,
    user_key_nonce BYTEA NOT NULL,
    email BYTEA NOT NULL
);

INSERT INTO useraccounts 
(id, username, password_hash,  verified, email_nonce, user_key, user_key_nonce, email) 
VALUES (
    1, 
    'pavuk', 
    '$argon2id$v=19$m=19456,t=2,p=1$iYEyZ2A7qtKy8TbPDF6B3Q$3NBbdl+q8APkOoOwDlot5MQiVtLNaxarDCmMwMGlIyU', 
    't', 
    '\x477090d3d0ca297a8b1de17b', 
    '\xcaacb1e94c241b4d3af7af33e20874f08204fe7b6a0e846014ba8c6852ecdf4a4e07cdcc0bc2591643f67fa3aad75e4d',
    '\x4e10fd6a2d32715048b91d90',
    '\x64cb3a534f70714e7fafa0740c6fb400da03c71bd783d57c9a27918213e118a63b56af96'
    );

CREATE TABLE fighters (
    user_id SERIAL PRIMARY KEY REFERENCES useraccounts(id) ON DELETE CASCADE,
    name TEXT DEFAULT 'No Name' NOT NULL,
    rank TEXT DEFAULT 'white' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
    description VARCHAR(255) DEFAULT '???',
    wins INTEGER DEFAULT 0 NOT NULL,
    losses INTEGER DEFAULT 0 NOT NULL,
    draws INTEGER DEFAULT 0 NOT NULL,
    weight_kg INTEGER DEFAULT 0 NOT NULL,
    picture_url TEXT DEFAULT 'default.png',
    gym_id_fk INTEGER
);

CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    session_id BYTEA NOT NULL,
    user_id INTEGER NOT NULL REFERENCES useraccounts(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT now(),
    expires_at TIMESTAMPTZ DEFAULT (now() + INTERVAL '30 minutes') NOT NULL,
    last_seen_at TIMESTAMPTZ DEFAULT now()
);


