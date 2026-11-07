import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { answer } from "../../test/transport";
import View from "./View.svelte";

// No history yet (first run, before the core's import): the core answers null, not an error.
answer("exploration_unsold", () => null);

describe("Exploration unsold data without a history", () => {
  it("says there is no commander yet instead of an error", async () => {
    render(View);
    await screen.findByText("No commander in the journal history yet.");
    expect(document.querySelector(".err")).toBeNull();
  });
});
