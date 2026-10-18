/**
 * What a failed call says on screen: what was being read, then the reason as the core gave it
 * (the shell rejects with a string, other callers with an Error). Never "Error: ..." or a bare
 * reason with no page context.
 */
export function failure(what: string, e: unknown): string {
  const detail = (e instanceof Error ? e.message : String(e))
    .replace(/^Error:\s*/, "")
    .trim()
    .replace(/[.\s]+$/, "");
  return detail ? `Could not ${what}: ${detail}.` : `Could not ${what}.`;
}
