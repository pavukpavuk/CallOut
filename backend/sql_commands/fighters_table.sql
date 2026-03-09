CREATE TYPE belt_rank AS ENUM (
'white','blue','purple','brown','black'
);

CREATE TABLE fighters (
    user_id SERIAL PRIMARY KEY,
    name TEXT DEFAULT 'No Name', 
    rank belt_rank DEFAULT 'white',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    description VARCHAR(255) DEFAULT '???',
    wins INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    draws INTEGER DEFAULT 0,
    weight_kg NUMERIC DEFAULT 0,
    picture_url TEXT DEFAULT 'default.png',
    gym_id_fk INTEGER DEFAULT NULL,


    CONSTRAINT fk_fighter_user
        FOREIGN KEY (user_id)
        REFERENCES useraccounts(id)
        ON DELETE CASCADE
);
