CREATE TABLE user_refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_hash TEXT NOT NULL UNIQUE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMPTZ NOT NULL,
    replace_by_token_id UUID REFERENCES user_refresh_tokens(id) ON DELETE SET NULL
);

CREATE INDEX idx_user_refresh_tokens_user_id
    ON user_refresh_tokens(user_id);

CREATE INDEX idx_user_refresh_tokens_expires_at
    ON user_refresh_tokens(expires_at);