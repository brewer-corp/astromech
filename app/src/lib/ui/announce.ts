/** How long a live region stays empty before it is filled: long enough for the browser to
 * update its accessibility tree in between (Svelte's `tick` alone isn't: both changes would
 * land before the screen reader sees either). */
const GAP_MS = 100;

let latest = 0;

/**
 * Puts `text` in a live region so a screen reader announces it, even when it is the same words
 * as last time (a second "Saved", the same error twice): an unchanged region is not read again,
 * so it is emptied first and filled a moment later. A newer call wins over one still waiting.
 */
export async function announce(set: (text: string) => void, text: string): Promise<void> {
  const mine = ++latest;
  set("");
  await new Promise((r) => setTimeout(r, GAP_MS));
  if (mine === latest) set(text);
}
