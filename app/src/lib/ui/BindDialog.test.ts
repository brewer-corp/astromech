import { describe, expect, it, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import { tick } from "svelte";
import BindDialog from "./BindDialog.svelte";
import { binds } from "../store.svelte";
import { api } from "../binds";

// The browser fallback serves the sample preset. Raw Input (INPUTSINK) forwards the window's own
// clicks as Mouse_1-3 presses, so a click in the dialog must never become the binding.
const press = (key: string) => binds.press({ device: "Mouse", key, kind: "button", down: true, device_name: "", t_ms: 0 });

async function open() {
  await binds.load();
  const row = binds.rows.find((r) => !r.analogue)!;
  const view = render(BindDialog, { row, slot: "Primary", onclose: () => {} });
  await tick();
  return view;
}

describe("Bind dialog capture", () => {
  it("does not take the window's own Mouse_1 click as the binding", async () => {
    await open();
    await fireEvent.click(screen.getByRole("dialog"));
    press("Mouse_1");
    await tick();

    expect(screen.getByText(/Press a button, hat or key/)).toBeTruthy();
    expect((screen.getByText("Save") as HTMLButtonElement).disabled).toBe(true);
  });

  it("never saves Mouse_1 after a click inside the dialog", async () => {
    const setSlot = vi.spyOn(api, "setSlot");
    await open();
    press("Mouse_1");
    await tick();
    await fireEvent.click(screen.getByText("Save"));
    expect(setSlot.mock.calls.every(([a]) => a.binding?.key !== "Mouse_1")).toBe(true);
    setSlot.mockRestore();
  });

  it("still binds a side mouse button (Mouse_4)", async () => {
    await open();
    press("Mouse_4");
    await tick();

    expect(screen.getByText("Mouse 4")).toBeTruthy();
    expect((screen.getByText("Save") as HTMLButtonElement).disabled).toBe(false);
  });
});
