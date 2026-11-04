import { tick } from "svelte";

/**
 * Puts `text` in a live region so a screen reader announces it, even when it is the same words
 * as last time (a second "Saved", the same error twice): an unchanged region is not read again,
 * so it is emptied first and filled once the page has updated.
 */
export async function announce(set: (text: string) => void, text: string): Promise<void> {
  set("");
  await tick();
  set(text);
}
