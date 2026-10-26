-- Device-side mirror of the server's `entries` table. `seq` is the
-- server's value (not AUTOINCREMENT here) so INSERT OR IGNORE dedupes.
CREATE TABLE IF NOT EXISTS entries (
    seq        INTEGER PRIMARY KEY,
    body       TEXT    NOT NULL,
    created_at TEXT    NOT NULL,
    synced_at  TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
