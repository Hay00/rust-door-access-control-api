-- Undo added fields is_admin and is_active

ALTER TABLE users
DROP COLUMN is_admin,
DROP COLUMN is_active;
