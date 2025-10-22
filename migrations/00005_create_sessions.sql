-- sessions / refresh tokens
CREATE TABLE IF NOT EXISTS sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    auth_user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL,
    user_agent TEXT,
    ip TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    expires_at TIMESTAMPTZ NOT NULL,
);

CREATE INDEX IF NOT EXISTS idx_sessions_auth_user_id ON sessions(auth_user_id);
