import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";

// No history yet (first run, before the core's import): the shell answers null, not an error.
vi.mock("../../lib/exploration", async (orig) => ({
  ...(await orig<typeof import("../../lib/exploration")>()),
  loadUnsold: vi.fn(async () => null),
}));

import View from "./View.svelte";

describe("Exploration unsold data without a history", () => {
  it("says there is no commander yet instead of an error", async () => {
    render(View);
    await screen.findByText("No commander in the journal history yet.");
    expect(document.querySelector(".err")).toBeNull();
  });
});
