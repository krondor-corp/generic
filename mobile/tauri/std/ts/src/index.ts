// @mobile/std — the frontend half of mobile-std.
//
// The rule for apps in this repo: the webview is a view. Network, SQLite,
// notifications, and sync all run in Rust; the frontend calls commands
// and listens to events. These helpers make that typed and boring.

import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useEffect, useRef } from "react";

/** True when running inside a Tauri webview (false in a plain browser tab). */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Name a `#[tauri::command]` once, get a typed async function.
 *
 *   const listStories = command<{ limit: number }, Story[]>("list_stories");
 *   const stories = await listStories({ limit: 50 });
 *
 * Errors are the Rust error's display string (mobile-std's `Error`
 * serializes that way), rethrown as `CommandError`.
 */
export function command<Args extends Record<string, unknown> | void, Result>(
  name: string,
): (args: Args) => Promise<Result> {
  return async (args: Args) => {
    try {
      return await invoke<Result>(name, (args ?? undefined) as Record<string, unknown> | undefined);
    } catch (e) {
      throw new CommandError(name, typeof e === "string" ? e : String(e));
    }
  };
}

export class CommandError extends Error {
  constructor(
    public readonly command: string,
    message: string,
  ) {
    super(message);
    this.name = "CommandError";
  }
}

/**
 * Subscribe to a Tauri event for the lifetime of a component. The handler
 * ref is kept current so callers can pass inline closures without
 * resubscribing on every render.
 */
export function useTauriEvent<T>(name: string, handler: (payload: T) => void): void {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    if (!isTauri()) return;
    let unlisten: UnlistenFn | undefined;
    let cancelled = false;
    listen<T>(name, (event) => handlerRef.current(event.payload)).then((fn) => {
      if (cancelled) fn();
      else unlisten = fn;
    });
    return () => {
      cancelled = true;
      unlisten?.();
    };
  }, [name]);
}
