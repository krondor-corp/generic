// Every command the Rust side exposes, named once. Shapes mirror the
// serde structs in src-tauri/src/commands.

import { command } from "@mobile/std";

export interface Story {
  id: number;
  seq: number;
  title: string;
  url: string | null;
  by: string;
  score: number;
  posted_at: number;
  fetched_at: string;
}

export interface ServerConfig {
  url: string;
}

export interface DeviceInfo {
  device_id: string;
  name: string;
  platform: string;
}

export interface Status {
  server: ServerConfig | null;
  device: DeviceInfo;
  cursor: number | null;
  story_count: number;
  last_sync_at: string | null;
  last_error: string | null;
}

export interface SyncReport {
  fetched: number;
  new_count: number;
  cursor: number;
}

export const getStatus = command<void, Status>("get_status");
export const saveSettings = command<{ url: string; deviceName: string }, Status>("save_settings");
export const testConnection = command<{ url: string }, string>("test_connection");
export const listStories = command<{ limit: number }, Story[]>("list_stories");
export const syncNow = command<void, SyncReport>("sync_now");
export const openStory = command<{ url: string }, void>("open_story");
export const resetData = command<void, Status>("reset_data");

export const EVENTS = {
  syncDone: "sync:done",
  syncError: "sync:error",
} as const;
