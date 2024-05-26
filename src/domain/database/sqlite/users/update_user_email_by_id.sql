UPDATE users SET email = ?2 WHERE id = ?1 RETURNING "id", "email", "password"
