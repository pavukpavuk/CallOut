INSERT INTO useraccounts 
(username, email, password_hash, email_nonce, user_key, user_key_nonce) 
VALUES ($1, $2, $3, $4, $5, $6);