-- Referral system
ALTER TABLE users ADD COLUMN referral_code TEXT UNIQUE;
ALTER TABLE users ADD COLUMN referred_by INTEGER REFERENCES users(id);
ALTER TABLE users ADD COLUMN referral_count INTEGER DEFAULT 0;
ALTER TABLE users ADD COLUMN bonus_days INTEGER DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_users_referral_code ON users(referral_code);
