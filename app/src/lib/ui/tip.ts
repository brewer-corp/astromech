// DESIGN.md's Tip: the window's one tooltip, in place of the native title (which takes about a
// second to appear and is small). Attach it to what it explains: `<IconButton … />` has it
// built in; anything else takes `{@attach tip("Full tank and full cargo")}`, on an element or on
// a component that spreads its props onto one (Button does). It shows at once on hover and on
// keyboard focus, swaps its content in place when the pointer moves to the next target, and goes
// when the pointer leaves, on scroll and on Escape. Screen readers get the same text through
// aria-describedby, except when it only repeats the target's aria-label.
//
// Plain is one line; rich is a title, a fact line and a note. A target that can't take focus is
// given a tab stop so the keyboard reaches it; `stop: false` leaves one out, for a mark repeated
// down a column whose text is already its accessible name (a station type icon).
import type { Attachment } from "svelte/attachments";

export type TipContent = string | { title: string; fact?: string; note?: string };
/** What `tip` takes: the content, or nothing for no tip (a condition that is off). */
export type TipArg = string | { title: string; fact?: string; note?: string; stop?: false } | null | undefined;

const GAP = 8; // between the target and the pointer's tip
const EDGE = 8; // the closest the tip comes to the window's edge

let box: HTMLDivElement | null = null;
let texts: HTMLDivElement | null = null;
let shown: Element | null = null;
let serial = 0;

function host(): HTMLDivElement {
  if (box?.isConnected) return box;
  box = document.createElement("div");
  box.className = "tip";
  box.setAttribute("role", "tooltip");
  box.setAttribute("aria-hidden", "true"); // the target's aria-describedby carries the words
  box.hidden = true;
  document.body.append(box);
  return box;
}

/** The hidden home of every target's description, so aria-describedby always has its words. */
function descriptions(): HTMLDivElement {
  if (texts?.isConnected) return texts;
  texts = document.createElement("div");
  texts.hidden = true;
  document.body.append(texts);
  return texts;
}

function fill(el: HTMLElement, content: TipContent) {
  el.replaceChildren();
  if (typeof content === "string") {
    el.classList.remove("rich");
    el.textContent = content;
    return;
  }
  el.classList.add("rich");
  const line = (cls: string, text: string | undefined) => {
    if (!text) return;
    const p = document.createElement("span");
    p.className = cls;
    p.textContent = text;
    el.append(p);
  };
  line("tip-title", content.title);
  line("tip-fact", content.fact);
  line("tip-note", content.note);
}

/** The description's text: the lines in reading order. */
const said = (c: TipContent) => (typeof c === "string" ? c : [c.title, c.fact, c.note].filter(Boolean).join(". "));

/** Above the target with the pointer at its middle; below when the top edge is too close; kept
 * inside the window. */
function place(el: HTMLElement, target: Element) {
  const t = target.getBoundingClientRect();
  el.style.left = "0px";
  el.style.top = "0px";
  const w = el.offsetWidth;
  const h = el.offsetHeight;
  const vw = document.documentElement.clientWidth;
  const below = t.top - GAP - h < EDGE;
  const left = Math.max(EDGE, Math.min(t.left + t.width / 2 - w / 2, vw - EDGE - w));
  const top = below ? t.bottom + GAP : t.top - GAP - h;
  el.classList.toggle("below", below);
  el.style.left = `${Math.round(left)}px`;
  el.style.top = `${Math.round(top)}px`;
  el.style.setProperty("--tip-x", `${Math.round(Math.max(GAP, Math.min(t.left + t.width / 2 - left, w - GAP)))}px`);
}

function show(target: Element, content: TipContent) {
  const el = host();
  fill(el, content);
  el.hidden = false;
  place(el, target);
  if (shown) return void (shown = target);
  shown = target;
  window.addEventListener("scroll", hide, { capture: true, passive: true });
  document.addEventListener("keydown", escape, true);
}

function hide() {
  if (!shown) return;
  shown = null;
  if (box) box.hidden = true;
  window.removeEventListener("scroll", hide, { capture: true });
  document.removeEventListener("keydown", escape, true);
}

function escape(e: KeyboardEvent) {
  if (e.key === "Escape") hide();
}

const FOCUSABLE = "a[href], button, input, select, textarea, summary, [tabindex], [contenteditable]";

/** A tooltip on the element (see the file's header). */
export function tip(arg: TipArg): Attachment<HTMLElement> {
  return (node) => {
    if (arg == null || arg === "") return;
    const content: TipContent = typeof arg === "string" ? arg : { title: arg.title, fact: arg.fact, note: arg.note };
    const stop = typeof arg === "string" || arg.stop !== false;

    const own: string[] = [];
    if (said(content) !== node.getAttribute("aria-label")) {
      const d = document.createElement("span");
      d.id = `tip-${++serial}`;
      d.textContent = said(content);
      descriptions().append(d);
      own.push(d.id);
      node.setAttribute("aria-describedby", [node.getAttribute("aria-describedby"), d.id].filter(Boolean).join(" "));
    }
    const added = stop && !node.matches(FOCUSABLE);
    if (added) node.tabIndex = 0;

    const enter = () => show(node, content);
    const focus = () => {
      if (node.matches(":focus-visible")) show(node, content);
    };
    const leave = () => {
      if (shown === node) hide();
    };
    node.addEventListener("pointerenter", enter);
    node.addEventListener("pointerleave", leave);
    node.addEventListener("focus", focus);
    node.addEventListener("blur", leave);
    // The content changed under the pointer or keyboard focus (a cell whose action changed when
    // clicked, a copy button that now says "Copied").
    if (node.matches(":hover, :focus-visible")) enter();

    return () => {
      node.removeEventListener("pointerenter", enter);
      node.removeEventListener("pointerleave", leave);
      node.removeEventListener("focus", focus);
      node.removeEventListener("blur", leave);
      leave();
      if (added) node.removeAttribute("tabindex");
      for (const id of own) document.getElementById(id)?.remove();
      const rest = (node.getAttribute("aria-describedby") ?? "").split(" ").filter((id) => id && !own.includes(id));
      if (rest.length) node.setAttribute("aria-describedby", rest.join(" "));
      else node.removeAttribute("aria-describedby");
    };
  };
}
