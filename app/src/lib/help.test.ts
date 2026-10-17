import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import App from "../App.svelte";
import { GUIDE_URL, helpUrl, SETUP_HELP } from "./help";
import { modules } from "./modules";

// The guide's pages, as text. A help link whose page or heading is missing fails here, so a
// renamed page or heading has to take its links along.
const pages = import.meta.glob<string>("../../../docs/guide/*.md", { query: "?raw", import: "default", eager: true });
const guide = new Map(Object.entries(pages).map(([path, text]) => [path.split("/").pop() ?? path, text]));

/** GitHub's heading anchor: lower case, punctuation dropped, spaces to hyphens. */
function slug(heading: string): string {
  return heading
    .trim()
    .toLowerCase()
    .replace(/[`*_]/g, "")
    .replace(/[^\w\- ]/g, "")
    .replace(/ /g, "-");
}

/** The anchors of a page, with GitHub's -1, -2 for repeated headings; code fences are skipped. */
function anchors(text: string): Set<string> {
  const out = new Set<string>();
  const seen = new Map<string, number>();
  let fence = false;
  for (const line of text.split("\n")) {
    if (line.startsWith("```")) fence = !fence;
    const m = fence ? null : /^#{1,6}\s+(.*)/.exec(line);
    if (!m?.[1]) continue;
    const s = slug(m[1]);
    const n = seen.get(s) ?? 0;
    seen.set(s, n + 1);
    out.add(n === 0 ? s : `${s}-${n}`);
  }
  return out;
}

/** Why `help` ("galaxy.md#services") does not point at a guide page and heading, or null. */
function problem(help: string): string | null {
  const [file, anchor] = help.split("#") as [string, string | undefined];
  const text = guide.get(file);
  if (text === undefined) return `${help}: no docs/guide/${file}`;
  if (anchor !== undefined && !anchors(text).has(anchor)) return `${help}: no heading #${anchor} in ${file}`;
  return null;
}

describe("help links", () => {
  it("finds the guide", () => {
    expect(guide.has("index.md")).toBe(true);
  });

  it("gives every page a help link to a guide page and heading that exist", () => {
    expect(modules.length).toBeGreaterThan(20);
    const bad = modules.map((m) => (m.help ? problem(m.help) : `${m.id}: no help link`)).filter(Boolean);
    expect(bad).toEqual([]);
  });

  it("has a target for first-run setup", () => {
    expect(problem(SETUP_HELP)).toBeNull();
  });

  it("uses only the project's GitHub pages, which the shell allows to open", () => {
    expect(GUIDE_URL.startsWith("https://github.com/spadaval/astromech/")).toBe(true);
    expect(helpUrl("trade.md")).toBe("https://github.com/spadaval/astromech/blob/main/docs/guide/trade.md");
  });

  it("links every guide page from the index", () => {
    const index = guide.get("index.md") ?? "";
    const missing = [...guide.keys()].filter((f) => f !== "index.md" && !index.includes(`](${f})`));
    expect(missing).toEqual([]);
  });
});

describe("the Help link in the window", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
    vi.restoreAllMocks();
  });

  it("opens the guide page of the page you are on", async () => {
    const open = vi.spyOn(window, "open").mockImplementation(() => null);
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");
    await fireEvent.click(screen.getByRole("button", { name: "Help" }));
    expect(open).toHaveBeenCalledWith(helpUrl("powerplay.md"), "_blank");
  });

  it("follows the page: Trade opens the Trade guide", async () => {
    const open = vi.spyOn(window, "open").mockImplementation(() => null);
    location.hash = "#/trade";
    render(App);
    await screen.findByRole("button", { name: "Sell" });
    await fireEvent.click(screen.getByRole("button", { name: "Help" }));
    expect(open).toHaveBeenCalledWith(helpUrl("trade.md"), "_blank");
  });
});
