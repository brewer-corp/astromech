import { afterEach, expect, it } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import View from "./View.svelte";
import { answer } from "../../test/transport";
import searches from "../../preview/fixtures/engineer-search.json";

afterEach(() => {
  cleanup();
  location.hash = "";
});

/** A node of the tree, by engineer. */
const node = (name: string) => screen.getByRole("button", { name });
const panel = () => screen.getByRole("complementary", { name: "Details" });

it("shows both trees with each engineer's standing, counted in the summary", async () => {
  render(View);
  const ship = await screen.findByRole("region", { name: "Ship engineers" });
  const suit = screen.getByRole("region", { name: "Suit engineers" });
  expect(within(ship).getByRole("button", { name: 'Tod "The Blaster" McQuinn' })).toBeTruthy();
  expect(within(suit).getByRole("button", { name: "Kit Fowler" })).toBeTruthy();
  expect(screen.getByText(/4 of 25 ship and 2 of 13 suit engineers unlocked · 15 to work on/)).toBeTruthy();
  // Unlocked: the rank as lit hexagons and the system; a suit engineer, who has no ranks, one.
  const felicity = node("Felicity Farseer");
  expect(felicity.querySelectorAll(".hex.on").length).toBe(5);
  expect(felicity.textContent).toContain("Deciat");
  expect(node("Liz Ryder").querySelectorAll(".hex.on").length).toBe(1);
  expect(node("Domino Green").querySelectorAll(".hex").length).toBe(1);
  // One the commander can work on now: the step left.
  expect(node('Tod "The Blaster" McQuinn').textContent).toContain("Invitation: Earn 15 bounty vouchers.");
  expect(node("Kit Fowler").textContent).toContain("Visit them at Capoya to unlock.");
  // Not introduced yet.
  expect(node("Lei Cheung").textContent).toContain("Not introduced");
});

it("opens from the old On foot link", async () => {
  location.hash = "#/unlocks?kind=suit";
  render(View);
  expect(await screen.findByRole("region", { name: "Suit engineers" })).toBeTruthy();
});

it("lights the introducing engineer when one is selected", async () => {
  render(View);
  const lei = await screen.findByRole("button", { name: "Lei Cheung" });
  expect(screen.queryByRole("complementary", { name: "Details" })).toBeNull();
  await fireEvent.click(lei);
  expect(lei.getAttribute("aria-pressed")).toBe("true");
  expect(node("The Dweller").className).toMatch(/pred/);
  await fireEvent.click(node("Marsha Hicks"));
  expect(node("The Dweller").className).toMatch(/pred/);
  await fireEvent.click(node("Felicity Farseer"));
  expect(node("The Dweller").className).not.toMatch(/pred/);
});

it("lists every mod a suit engineer fits in the panel, with the unsure ones marked, and Colonia", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "Hero Ferrari" }));
  const works = within(panel()).getByRole("list", { name: "Works on" });
  expect(works.textContent).toMatch(/Increased Sprint Duration\s\(\?\)/);
  // What Hero asks to refer Wellington Beck shows before Hero is unlocked.
  await waitFor(() => expect(panel().textContent).toContain("Referral task: Provide 5 Settlement Defence Plans"));
  expect(panel().textContent).toContain("Wellington Beck");
  await fireEvent.click(node("Baltanos"));
  await waitFor(() => expect(panel().textContent).toContain("Colonia"));
});

it("describes the selected engineer: steps to unlock, what they work on and the grades, who they introduce", async () => {
  render(View);
  await fireEvent.click(await screen.findByRole("button", { name: "The Dweller" }));
  const p = panel();
  expect(within(p).getByRole("heading", { name: "The Dweller" })).toBeTruthy();
  const steps = [...p.querySelectorAll("ol li")].map((li) => li.textContent);
  expect(steps).toEqual(["Invitation: Deal in stolen or illicit goods at 5 Black Markets.", "Unlock at Wyrd: Pay 500,000 CR."]);
  // Not unlocked: no grade today, the grade each type reaches.
  const works = within(p).getByRole("list", { name: "Works on" });
  expect(within(works).getByRole("img", { name: "Reaches grade 5" })).toBeTruthy();
  expect(within(works).getAllByRole("img", { name: "Reaches grade 3" }).length).toBe(2);
  expect(p.textContent).toContain("Lei Cheung and Marsha Hicks");
  // From Shinrarta Dezhra, the recorded commander's system.
  expect(p.textContent).toMatch(/75\.5 ly/);
  expect(within(p).getByRole("button", { name: "Set destination" })).toBeTruthy();
  expect(within(p).getByRole("button", { name: "Add stop" })).toBeTruthy();

  // Unlocked at rank 2: the grade Martuuk gives today, and the distance as the main value.
  await fireEvent.click(node("Elvira Martuuk"));
  expect(within(panel()).getByRole("img", { name: "Grade 2 today, reaches 5" })).toBeTruthy();
  expect(panel().querySelector("ol")).toBeNull();
});

it("answers a module search with the best grade today, and clearing it brings the tree back", async () => {
  render(View);
  await screen.findByRole("button", { name: "Felicity Farseer" });
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
  expect(await screen.findByRole("region", { name: "Ship engineers" })).toBeTruthy();
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
