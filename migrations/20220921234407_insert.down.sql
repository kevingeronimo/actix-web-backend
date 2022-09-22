-- Add down migration script here
DELETE FROM users WHERE username IN ('kev1', 'kev2');