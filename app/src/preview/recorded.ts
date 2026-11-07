/**
 * The browser preview's handler types, and its fixtures checked against the core's reply types
 * (app/src/lib/api/, generated from the command registry).
 */
import type { Args, Reply, WindowCommand } from "../lib/transport";

/** How the preview answers command `K`: its reply type, or a rejection as a failed call. */
export type Handler<K extends WindowCommand> = (args: Args<K>) => Reply<K> | Promise<Reply<K>>;
/** A handler for every command the window may send. */
export type Handlers = { [K in WindowCommand]: Handler<K> };

/** `T` as a JSON import types it: literals widened (`"ok"` as `string`, `true` as `boolean`),
 * tuples as arrays. */
type Json<T> = T extends string
  ? string
  : T extends number
    ? number
    : T extends boolean
      ? boolean
      : T extends null | undefined
        ? T
        : T extends readonly (infer E)[]
          ? Json<E>[]
          : T extends object
            ? { [K in keyof T]: Json<T[K]> }
            : T;

/**
 * A recorded fixture as the reply type `T` it holds. A field the type has and the fixture lacks,
 * or one of the wrong type, fails `pnpm check`. A JSON import widens string literals and tuples,
 * so those are checked as `string` and arrays; a fixture's extra fields aren't checked.
 */
export function recorded<T>(json: Json<T>): T {
  return json as unknown as T;
}

/** `reply` labelled as the preview's sample ("Sample journal history"), which the page shows
 * beside it (lib/transport.ts `Sampled`); the core never sends `source` for these. */
export function labelled<T extends object>(reply: T, source: string): T {
  return { ...reply, source };
}
