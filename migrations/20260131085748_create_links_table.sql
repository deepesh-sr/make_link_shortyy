-- Add migration script here
CREATE TABLE links(
    id BIGSERIAL PRIMARY KEY,
    short_code VARCHAR(10) UNIQUE NOT NULL, 
    original_url TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    click_count INTEGER DEFAULT 0
);

CREATE INDEX idx_short_code ON links(short_code);