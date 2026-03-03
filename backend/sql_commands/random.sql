-- UPDATE useraccounts
-- SET email_nonce_bin = decode(email_nonce, 'hex'),
-- user_key_bin = decode(user_key, 'hex'),
-- user_key_nonce_bin = decode(user_key_nonce, 'hex'),
-- email_bin = decode(email, 'hex');


ALTER TABLE useraccounts
ALTER COLUMN email SET NOT NULL,
ALTER COLUMN user_key SET NOT NULL,
ALTER COLUMN email_nonce SET NOT NULL,
ALTER COLUMN user_key_nonce SET NOT NULL;