-- Add admin and ban fields to users table
ALTER TABLE users ADD COLUMN is_admin INTEGER NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN is_banned INTEGER NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN banned_at TEXT;
ALTER TABLE users ADD COLUMN banned_reason TEXT;

-- Create indexes for query performance
CREATE INDEX idx_users_is_admin ON users(is_admin);
CREATE INDEX idx_users_is_banned ON users(is_banned);
