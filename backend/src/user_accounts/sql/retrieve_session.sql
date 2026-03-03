SELECT * FROM sessions s
WHERE s.session_id = $1
    AND s.expires_at > NOW()

