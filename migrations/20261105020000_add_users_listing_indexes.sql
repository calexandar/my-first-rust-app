CREATE INDEX users_created_at_idx ON users (created_at DESC, id DESC);

CREATE INDEX users_is_admin_created_at_idx ON users (is_admin, created_at DESC, id DESC);
