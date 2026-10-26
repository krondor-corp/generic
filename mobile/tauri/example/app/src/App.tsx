import { useCallback, useEffect, useState } from "react";
import { useTauriEvent } from "@mobile/std";

import {
  EVENTS,
  getStatus,
  listEntries,
  syncNow,
  type Entry,
  type Status,
  type SyncReport,
} from "./api";

export default function App() {
  const [status, setStatus] = useState<Status | null>(null);
  const [entries, setEntries] = useState<Entry[]>([]);
  const [busy, setBusy] = useState(false);
  // One line of feedback, rendered inside the status line so showing or
  // changing it never shifts the layout.
  const [note, setNote] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    const [s, list] = await Promise.all([getStatus(), listEntries({ limit: 100 })]);
    setStatus(s);
    setEntries(list);
  }, []);

  useEffect(() => {
    refresh().catch((e) => setNote(String(e)));
  }, [refresh]);

  // The Rust poller syncs on its own; we just redraw when it reports.
  useTauriEvent<SyncReport>(EVENTS.syncDone, (report) => {
    if (report.new_count > 0) setNote(`${report.new_count} new`);
    refresh().catch(() => {});
  });
  useTauriEvent<string>(EVENTS.syncError, (message) => {
    setNote(message);
    refresh().catch(() => {});
  });

  const onSync = async () => {
    setBusy(true);
    try {
      const report = await syncNow();
      setNote(report.new_count > 0 ? `${report.new_count} new` : "Up to date");
    } catch (e) {
      setNote(String(e));
    } finally {
      setBusy(false);
      refresh().catch(() => {});
    }
  };

  const summary = status
    ? `${status.last_sync_at ? `Synced ${relative(status.last_sync_at)}` : "Never synced"} · ${status.entry_count} rows`
    : "";

  return (
    <div className="app">
      <header>
        <div className="brand">
          <img src="/icon.svg" alt="" />
          <h1>generic</h1>
        </div>
      </header>

      <main>
        <p className="about">A server writes a row every few minutes. This app mirrors it.</p>

        <div className="toolbar">
          <span className="muted">
            {summary}
            {note ? ` · ${note}` : ""}
          </span>
          <button onClick={onSync} disabled={busy}>
            {busy ? "Syncing…" : "Sync now"}
          </button>
        </div>

        {entries.length === 0 ? (
          <p className="empty">
            {status?.last_error
              ? status.last_error
              : "Nothing yet. The server writes a row on a schedule; sync to pull it."}
          </p>
        ) : (
          <ul className="entries">
            {entries.map((e) => (
              <li key={e.seq}>
                <span className="body">{e.body}</span>
                <span className="meta">
                  #{e.seq} · {relative(e.created_at)}
                </span>
              </li>
            ))}
          </ul>
        )}
      </main>

      <footer className="muted">{status?.server_url ?? ""}</footer>
    </div>
  );
}

function relative(iso: string): string {
  const seconds = Math.max(0, Math.round((Date.now() - Date.parse(iso)) / 1000));
  if (seconds < 60) return "just now";
  const minutes = Math.round(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.round(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  return `${Math.round(hours / 24)}d ago`;
}
