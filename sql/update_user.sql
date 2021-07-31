UPDATE users
    SET firstname = $2, lastname = $3
WHERE id = $1
RETURNING *