INSERT INTO fighters (
    user_id, 
    weight_kg, 
    name, 
    picture_url, 
    description, 
    rank, 
    wins, 
    losses, 
    draws, 
    created_at, 
    gym_id_fk
)
VALUES 
-- (2, 1, 'Johnny Jimson', 'johnny_jimson_profile_img.jpg', 'I am a shark and the ground is my ocean', 'blue', 1, 5, 20, NOW(), 2),

(
    1,
    1,
    'Johnny Jimson',
    'johnny_jimson_profile_img.jpg',
    'I am a shark and the ground is my ocean',
    'blue',
    1,
    5,
    20,
    NOW(),
    2
)