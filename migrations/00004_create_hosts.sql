-- host profile (role-specific)
CREATE TABLE IF NOT EXISTS hosts (
    host_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    auth_user_id UUID NOT NULL REFERENCES auth_users(id) ON DELETE CASCADE,
    username TEXT,
    is_active BOOLEAN NOT NULL DEFAULT FALSE,
    location TEXT,
    metadata JSONB DEFAULT '{}'::jsonb, -- machine info, verification status
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_hosts_auth_user_id ON hosts(auth_user_id);
