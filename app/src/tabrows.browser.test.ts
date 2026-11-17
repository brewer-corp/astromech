// DESIGN.md The Two Tab Rows Rule: top tabs plus one row of page tabs, never more. A third level
// is a choice (aria-pressed cells), a section or a page of its own. Every page is opened in the
// browser preview's data and its tab rows counted; folded levels keep their links.
import { afterEach, describe, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import App from "./App.svelte";
import { modules } from "./lib/modules";
import { FINDERS } from "./lib/places";
import { answer, idle } from "./test/transport";

answer("status", () => null);

/**
 * Rows that read as tabs: two or more sibling buttons or links, one marked current (class "on",
 * aria-selected or aria-current), that are not a choice (aria-pressed). Rows that are something
 * else are left out by their role, not their markup: a table's rows (Presets: a preset per row,
 * a cell per context) and a named navigation landmark, which picks a thing rather than a level
 * (the Ships page's rail, "Your ships"). The two tab rows are unnamed `nav`s, and the first test
 * below checks they are still counted.
 */
function tabRows(root: ParentNode): Element[] {
  const rows = new Set<Element>();
  for (const el of root.querySelectorAll("button, a")) {
    const row = el.parentElement;
    if (!row || rows.has(row)) continue;
    if (row.closest("table, [role=table], [role=grid], nav[aria-label]")) continue;
    const items = [...row.children].filter((c) => c.matches("button, a"));
    if (items.length < 2) continue;
    if (items.some((i) => i.hasAttribute("aria-pressed"))) continue;
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
  "materials?kind=onfoot",
  "unlocks?kind=suit",
];

describe("The Two Tab Rows Rule", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  it("counts the top tabs and the page tabs", async () => {
    location.hash = "#/materials";
    const { container } = render(App);
    await screen.findByRole("button", { name: "Raw", pressed: true });
    expect(tabRows(container).map(describeRow)).toEqual([
      "Ships | Engineering | Places | Trade | Exploration | Powerplay | Logbook | Controls | Settings",
      "Materials | Engineers",
    ]);
  });

  it.each(ROUTES)("#/%s shows at most two tab rows", async (route) => {
    location.hash = `#/${route}`;
    const { container } = render(App);
    const main = container.querySelector("main");
    // The page has drawn from the preview data once its loading lines are gone.
    await waitFor(() => expect(main?.children.length).toBeGreaterThan(0));
    await waitFor(() => expect(main?.textContent ?? "").not.toMatch(/Reading the|Loading/));
    await idle();
    const rows = tabRows(container);
    expect(rows.length, `tab rows on #/${route}: ${rows.map(describeRow).join(" // ")}`).toBeLessThanOrEqual(2);
  });
});

describe("Old links land where their part moved (astromech-5a01)", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  it.each([
    ["odyssey", "materials?kind=onfoot", "Engineering", "Materials"],
    ["odyssey?view=inventory", "materials?kind=onfoot", "Engineering", "Materials"],
    ["odyssey?view=blueprints", "suits", "Ships", "Suits"],
    ["odyssey?view=engineers", "unlocks?kind=suit", "Engineering", "Engineers"],
    ["engineering", "engineering", "Ships", "Ships"],
    ["stored", "stored", "Ships", "Stored modules"],
  ])("#/%s opens #/%s, under %s > %s", async (from, to, top, page) => {
    location.hash = `#/${from}`;
    const { container } = render(App);
    await waitFor(() => expect(location.hash).toBe(`#/${to}`));
    expect(container.querySelector("header nav button.on")?.textContent).toBe(top);
    expect(container.querySelector("nav.sub button.on")?.textContent).toBe(page);
  });
});

describe("Folded levels keep their links", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
  });

  const pressed = (name: string | RegExp) => screen.getByRole("button", { name, pressed: true });

  it("#/engineering/<id>/plan opens that ship's plan with Plan chosen", async () => {
    location.hash = "#/engineering/7/plan";
    render(App);
    await waitFor(() => expect(pressed("Plan")).toBeTruthy());
    expect(screen.getByRole("button", { name: "Loadout", pressed: false })).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Loadout", pressed: false }));
    expect(location.hash).toBe("#/engineering/7");
    await waitFor(() => expect(pressed("Loadout")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Plan", pressed: false }));
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

  it("#/materials?kind=onfoot opens the On foot kind, and an on-foot item opens in the panel", async () => {
    location.hash = "#/materials?kind=onfoot";
    render(App);
    await waitFor(() => expect(pressed("On foot")).toBeTruthy());
    await fireEvent.click(await screen.findByRole("button", { name: /^Graphene, / }));
    expect(location.hash).toBe("#/materials/onfoot/graphene");
    expect(await screen.findByRole("heading", { name: "Graphene" })).toBeTruthy();
    expect(pressed("On foot")).toBeTruthy();
  });

  it("#/unlocks?kind=suit opens the suit engineers", async () => {
    location.hash = "#/unlocks?kind=suit";
    render(App);
    await waitFor(() => expect(pressed("Suit engineers")).toBeTruthy());
    await fireEvent.click(screen.getByRole("button", { name: "Ship engineers", pressed: false }));
    expect(location.hash).toBe("#/unlocks");
  });
});
