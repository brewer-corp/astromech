import { afterEach, describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import OriginInput from "./OriginInput.svelte";

const names = vi.fn<(p: string) => Promise<string[]>>();
vi.mock("../search", async (orig) => ({
  ...(await orig<typeof import("../search")>()),
  systemNames: (p: string) => names(p),
}));

afterEach(() => vi.clearAllMocks());

async function typeAndEnter(text: string, hits: string[], arrows = 0) {
  names.mockResolvedValue(hits);
  const onchange = vi.fn();
  render(OriginInput, { origin: null, onchange });
  const box = screen.getByLabelText("Search from system");
  await fireEvent.input(box, { target: { value: text } });
  await screen.findByRole("listbox", {}, { timeout: 2000 });
  for (let i = 0; i < arrows; i++) await fireEvent.keyDown(box, { key: "ArrowDown" });
  await fireEvent.keyDown(box, { key: "Enter" });
  await waitFor(() => expect(onchange).toHaveBeenCalled());
  return onchange;
}

describe("OriginInput Enter", () => {
  it("takes the suggestion that matches the typed text, not the first one", async () => {
    const onchange = await typeAndEnter("sol", ["Solati", "SOL", "Solon"]);
    expect(onchange).toHaveBeenCalledWith("SOL");
  });

  it("takes the typed text when no suggestion matches it exactly", async () => {
    const onchange = await typeAndEnter("Sho", ["Shinrarta Dezhra", "Shoi"]);
    expect(onchange).toHaveBeenCalledWith("Sho");
  });

  it("takes the suggestion the commander arrowed to", async () => {
    const onchange = await typeAndEnter("Sho", ["Shinrarta Dezhra", "Shoi"], 1);
    expect(onchange).toHaveBeenCalledWith("Shoi");
  });
});
