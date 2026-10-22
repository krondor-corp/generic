// Every command the Rust side exposes, named once. Shapes mirror the
// serde structs in src-tauri/src/commands.

import { command } from "@mobile/std";

export interface Entry {
  seq: number;
  body: string;
  created_at: string;
}

export interface Status {
  server_url: string | null;
  cursor: number | null;
  entry_count: number;
  last_sync_at: string | null;
  last_error: string | null;
}

export interface SyncReport {
  fetched: number;
  new_count: number;
  cursor: number;
}

export const getStatus = command<void, Status>("get_status");
export const listEntries = command<{ limit: number }, Entry[]>("list_entries");
export const syncNow = command<void, SyncReport>("sync_now");

export const EVENTS = {
  syncDone: "sync:done",
  syncError: "sync:error",
} as const;
