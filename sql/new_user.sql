INSERT INTO users (
    id,
    email,
    password_hash,
    created_at,
    invitation_pending,
    firstname,
    lastname,
    updated_at,
    deleted
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9
)
RETURNING *;