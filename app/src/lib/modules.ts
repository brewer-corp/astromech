/**
 * Module manifest pattern (docs/ARCHITECTURE.md §18): each feature module (keybinds, ship,
 * engineering, ...) exports one manifest. The shell builds its navigation from the registry and
 * lazy-loads a module's view only when it is opened.
 */
import type { Component } from "svelte";

/** Top-level tabs. Each module sits under one; the group's modules are its sub-tabs. */
export const groups = [
  { id: "controls", title: "Controls" },
  { id: "engineering", title: "Engineering" },
  { id: "galaxy", title: "Galaxy" },
  { id: "powerplay", title: "Powerplay" },
  { id: "logbook", title: "Logbook" },
  { id: "settings", title: "Settings" },
] as const;

export type GroupId = (typeof groups)[number]["id"];

export interface ModuleManifest {
  /** Stable id, also the route: #/<id> */
  id: string;
  /** Top-level tab it sits under. */
  group: GroupId;
  title: string;
  /** Short line for the nav tooltip. */
  summary: string;
  /** Lower sorts first within its group; the first one is where the group opens. */
  order: number;
  /** API namespaces the module uses (for the core's capability check), e.g. "binds". */
  api: string[];
  /** Lazy view loader so modules do not bloat the first paint. */
  load: () => Promise<{ default: Component }>;
}

const found = import.meta.glob<ModuleManifest>("../modules/*/manifest.ts", { eager: true, import: "default" });

export const modules: ModuleManifest[] = Object.values(found).sort((a, b) => a.order - b.order);

export function findModule(id: string): ModuleManifest | undefined {
  return modules.find((m) => m.id === id);
}

export function groupModules(g: GroupId): ModuleManifest[] {
  return modules.filter((m) => m.group === g);
}
