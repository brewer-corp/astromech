import { beforeEach, describe, expect, it } from "vitest";
import { render, screen, fireEvent, within, cleanup } from "@testing-library/svelte";
import catalog from "../../../../data/catalog/activities.json";
import fixture from "../../lib/fixtures/sample.json";
import { maps } from "../../lib/devices";
import type { Row } from "../../lib/binds";
import { binds } from "../../lib/store.svelte";
import View from "./View.svelte";

const rows = (fixture.view.groups as unknown as { rows: Row[] }[]).flatMap((g) => g.rows);
const isBound = (r: Row) => r.slots.some((s) => s.device && s.key);
const device = maps[0]!.device_id;
const onDevice = rows.filter((r) => r.slots.some((s) => s.device === device && s.key));

const everything = () => screen.getByRole("heading", { name: /everything on this stick|on this stick/i }).parentElement!;
const names = (el: HTMLElement) => [...el.querySelectorAll(".row .n")].map((n) => n.textContent);

const other = maps[1]!.device_id;
const onOtherOnly = (ids: Set<string>) =>
  rows.filter((r) => ids.has(r.id) && !r.slots.some((s) => s.device === device && s.key) && r.slots.some((s) => s.device === other && s.key));

describe("My controller", () => {
  beforeEach(() => localStorage.clear());

  it("lists every binding on the device under All", async () => {
    render(View);
    await screen.findByText(onDevice[0]!.name, { selector: ".n" });

    expect(names(everything()).sort()).toEqual(onDevice.map((r) => r.name).sort());
    expect(screen.queryByText("Not bound yet")).toBeNull();
  });

  it("narrows to an activity's actions and lists its unbound gaps", async () => {
    const trading = catalog.activities.find((a) => a.id === "trading")!;
    const ids = new Set(trading.actions.flatMap((a) => [a.id, ...a.alternatives]));
    const byId = new Map(rows.map((r) => [r.id, r]));
    const gaps = trading.actions
      .filter((a) => a.important && ![a.id, ...a.alternatives].some((i) => byId.has(i) && isBound(byId.get(i)!)))
      .map((a) => byId.get(a.id)?.name ?? a.id);
    expect(gaps.length).toBeGreaterThan(0); // the fixture must have something unbound for this to mean anything

    render(View);
    await screen.findByText(onDevice[0]!.name, { selector: ".n" });
    await fireEvent.click(screen.getByRole("button", { name: trading.title }));

    const expected = onDevice.filter((r) => ids.has(r.id)).map((r) => r.name).sort();
    expect(expected.length).toBeLessThan(onDevice.length);
    expect(names(everything()).sort()).toEqual(expected);

    const gapList = screen.getByRole("heading", { name: "Not bound yet" }).parentElement!;
    const bindable = within(gapList).getAllByRole("button", { name: "Bind this" }).map((b) => b.parentElement!.querySelector(".n")!.textContent);
    expect(bindable.sort()).toEqual(gaps.sort());
  });

  it("lists an activity's actions on the other stick, which are neither on this stick nor elsewhere", async () => {
    const mining = catalog.activities.find((a) => a.id === "mining")!;
    const ids = new Set(mining.actions.flatMap((a) => [a.id, ...a.alternatives]));
    const expected = onOtherOnly(ids).map((r) => r.name).sort();
    expect(expected.length).toBeGreaterThan(0); // the fixture binds mining actions only on the right stick

    render(View);
    await screen.findByText(onDevice[0]!.name, { selector: ".n" });
    expect(screen.queryByRole("heading", { name: "On the other stick" })).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: mining.title }));

    // The rows under the heading, up to the next one.
    const listed: Element[] = [];
    for (let e = screen.getByRole("heading", { name: "On the other stick" }).nextElementSibling; e && e.tagName !== "H3"; e = e.nextElementSibling) {
      listed.push(e);
    }
    expect(listed.map((r) => r.querySelector(".n")!.textContent).sort()).toEqual(expected);
    // They are not also listed as "Elsewhere (keyboard, mouse)".
    const elsewhere = screen.queryByRole("heading", { name: /^Elsewhere/ })?.parentElement;
    for (const name of expected) {
      expect(within(everything()).queryByText(name, { selector: ".n" })).toBeNull();
      if (elsewhere) expect(within(elsewhere).queryAllByText(name, { selector: ".n" }).length).toBe(1); // only the other-stick row
    }
  });

  it("opens an action's detail with its slots and Bind another control", async () => {
    const gear = onDevice.find((r) => r.slots.some((s) => s.device === device && s.key))!;
    render(View);
    await fireEvent.click(await screen.findByText(gear.name, { selector: ".n" }));

    const panel = screen.getByText("Action").parentElement!;
    expect(within(panel).getByRole("heading", { name: gear.name })).toBeTruthy();
    expect(within(panel).getByText(gear.context)).toBeTruthy();
    const bound = gear.slots.filter((s) => s.device && s.key);
    expect(within(panel).getAllByText(/^(Primary|Secondary|Axis): /).length).toBe(gear.slots.length);
    expect(bound.length).toBeGreaterThan(0);
    expect(within(panel).getByRole("button", { name: "Bind another control" })).toBeTruthy();

    // An action on the other stick opens the same detail.
    const mining = catalog.activities.find((a) => a.id === "mining")!;
    const far = onOtherOnly(new Set(mining.actions.flatMap((a) => [a.id, ...a.alternatives])))[0]!;
    await fireEvent.click(screen.getByRole("button", { name: mining.title }));
    await fireEvent.click(screen.getByText(far.name, { selector: ".n" }));
    const detail = screen.getByText("Action").parentElement!;
    expect(within(detail).getByRole("heading", { name: far.name })).toBeTruthy();
    expect(within(detail).getByRole("button", { name: "Bind another control" })).toBeTruthy();
  });

  it("shows the control just pressed, not an action opened before", async () => {
    const gear = onDevice[0]!;
    const press = onDevice.find((r) => r.id !== gear.id)!.slots.find((s) => s.device === device && s.key)!;
    render(View);
    await fireEvent.click(await screen.findByText(gear.name, { selector: ".n" }));
    expect(screen.getByText("Action")).toBeTruthy();

    binds.press({ device, key: press.key, kind: "button", down: true, device_name: "VKB", t_ms: 1 });
    expect(await screen.findByText("You pressed")).toBeTruthy();
    expect(screen.queryByText("Action")).toBeNull();
  });

  it("opens the bind dialog from Bind another control", async () => {
    const gear = onDevice[0]!;
    render(View);
    await fireEvent.click(await screen.findByText(gear.name, { selector: ".n" }));
    await fireEvent.click(screen.getByRole("button", { name: "Bind another control" }));
    expect(screen.getByRole("dialog", { name: /^Bind / })).toBeTruthy();
  });

  it("remembers the chosen activity across a reload", async () => {
    const trading = catalog.activities.find((a) => a.id === "trading")!;
    render(View);
    await screen.findByText(onDevice[0]!.name, { selector: ".n" });
    await fireEvent.click(screen.getByRole("button", { name: trading.title }));
    expect(screen.getByRole("heading", { name: `${trading.title} on this stick` })).toBeTruthy();

    cleanup(); // the window is reloaded: a new page, the same browser storage
    render(View);
    expect(await screen.findByRole("heading", { name: `${trading.title} on this stick` })).toBeTruthy();

    await fireEvent.click(screen.getByRole("button", { name: "All" }));
    cleanup();
    render(View);
    expect(await screen.findByRole("heading", { name: "Everything on this stick" })).toBeTruthy();
  });
});
