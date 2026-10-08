-- SQL schema for Yukthi File Storage (YFS)

-- Users
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    domain VARCHAR(255) NOT NULL,
    -- TODO: CalDav authentication integration
    -- caldav_app_password_hash TEXT,

    organization_id UUID NOT NULL,
    organization_name VARCHAR(250) NOT NULL,

    private_info JSONB NOT NULL,    -- Available only to the user themselves
    public_info JSONB NOT NULL,     -- Available to all within the organization

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);


-- User Session management
CREATE TABLE sessions (
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,

    refresh_token UUID PRIMARY KEY,

    sso_token VARCHAR(50) NOT NULL,
    fcm_token TEXT NULL,

    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + INTERVAL '3 days')
);



-- Indexes for performance optimization
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_sso_token ON sessions(sso_token);
