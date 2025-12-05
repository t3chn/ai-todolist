-- Add subscription fields to users
ALTER TABLE users ADD COLUMN trial_ends_at TEXT;
ALTER TABLE users ADD COLUMN subscription_expires_at TEXT;
ALTER TABLE users ADD COLUMN subscription_type TEXT DEFAULT 'trial'; -- trial, monthly, yearly

-- Set trial end date for existing users (7 days from now)
UPDATE users SET trial_ends_at = datetime('now', '+7 days') WHERE trial_ends_at IS NULL;
