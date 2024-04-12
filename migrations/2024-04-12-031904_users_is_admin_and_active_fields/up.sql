-- Add fields is_admin and is_active on an already existent table

ALTER TABLE users
ADD COLUMN is_admin BOOLEAN DEFAULT false NOT NULL,
ADD COLUMN is_active BOOLEAN DEFAULT true NOT NULL;
