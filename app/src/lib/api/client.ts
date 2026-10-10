/**
 * Typed client for the core's local WebSocket API (astromech-proto). Placeholder: the real
 * types are generated from astromech-proto into src/lib/api/generated (planned).
 */
export type Origin = "voice" | "app_window" | "dashboard" | "overlay" | "cloud";

export type Request =
  | { kind: "query"; id: number; name: string; args: unknown }
  | { kind: "command"; id: number; name: string; args: unknown; origin: Origin }
  | { kind: "subscribe"; id: number; topic: string };

export type ConnectionState = "connecting" | "open" | "closed";

export const PROTOCOL_VERSION = 1;
