UPDATE users SET password = $2 WHERE id = $1 RETURNING "id", "email", "password"
