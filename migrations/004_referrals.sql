-- Referral system
-- Note: SQLite doesn't support ADD COLUMN ... UNIQUE directly
-- Using partial unique index instead (allows multiple NULLs)
ALTER TABLE users ADD COLUMN referral_code TEXT;
ALTER TABLE users ADD COLUMN referred_by INTEGER REFERENCES users(id);
ALTER TABLE users ADD COLUMN referral_count INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN bonus_days INTEGER DEFAULT 0;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_referral_code ON users(referral_code) WHERE referral_code IS NOT NULL;
