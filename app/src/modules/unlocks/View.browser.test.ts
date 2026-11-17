import { afterEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import View from "./View.svelte";
import { answer } from "../../test/transport";
import searches from "../../preview/fixtures/engineer-search.json";

afterEach(() => {
  cleanup();
  location.hash = "";
});

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
  expect(screen.getByText(/4 of 25 unlocked/)).toBeTruthy();
  const felicity = screen.getByLabelText("Felicity Farseer");
  expect(felicity.className).toMatch(/unlocked/);
  expect(felicity.querySelectorAll(".hex.on").length).toBe(5);
  expect(within(felicity).getByRole("img", { name: /Unlocked, rank 5/ })).toBeTruthy();
  const liz = screen.getByLabelText("Liz Ryder");
  expect(liz.querySelectorAll(".hex.on").length).toBe(1);
  const lei = screen.getByLabelText("Lei Cheung");
  expect(lei.className.split(/\s+/)).toContain("locked");
  expect(lei.textContent).toMatch(/Not introduced/);
  const todNext = screen.getByLabelText('Tod "The Blaster" McQuinn');
  expect(todNext.className.split(/\s+/)).toContain("next");

  await fireEvent.click(screen.getByRole("button", { name: "Suit engineers" }));
  expect(location.hash).toBe("#/unlocks?kind=suit");
  expect(await screen.findByLabelText("Kit Fowler")).toBeTruthy();
  const kit = screen.getByLabelText("Kit Fowler");
  expect(kit.className.split(/\s+/)).toContain("invited");
  expect(kit.textContent).toMatch(/Invited/);
  expect(within(kit).getByRole("img", { name: "Invited" })).toBeTruthy();
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

it("lists every mod a suit engineer fits, with the unsure ones marked, and Colonia", async () => {
  location.hash = "#/unlocks?kind=suit";
  render(View);
  const hero = await screen.findByLabelText("Hero Ferrari");
  // From the Odyssey data (it was On foot's Engineers view until astromech-5a01).
  await waitFor(() => expect(hero.textContent).toContain("Increased Sprint Duration (?)"));
  expect(screen.getByLabelText("Baltanos").textContent).toContain(" · Colonia");
  // What Hero asks to refer Wellington Beck shows before Hero is unlocked, while Wellington's card
  // says only to unlock Hero first.
  expect(hero.className.split(/\s+/)).not.toContain("unlocked");
  expect(hero.textContent).toContain("Refers Wellington Beck: Provide 5 Settlement Defence Plans");
});

it("answers a module search with the best grade today, and clearing it brings the tree back", async () => {
  render(View);
  await screen.findByLabelText("Felicity Farseer");
  const box = screen.getByRole("textbox", { name: "Engineers for a module" });
  await fireEvent.input(box, { target: { value: "fsd" } });
  // Farseer at rank 5 gives grade 5 today; Martuuk reaches 5 but rank 2 allows 2.
  const main = await screen.findByLabelText("Main answer");
  expect(within(main).getByText("Felicity Farseer")).toBeTruthy();
  expect(within(main).getByRole("button", { name: "Set destination" })).toBeTruthy();
  const martuuk = screen.getByRole("listitem", { name: "Elvira Martuuk" });
  expect(within(martuuk).getByRole("img", { name: "Rank 2 of 5" })).toBeTruthy();
  expect(within(martuuk).getByRole("img", { name: "Grade 2 today, reaches 5" })).toBeTruthy();
  expect(martuuk.textContent).toContain("grade 5 at rank 5");
  expect(location.hash).toBe("#/unlocks?q=fsd");

  await fireEvent.click(screen.getByRole("button", { name: "Clear the search" }));
  expect(screen.queryByLabelText("Main answer")).toBeNull();
  expect(screen.getByLabelText("Felicity Farseer").tagName).toBe("ARTICLE");
  expect(location.hash).toBe("#/unlocks");
});

it("says when a word matches nothing, with near matches and the suit engineers' way", async () => {
  location.hash = "#/unlocks?q=lazer";
  render(View);
  expect(await screen.findByText(/No module type or modification called "lazer"/)).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "laser" }));
  // None unlocked: the main answer is the nearest to unlock, and its step.
  const main = await screen.findByLabelText("Main answer");
  expect(within(main).getByText("The Dweller")).toBeTruthy();
  expect(main.textContent).toContain("Invitation:");

  const box = screen.getByRole("textbox", { name: "Engineers for a module" });
  await fireEvent.input(box, { target: { value: "pistol" } });
  expect(await screen.findByText(/Suit engineers are searched by the modification you want/)).toBeTruthy();
  await fireEvent.click(screen.getByRole("button", { name: "range" }));
  expect(await screen.findByRole("listitem", { name: "Domino Green" })).toBeTruthy();
});

it("hides the last answer and its actions while the next search is on its way", async () => {
  // Held at the transport: the recorded laser reply arrives only when the test lets it.
  let release = () => {};
  answer("engineering_search", ({ query }: { query: string }) =>
    query === "laser" ? new Promise((r) => (release = () => r(searches.laser))) : searches.fsd,
  );
  location.hash = "#/unlocks?q=fsd";
  render(View);
  expect(within(await screen.findByLabelText("Main answer")).getByText("Felicity Farseer")).toBeTruthy();
  await fireEvent.input(screen.getByRole("textbox", { name: "Engineers for a module" }), { target: { value: "laser" } });
  expect(screen.queryByLabelText("Main answer")).toBeNull();
  expect(screen.queryByRole("button", { name: "Set destination" })).toBeNull();
  expect(screen.getByText("Searching the engineers…")).toBeTruthy();
  release();
  expect(within(await screen.findByLabelText("Main answer")).getByText("The Dweller")).toBeTruthy();
});

it("marks a suit mod the wiki is unsure of", async () => {
  location.hash = "#/unlocks?q=range";
  render(View);
  const rosa = await screen.findByRole("listitem", { name: "Rosa Dayette" });
  expect(rosa.textContent).toContain("Greater Range (?)");
  expect(screen.getByRole("listitem", { name: "Domino Green" }).textContent).not.toContain("(?)");
});
