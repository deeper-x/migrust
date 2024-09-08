UPDATE migrations
SET query = $1
WHERE id = $2
AND project_hash = $3
RETURNING id;