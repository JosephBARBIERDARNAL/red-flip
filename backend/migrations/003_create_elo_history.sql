CREATE TABLE IF NOT EXISTS elo_history (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users(id),
    match_id TEXT NOT NULL REFERENCES matches(id),
    elo_before INTEGER NOT NULL,
    elo_after INTEGER NOT NULL,
    elo_change INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
