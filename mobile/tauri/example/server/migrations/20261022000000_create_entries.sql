-- The one table. `seq` is the monotonic cursor devices page on.
CREATE TABLE IF NOT EXISTS entries (
    seq        INTEGER PRIMARY KEY AUTOINCREMENT,
    body       TEXT    NOT NULL,
    created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
