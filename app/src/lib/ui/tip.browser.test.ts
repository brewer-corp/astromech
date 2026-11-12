import { afterEach, describe, expect, it } from "vitest";
import { userEvent } from "vitest/browser";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import IconButton from "./IconButton.svelte";
import Button from "./Button.svelte";
import { tip } from "./tip";
import { createRawSnippet } from "svelte";
import { createAttachmentKey } from "svelte/attachments";
import "../../design/tokens.css";

// What is drawn, with the window's stylesheet: a rule that overrides `hidden` would leave it up.
const shown = () => [...document.querySelectorAll<HTMLElement>(".tip")].filter((t) => t.checkVisibility());
const words = (s: string) => createRawSnippet(() => ({ render: () => `<span>${s}</span>` }));

describe("Tip", () => {
  afterEach(() => cleanup());

  it("shows at once on hover, swaps in place on the next target, and goes when the pointer leaves", async () => {
    render(IconButton, { icon: "add-stop", label: "Add Sol as a stop" });
    render(IconButton, { icon: "destination", label: "Set Sol as the destination", tip: "Set Sol as the destination (replaces the route)" });
    const [add, dest] = screen.getAllByRole("button");

    await fireEvent.pointerEnter(add!);
    expect(shown().map((t) => t.textContent)).toEqual(["Add Sol as a stop"]);
    const box = shown()[0];

    await fireEvent.pointerLeave(add!);
    await fireEvent.pointerEnter(dest!);
    expect(shown()).toEqual([box]);
    expect(box!.textContent).toBe("Set Sol as the destination (replaces the route)");

    await fireEvent.pointerLeave(dest!);
    expect(shown()).toEqual([]);
  });

  it("goes on Escape and on scroll, plain or rich", async () => {
    render(IconButton, { icon: "copy", label: "Copy Sol" });
    const copy = screen.getByRole("button");
    const mark = document.createElement("span");
    document.body.append(mark);
    const off = tip({ title: "Coriolis Starport", note: "Large pads", stop: false })(mark);

    await fireEvent.pointerEnter(copy);
    await fireEvent.keyDown(document, { key: "Escape" });
    expect(shown()).toEqual([]);

    await fireEvent.pointerEnter(mark);
    expect(shown().map((t) => t.textContent)).toEqual(["Coriolis StarportLarge pads"]);
    await fireEvent.keyDown(document, { key: "Escape" });
    expect(shown()).toEqual([]);

    await fireEvent.pointerEnter(copy);
    await fireEvent.scroll(window);
    expect(shown()).toEqual([]);
    if (off) off();
    mark.remove();
  });

  it("keeps showing on keyboard focus when its words change", async () => {
    const { rerender } = render(IconButton, { icon: "copy", label: "Copy Sol" });
    await userEvent.tab();
    await rerender({ icon: "copy", label: "Copy Sol", tip: "Copied" });
    expect(shown().map((t) => t.textContent)).toEqual(["Copied"]);
  });

  it("shows on keyboard focus, and screen readers get the same words", async () => {
    render(Button, { children: words("Spansh"), [createAttachmentKey()]: tip("Open Sol on Spansh") });
    render(IconButton, { icon: "copy", label: "Copy Sol" });

    await userEvent.tab();
    const spansh = screen.getByRole("button", { name: "Spansh", description: "Open Sol on Spansh" });
    expect(document.activeElement).toBe(spansh);
    expect(shown().map((t) => t.textContent)).toEqual(["Open Sol on Spansh"]);

    await userEvent.tab();
    expect(shown().map((t) => t.textContent)).toEqual(["Copy Sol"]);
    // Its tip is its name: it isn't read twice.
    expect(screen.getByRole("button", { name: "Copy Sol" }).hasAttribute("aria-describedby")).toBe(false);
  });

  it("gives a plain element a tab stop, and a repeated mark none", async () => {
    const host = document.createElement("div");
    host.innerHTML = `<span id="fact">12.4</span><span id="mark" role="img" aria-label="Coriolis Starport"></span>`;
    document.body.append(host);
    const offFact = tip("Full tank and full cargo")(host.querySelector("#fact")!);
    const offMark = tip({ title: "Coriolis Starport", stop: false })(host.querySelector("#mark")!);

    expect(host.querySelector<HTMLElement>("#fact")!.tabIndex).toBe(0);
    expect(host.querySelector("#mark")!.hasAttribute("tabindex")).toBe(false);

    if (offFact) offFact();
    if (offMark) offMark();
    host.remove();
  });
  it("sits beside its target and inside the window under the window's zoom", async () => {
    const root = document.documentElement;
    root.style.zoom = "1.44";
    root.style.setProperty("--zoom", "1.44");
    const host = document.createElement("div");
    host.innerHTML = `<button style="position:fixed;top:4px;right:4px">Help</button><span style="position:fixed;top:300px;left:200px">12.4</span>`;
    document.body.append(host);
    const [help, value] = [host.querySelector("button")!, host.querySelector("span")!];
    const offs = [tip("Open this page's guide in your browser")(help), tip("Full tank and full cargo")(value)];
    try {
      for (const [target, side] of [[help, "below"], [value, "above"]] as const) {
        await fireEvent.pointerEnter(target);
        const t = target.getBoundingClientRect();
        const b = shown()[0]!.getBoundingClientRect();
        expect(b.left).toBeGreaterThanOrEqual(0);
        expect(b.right).toBeLessThanOrEqual(window.innerWidth);
        // The gap is 8 px at the window's scale: within a few px of its edge.
        const gap = side === "below" ? b.top - t.bottom : t.top - b.bottom;
        expect(gap).toBeGreaterThan(0);
        expect(gap).toBeLessThan(20);
        expect(b.left).toBeLessThan(t.right);
        expect(b.right).toBeGreaterThan(t.left);
        await fireEvent.pointerLeave(target);
      }
    } finally {
      for (const off of offs) if (off) off();
      host.remove();
      root.style.zoom = "";
      root.style.removeProperty("--zoom");
    }
  });
});
