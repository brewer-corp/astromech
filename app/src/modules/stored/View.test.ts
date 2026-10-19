import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import sample from "../../lib/fixtures/engineering.json";
import type { Fleet } from "../../lib/engineering";
import View from "./View.svelte";

let fleet: Fleet;

vi.mock("../../lib/engineering", async (importActual) => {
  const real = await importActual<typeof import("../../lib/engineering")>();
  return { ...real, loadFleet: async () => fleet };
});

beforeEach(() => {
  fleet = structuredClone(sample.fleet) as unknown as Fleet;
});
afterEach(cleanup);

it("lists stored modules with blueprint, system and value", async () => {
  render(View);
  expect(await screen.findByText("5A Frame Shift Drive")).toBeTruthy();
  expect(screen.getByText(/G3 Increased range/)).toBeTruthy();
  expect(screen.getByText("Shinrarta Dezhra")).toBeTruthy();
});

it("says when storage is empty", async () => {
  fleet.stored_modules = [];
  render(View);
  expect(await screen.findByText("No modules in storage.")).toBeTruthy();
});
