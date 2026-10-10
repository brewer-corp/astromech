/**
 * Module manifest pattern (docs/ARCHITECTURE.md §18): each feature module (keybinds, ship,
 * engineering, ...) exports one manifest. The shell builds its navigation from the registry and
 * lazy-loads a module's view only when it is opened.
 */
import type { Component } from "svelte";

export interface ModuleManifest {
  /** Stable id, also the route: #/<id> */
  id: string;
  title: string;
  /** Short line for the nav tooltip. */
  summary: string;
  /** Lower sorts first in the nav. */
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
