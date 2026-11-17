import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, render, screen, within } from "@testing-library/svelte";
import sample from "../../preview/fixtures/engineering.json";
import type { Fleet } from "../../lib/engineering";
import { answer, idle } from "../../test/transport";
import View from "./View.svelte";

let fleet: Fleet;

answer("engineering_fleet", () => fleet);

beforeEach(() => {
  fleet = structuredClone(sample.fleet) as unknown as Fleet;
});
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

it("lists stored modules with blueprint, system and value", async () => {
  render(View);
  expect(await screen.findByText("5A Frame Shift Drive")).toBeTruthy();
  expect(screen.getByRole("img", { name: "Increased range, grade 3 of 5" })).toBeTruthy();
  const where = screen.getByRole("region", { name: "Shinrarta Dezhra" });
  expect(within(where).getByText("5A Frame Shift Drive")).toBeTruthy();
  expect(within(where).getByText(/^5M/)).toBeTruthy();
});

it("says when storage is empty", async () => {
  fleet.stored_modules = [];
  render(View);
  expect(await screen.findByText("No modules in storage.")).toBeTruthy();
});

it("keeps the selected module in the panel when another one leaves storage", async () => {
  vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
  render(View);
  (await screen.findByText("5A Shield Generator")).click();
  const panel = await screen.findByRole("complementary", { name: "Details" });
  expect(within(panel).getByRole("heading").textContent).toBe("5A Shield Generator");

  fleet.stored_modules = fleet.stored_modules.slice(1);
  await vi.advanceTimersByTimeAsync(1000);
  await idle();
  expect(screen.queryByText("5A Frame Shift Drive")).toBeNull();
  expect(within(screen.getByRole("complementary", { name: "Details" })).getByRole("heading").textContent).toBe("5A Shield Generator");
});
