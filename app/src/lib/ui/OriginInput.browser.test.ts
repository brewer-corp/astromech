import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { answer, idle } from "../../test/transport";
import OriginInput from "./OriginInput.svelte";

const names = vi.fn<(p: string) => Promise<string[]>>();
answer("galaxy_systems", ({ prefix }: { prefix: string }) => names(prefix));

afterEach(() => vi.clearAllMocks());

async function typeAndEnter(text: string, hits: string[], arrows = 0) {
  names.mockResolvedValue(hits);
  const onchange = vi.fn();
  render(OriginInput, { origin: null, onchange });
  const box = screen.getByLabelText("Search from system");
  await fireEvent.input(box, { target: { value: text } });
  await screen.findByRole("listbox");
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

const route = vi.fn<() => Promise<{ next: { system: string } | null }>>();
answer("route", () => route().then((view) => ({ ok: true, view })));

describe("OriginInput around (a Places finder)", () => {
  it("is labelled Around and offers the route's next stop, then Here once it searches elsewhere", async () => {
    route.mockResolvedValue({ next: { system: "Deciat" } });
    const onchange = vi.fn();
    const { rerender } = render(OriginInput, { origin: null, onchange, around: true });
    expect(screen.getByText("Around")).toBeTruthy();
    const box = screen.getByLabelText("Search from system");
    await fireEvent.focus(box);
    await fireEvent.click(await screen.findByRole("option", { name: "Next stop: Deciat" }));
    expect(onchange).toHaveBeenLastCalledWith("Deciat");
    await rerender({ origin: "Deciat", onchange, around: true });
    expect((box as HTMLInputElement).value).toBe("Next stop: Deciat");

    await fireEvent.focus(box);
    const options = await screen.findAllByRole("option");
    expect(options.map((o) => o.textContent)).toEqual(["Here"]);
    await fireEvent.click(options[0]!);
    expect(onchange).toHaveBeenLastCalledWith(null);
  });

  it("offers nothing extra without a route, and nothing at all outside a finder", async () => {
    route.mockResolvedValue({ next: null });
    render(OriginInput, { origin: null, onchange: vi.fn(), around: true });
    await fireEvent.focus(screen.getByLabelText("Search from system"));
    await idle();
    expect(screen.queryByRole("listbox")).toBeNull();
    expect(route).toHaveBeenCalledTimes(1);
    cleanup();
    route.mockClear();
    render(OriginInput, { origin: null, onchange: vi.fn() });
    expect(screen.getByText("From")).toBeTruthy();
    await fireEvent.focus(screen.getByLabelText("Search from system"));
    expect(route).not.toHaveBeenCalled();
  });
});
