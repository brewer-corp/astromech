// DESIGN.md The Two Tab Rows Rule: top tabs plus one row of page tabs, never more. A third level
// is a choice (aria-pressed cells), a section or a page of its own. Every page is opened in the
// browser preview's data and its tab rows counted; folded levels keep their links.
import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import App from "./App.svelte";
import { modules } from "./lib/modules";
import { FINDERS } from "./lib/places";

vi.mock("./lib/core", async (orig) => ({
  ...(await orig<typeof import("./lib/core")>()),
  coreStatus: () => Promise.resolve(null),
}));

/**
 * Rows that read as tabs: two or more sibling buttons or links of one kind, one marked current
 * (class "on", aria-selected or aria-current), that are not a choice (aria-pressed) and not cards
 * with pictures (the Ships page's ship rail picks a ship, it is not a level of navigation). A
 * Presets row (a name, then a cell per context) mixes kinds, so it is not counted.
 */
function tabRows(root: ParentNode): Element[] {
  const rows = new Set<Element>();
  for (const el of root.querySelectorAll("button, a")) {
    const row = el.parentElement;
    if (!row || rows.has(row)) continue;
    const items = [...row.children].filter((c) => c.matches("button, a"));
    if (items.length < 2) continue;
    const kind = (i: Element) => [i.tagName, ...[...i.classList].filter((c) => c !== "on").sort()].join(".");
    if (new Set(items.map(kind)).size > 1) continue;
    if (items.some((i) => i.hasAttribute("aria-pressed"))) continue;
    if (items.some((i) => i.querySelector("img, svg, canvas"))) continue;
    const current = (i: Element) =>
      i.classList.contains("on") || i.getAttribute("aria-selected") === "true" || i.hasAttribute("aria-current");
    if (items.some(current)) rows.add(row);
  }
  return [...rows];
}

const describeRow = (r: Element) => [...r.children].map((c) => c.textContent?.trim()).join(" | ");

/** Every page, the Ships page's plan, every Places finder and the folded levels' own links. */
const ROUTES = [
  ...modules.map((m) => m.id),
  "engineering/7/plan",
  ...FINDERS.map((f) => `places/${f.id}`),
  "materials?kind=manufactured",
  "odyssey?view=blueprints",
  "odyssey?view=engineers",
  "unlocks?kind=suit",
];

describe("The Two Tab Rows Rule", () => {
  beforeAll(() => (Element.prototype.scrollIntoView = vi.fn())); // jsdom has no layout
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  it.each(ROUTES)("#/%s shows at most two tab rows", async (route) => {
    location.hash = `#/${route}`;
    const { container } = render(App);
    const main = container.querySelector("main");
    // The page has drawn from the preview data once its loading lines are gone.
    await waitFor(() => expect(main?.children.length).toBeGreaterThan(0));
    await waitFor(() => expect(main?.textContent ?? "").not.toMatch(/Reading the|Loading/));
    await new Promise((r) => setTimeout(r, 50));
    const rows = tabRows(container);
    expect(rows.length, `tab rows on #/${route}: ${rows.map(describeRow).join(" // ")}`).toBeLessThanOrEqual(2);
  });
});

describe("Folded levels keep their links", () => {
  beforeAll(() => (Element.prototype.scrollIntoView = vi.fn()));
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  const pressed = (name: string | RegExp) => screen.getByRole("button", { name, pressed: true });

  it("#/engineering/<id>/plan opens that ship's plan with Engineering chosen", async () => {
    location.hash = "#/engineering/7/plan";
    render(App);
    await waitFor(() => expect(pressed("Engineering")).toBeTruthy());
    expect(screen.getByRole("button", { name: "Info", pressed: false })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Info", pressed: false }));
    expect(location.hash).toBe("#/engineering/7");
    await waitFor(() => expect(pressed("Info")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Engineering", pressed: false }));
    expect(location.hash).toBe("#/engineering/7/plan");
  });

  it("#/materials/<id> chooses the material's kind; #/materials?kind= chooses a kind", async () => {
    location.hash = "#/materials/dataminedwake";
    render(App);
    await screen.findByRole("heading", { name: "Datamined Wake Exceptions" });
    expect(pressed("Encoded")).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Manufactured", pressed: false }));
    expect(location.hash).toBe("#/materials?kind=manufactured");
    await waitFor(() => expect(pressed("Manufactured")).toBeTruthy());
    cleanup();

    location.hash = "#/materials?kind=encoded";
    render(App);
    await waitFor(() => expect(pressed("Encoded")).toBeTruthy());
  });

  it("#/odyssey?view= opens On foot's inventory, blueprints or engineers", async () => {
    location.hash = "#/odyssey?view=engineers";
    render(App);
    await waitFor(() => expect(pressed("Engineers")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Blueprints", pressed: false }));
    expect(location.hash).toBe("#/odyssey?view=blueprints");
    await waitFor(() => expect(pressed("Blueprints")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Inventory", pressed: false }));
    expect(location.hash).toBe("#/odyssey");
    await waitFor(() => expect(pressed("Goods")).toBeTruthy());
  });

  it("#/unlocks?kind=suit opens the suit engineers", async () => {
    location.hash = "#/unlocks?kind=suit";
    render(App);
    await waitFor(() => expect(pressed("Suit engineers")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Ship engineers", pressed: false }));
    expect(location.hash).toBe("#/unlocks");
  });
});
