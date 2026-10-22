-- Device-side mirror of the server's stories. `seq` is the server's
-- cursor value for the row, so MAX(seq) is also "where to resume from".
CREATE TABLE stories (
    seq INTEGER PRIMARY KEY NOT NULL,
    id INTEGER NOT NULL UNIQUE,
    title TEXT NOT NULL,
    url TEXT,
    author TEXT NOT NULL DEFAULT '',
    score INTEGER NOT NULL DEFAULT 0,
    posted_at INTEGER NOT NULL,
    fetched_at TEXT NOT NULL,
    synced_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
