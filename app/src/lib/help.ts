/**
 * Help links: every page has a page in the user guide (docs/guide/), opened on GitHub in the
 * browser through the shell's `open_external` (which allows the project's GitHub pages).
 * `help.test.ts` checks that each target file and heading exists.
 */
import { openUrl } from "./core";

/** Where the guide is served: the pages on `main`, rendered by GitHub. */
export const GUIDE_URL = "https://github.com/spadaval/astromech/blob/main/docs/guide/";

/** The guide page of first-run setup (it is not a module). */
export const SETUP_HELP = "getting-started.md#first-run";

/** The link for a manifest's `help` ("galaxy.md#services"). */
export function helpUrl(help: string): string {
  return GUIDE_URL + help;
}

export function openHelp(help: string): Promise<void> {
  return openUrl(helpUrl(help));
}
