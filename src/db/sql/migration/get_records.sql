select id, query, project_hash, ts_created
FROM migrations
WHERE project_hash = $1; 