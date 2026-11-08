/// <reference types="vite/client" />
interface ImportMetaEnv {
  /** Set by scripts/preview-publish: outside Tauri, the browser preview answers (lib/transport.ts). */
  readonly VITE_PREVIEW?: string;
  /** A preview build's default query (scripts/preview-publish), e.g. "core=ready&route=exact". */
  readonly VITE_PREVIEW_QUERY?: string;
  /** A preview build's start page, e.g. "route". */
  readonly VITE_PREVIEW_PAGE?: string;
  /** Defined by the dev bridge's plugin (app/dev/bridge.ts) in `ASTROMECH_WORLD=<dir> pnpm dev`:
   * the path the page posts its calls to (lib/transport.ts). */
  readonly VITE_ASTROMECH_BRIDGE?: string;
}

/** app/package.json's version, built in by vite.config.ts (lib/core.ts WINDOW_VERSION). */
declare const __APP_VERSION__: string;
