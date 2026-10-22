-- Stories scraped from upstream. `seq` is the device-facing cursor: it
-- only ever grows, so "seq > last seen" is "new since last sync".
-- `id` is the upstream item id.
CREATE TABLE stories (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    id INTEGER NOT NULL UNIQUE,
    title TEXT NOT NULL,
    url TEXT,
    author TEXT NOT NULL DEFAULT '',
    score INTEGER NOT NULL DEFAULT 0,
    posted_at INTEGER NOT NULL,
    fetched_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_stories_posted_at ON stories (posted_at);
