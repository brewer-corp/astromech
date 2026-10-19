import { afterEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

afterEach(cleanup);

it("shows engineer cards with specialties and access, without column titles", async () => {
  render(View);
  expect(await screen.findByText('Tod "The Blaster" McQuinn')).toBeTruthy();
  expect(screen.queryByText("Introduced again")).toBeNull();
  expect(screen.queryByText("Common knowledge")).toBeNull();
  expect(screen.queryByText(/unlocked · rank/i)).toBeNull();
  expect(screen.getAllByText(/Multi-Cannon/).length).toBeGreaterThan(0);
  const tod = screen.getByLabelText('Tod "The Blaster" McQuinn');
  expect(tod.className).toMatch(/known/);
  expect(tod.className).not.toMatch(/unlocked/);
  expect(tod.className).not.toMatch(/locked/);
  expect(tod.textContent).toMatch(/Known · no access yet/);
  const felicity = screen.getByLabelText("Felicity Farseer");
  expect(felicity.className).toMatch(/unlocked/);
  expect(felicity.querySelectorAll(".badge").length).toBe(5);
  const liz = screen.getByLabelText("Liz Ryder");
  expect(liz.querySelectorAll(".badge").length).toBe(1);
  const lei = screen.getByLabelText("Lei Cheung");
  expect(lei.className).toMatch(/locked/);
  expect(lei.textContent).toMatch(/Not introduced/);
});

it("lights the introducing engineer on hover", async () => {
  render(View);
  const lei = await screen.findByLabelText("Lei Cheung");
  await fireEvent.mouseOver(lei);
  expect(lei.className).toMatch(/hot/);
  expect(screen.getByLabelText("The Dweller").className).toMatch(/pred/);
  await fireEvent.mouseLeave(lei);
  expect(screen.getByLabelText("The Dweller").className).not.toMatch(/pred/);
});
