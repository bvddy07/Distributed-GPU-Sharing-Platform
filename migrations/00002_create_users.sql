-- users table
CREATE TABLE IF NOT EXISTS auth_users (
    auth_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    auth_email CITEXT UNIQUE NOT NULL,
    auth_password_hash TEXT NOT NULL,
    username TEXT,
    role NOT NULL CHECK (role IN ('host', 'client'))
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- index on email (citext unique already enforces), keep created_at for queries
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);
