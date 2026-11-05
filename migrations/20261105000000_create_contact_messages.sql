CREATE TABLE contact_messages (
    id BIGSERIAL PRIMARY KEY,
    session_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX contact_messages_session_created_idx
    ON contact_messages (session_id, created_at DESC, id DESC);
