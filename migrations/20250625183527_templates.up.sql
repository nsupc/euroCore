-- Add up migration script here
CREATE TABLE templates
(
    id          UUID        NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    nation      TEXT        NOT NULL,
    tgid        INTEGER     NOT NULL,
    key         TEXT        NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL             DEFAULT current_timestamp,
    modified_at TIMESTAMPTZ NOT NULL             DEFAULT current_timestamp
);