INSERT INTO migrations (query, project_hash)
VALUES ($1, $2)
RETURNING id;