/// <reference types="vite/client" />
interface ImportMetaEnv {
  /** A preview build's default query (scripts/preview-publish), e.g. "core=ready&route=exact". */
  readonly VITE_PREVIEW_QUERY?: string;
  /** A preview build's start page, e.g. "route". */
  readonly VITE_PREVIEW_PAGE?: string;
}
