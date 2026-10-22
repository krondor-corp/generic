import { useCallback, useEffect, useState } from "react";
import { useTauriEvent } from "@mobile/std";

import {
  EVENTS,
  getStatus,
  listStories,
  openStory,
  resetData,
  saveSettings,
  syncNow,
  testConnection,
  type Status,
  type Story,
  type SyncReport,
} from "./api";

type Tab = "stories" | "settings";

export default function App() {
  const [tab, setTab] = useState<Tab>("stories");
  const [status, setStatus] = useState<Status | null>(null);
  const [stories, setStories] = useState<Story[]>([]);
  const [busy, setBusy] = useState(false);
  const [flash, setFlash] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    const [s, list] = await Promise.all([getStatus(), listStories({ limit: 100 })]);
    setStatus(s);
    setStories(list);
    if (!s.server) setTab("settings");
  }, []);

  useEffect(() => {
    refresh().catch((e) => setFlash(String(e)));
  }, [refresh]);

  // The Rust poller syncs on its own; we just redraw when it reports.
  useTauriEvent<SyncReport>(EVENTS.syncDone, (report) => {
    if (report.new_count > 0) setFlash(`${report.new_count} new`);
    refresh().catch(() => {});
  });
  useTauriEvent<string>(EVENTS.syncError, (message) => {
    setFlash(message);
    refresh().catch(() => {});
  });

  const onSync = async () => {
    setBusy(true);
    setFlash(null);
    try {
      const report = await syncNow();
      setFlash(report.new_count > 0 ? `${report.new_count} new` : "Up to date");
    } catch (e) {
      setFlash(String(e));
    } finally {
      setBusy(false);
      refresh().catch(() => {});
    }
  };

  return (
    <div className="app">
      <header>
        <h1>Top Stories</h1>
        <nav>
          <button className={tab === "stories" ? "active" : ""} onClick={() => setTab("stories")}>
            Stories
          </button>
          <button className={tab === "settings" ? "active" : ""} onClick={() => setTab("settings")}>
            Settings
          </button>
        </nav>
      </header>

      {flash && (
        <div className="flash" onClick={() => setFlash(null)}>
          {flash}
        </div>
      )}

      {tab === "stories" ? (
        <Stories stories={stories} status={status} busy={busy} onSync={onSync} />
      ) : (
        <Settings status={status} onSaved={refresh} />
      )}
    </div>
  );
}

function Stories({
  stories,
  status,
  busy,
  onSync,
}: {
  stories: Story[];
  status: Status | null;
  busy: boolean;
  onSync: () => void;
}) {
  return (
    <main>
      <div className="toolbar">
        <span className="muted">
          {status?.last_sync_at ? `Synced ${relative(status.last_sync_at)}` : "Never synced"}
          {status ? ` · ${status.story_count} stored` : ""}
        </span>
        <button onClick={onSync} disabled={busy || !status?.server}>
          {busy ? "Syncing…" : "Sync now"}
        </button>
      </div>

      {stories.length === 0 ? (
        <p className="empty">
          {status?.server
            ? "Nothing yet. Sync to pull stories from your server."
            : "Add your server in Settings to get started."}
        </p>
      ) : (
        <ul className="stories">
          {stories.map((s) => (
            <li key={s.id}>
              <button
                className="story"
                onClick={() => s.url && openStory({ url: s.url }).catch(() => {})}
                disabled={!s.url}
              >
                <span className="title">{s.title}</span>
                <span className="meta">
                  {s.score} pts · {s.by} · {hostOf(s.url)}
                </span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}

function Settings({ status, onSaved }: { status: Status | null; onSaved: () => Promise<void> }) {
  const [url, setUrl] = useState("");
  const [deviceName, setDeviceName] = useState("");
  const [result, setResult] = useState<string | null>(null);

  useEffect(() => {
    if (!status) return;
    setUrl(status.server?.url ?? "");
    setDeviceName(status.device.name);
  }, [status]);

  const onTest = async () => {
    setResult("Testing…");
    try {
      setResult(await testConnection({ url }));
    } catch (e) {
      setResult(String(e));
    }
  };

  const onSave = async () => {
    setResult(null);
    try {
      await saveSettings({ url, deviceName });
      setResult("Saved");
      await onSaved();
    } catch (e) {
      setResult(String(e));
    }
  };

  const onReset = async () => {
    try {
      await resetData();
      setResult("Local stories and cursor cleared");
      await onSaved();
    } catch (e) {
      setResult(String(e));
    }
  };

  return (
    <main>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          onSave();
        }}
      >
        <label>
          Server URL
          <input
            type="url"
            inputMode="url"
            autoCapitalize="off"
            autoCorrect="off"
            placeholder="https://ios.generic.krondor.org"
            value={url}
            onChange={(e) => setUrl(e.target.value)}
          />
        </label>
        <label>
          This device's name
          <input value={deviceName} onChange={(e) => setDeviceName(e.target.value)} />
        </label>
        <div className="row">
          <button type="button" onClick={onTest} disabled={!url}>
            Test
          </button>
          <button type="submit" disabled={!url}>
            Save
          </button>
        </div>
      </form>

      {result && <p className="result">{result}</p>}

      {status && (
        <dl className="facts">
          <dt>Device id</dt>
          <dd>
            <code>{status.device.device_id}</code>
          </dd>
          <dt>Platform</dt>
          <dd>{status.device.platform}</dd>
          <dt>Cursor</dt>
          <dd>{status.cursor ?? "—"}</dd>
          {status.last_error && (
            <>
              <dt>Last error</dt>
              <dd className="error">{status.last_error}</dd>
            </>
          )}
        </dl>
      )}

      <button type="button" className="danger" onClick={onReset}>
        Clear local data
      </button>
    </main>
  );
}

function hostOf(url: string | null): string {
  if (!url) return "text";
  try {
    return new URL(url).hostname.replace(/^www\./, "");
  } catch {
    return "";
  }
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
