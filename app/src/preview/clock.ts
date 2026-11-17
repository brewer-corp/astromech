/**
 * `?clock=<RFC 3339>`: the preview's clock starts at that moment and runs on from it, so a page
 * that tells time (the Powerplay cycle's time left, data ages) draws the same on any day. The
 * captures set it (scripts/browser/audit.mjs, a plan's `clock`). Without it the preview runs on the
 * wall clock. Only `Date` moves; timers keep their real pace.
 */
export function installPreviewClock(at: string | null): void {
  if (at == null) return;
  const start = Date.parse(at);
  if (Number.isNaN(start)) throw new Error(`preview: ?clock=${at} is not a date`);
  const Real = Date;
  const offset = start - Real.now();
  class Shifted extends Real {
    constructor(...args: unknown[]) {
      if (args.length) super(...(args as [string]));
      else super(Real.now() + offset);
    }
    static override now(): number {
      return Real.now() + offset;
    }
  }
  globalThis.Date = Shifted as DateConstructor;
}
