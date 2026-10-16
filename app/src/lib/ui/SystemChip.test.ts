import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import SystemChip from "./SystemChip.svelte";

describe("SystemChip", () => {
  afterEach(() => vi.unstubAllGlobals());

  it("copies the exact system name to the clipboard", async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    vi.stubGlobal("navigator", { clipboard: { writeText } });
    render(SystemChip, { name: "Col 285 Sector AB-C d12-3" });

    await fireEvent.click(screen.getByRole("button", { name: "Copy Col 285 Sector AB-C d12-3" }));

    expect(writeText).toHaveBeenCalledExactlyOnceWith("Col 285 Sector AB-C d12-3");
    expect(await screen.findByText("Copied")).toBeTruthy();
  });
});
