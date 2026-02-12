CREATE TABLE IF NOT EXISTS matches (
    id TEXT PRIMARY KEY NOT NULL,
    player1_id TEXT NOT NULL REFERENCES users(id),
    player2_id TEXT NOT NULL REFERENCES users(id),
    winner_id TEXT REFERENCES users(id),
    is_ranked INTEGER NOT NULL DEFAULT 1,
    player1_score INTEGER NOT NULL DEFAULT 0,
    player2_score INTEGER NOT NULL DEFAULT 0,
    rounds_json TEXT NOT NULL DEFAULT '[]',
    player1_elo_before INTEGER,
    player1_elo_after INTEGER,
    player2_elo_before INTEGER,
    player2_elo_after INTEGER,
    status TEXT NOT NULL DEFAULT 'in_progress',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    finished_at TEXT
);
