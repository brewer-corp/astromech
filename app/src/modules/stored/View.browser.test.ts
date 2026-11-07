import { afterEach, beforeEach, expect, it } from "vitest";
import { cleanup, render, screen } from "@testing-library/svelte";
import sample from "../../preview/fixtures/engineering.json";
import type { Fleet } from "../../lib/engineering";
import { answer } from "../../test/transport";
import View from "./View.svelte";

let fleet: Fleet;

answer("engineering_fleet", () => fleet);

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
