SELECT *
FROM intern_merchandise
ORDER BY created_at ASC
LIMIT $1
OFFSET $2;