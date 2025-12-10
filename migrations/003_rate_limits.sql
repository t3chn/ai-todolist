-- Rate limits table
CREATE TABLE IF NOT EXISTS rate_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    action_type TEXT NOT NULL,  -- 'task', 'voice', 'ai_call'
    count INTEGER DEFAULT 0,
    window_start TEXT NOT NULL,  -- datetime
    UNIQUE(user_id, action_type, window_start)
);

CREATE INDEX IF NOT EXISTS idx_rate_limits_user ON rate_limits(user_id, action_type);
