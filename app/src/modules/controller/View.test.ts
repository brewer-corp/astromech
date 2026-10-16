import { describe, expect, it } from "vitest";
import { render, screen, fireEvent, within } from "@testing-library/svelte";
import catalog from "../../../../data/catalog/activities.json";
import fixture from "../../lib/fixtures/sample.json";
import { maps } from "../../lib/devices";
import type { Row } from "../../lib/binds";
import View from "./View.svelte";

const rows = (fixture.view.groups as unknown as { rows: Row[] }[]).flatMap((g) => g.rows);
const isBound = (r: Row) => r.slots.some((s) => s.device && s.key);
const device = maps[0]!.device_id;
const onDevice = rows.filter((r) => r.slots.some((s) => s.device === device && s.key));

const everything = () => screen.getByRole("heading", { name: /everything on this stick|on this stick/i }).parentElement!;
const names = (el: HTMLElement) => [...el.querySelectorAll(".row .n")].map((n) => n.textContent);

describe("My controller", () => {
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
});
