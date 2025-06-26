-- Add up migration script here
CREATE VIEW users_with_permissions AS
SELECT u.id,
       u.username,
       COALESCE(array_agg(p.name ORDER BY p.name) FILTER (WHERE p.name IS NOT NULL), '{}') AS permissions
FROM users u
         LEFT JOIN
     user_permissions up ON u.id = up.user_id
         LEFT JOIN
     permissions p ON up.permission_id = p.id
GROUP BY u.id, u.username;