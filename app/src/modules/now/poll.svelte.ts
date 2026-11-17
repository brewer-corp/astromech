// One of the Now page's reads, polled on its own clock as the page it summarises polls it
// (Station's docked_here, Session's summary, Powerplay's overview, System's unsold data). A
// failed read keeps the last answer and says so (the shared Stale line); with no answer yet it
// is the failure in words, with no action, since the next read is the recovery.
import { pollFailure } from "../../lib/ui/errors";

export class Poll<T> {
  /** The last answer; null both before one and when the answer is null (see `loaded`). */
  value = $state<T | null>(null);
  /** An answer has come. */
  loaded = $state(false);
  /** The last read failed: "Could not read …" with no answer yet, else "Could not refresh: …". */
  failed = $state<string | null>(null);

  constructor(
    private readonly read: () => Promise<T>,
    /** What it reads, for the failure: "read the session". */
    private readonly what: string,
    private readonly every: number,
  ) {}

  start(): () => void {
    const tick = () =>
      this.read()
        .then((v) => {
          this.value = v;
          this.loaded = true;
          this.failed = null;
        })
        .catch((e: unknown) => (this.failed = pollFailure(this.what, e, this.loaded)));
    void tick();
    const t = setInterval(() => void tick(), this.every);
    return () => clearInterval(t);
  }
}
