#!/usr/bin/env node
/**
 * Capture pages of the browser preview at several window sizes and Text size steps, and measure
 * each capture for the checkpoint checks (astromech-irtr, astromech-8e22):
 *
 *   node scripts/browser/audit.mjs --base http://localhost:5291 --plan docs/design/increment2/plan.json
 *
 * The plan is JSON: { "out": dir, "query": "core=ready&route=exact", "sizes": [{ "w", "h", "step" }],
 * "shots": [{ "name", "hash", "query"?, "sizes"?, "steps"?: [action] }] }. An action is one of
 * { "click": css } (the first match), { "waitSel": css }, { "waitText": text },
 * { "clickText": text } (the first button or link whose own text is that, ignoring case: a
 * control with no selector of its own, such as Skip setup or a top tab), { "scrollTop": css } (scroll `main` so that element's top is at the column's top),
 * { "rows": css } (count the whole rows of that selector in view; with scrollTop, once more after it).
 * Each capture loads fresh in its own browser context, so no state (a collapsed sidebar, a row
 * gone compact) carries over. Writes <out>/<name>-<w>x<h>.png and <out>/report.json.
 *
 * Per capture, in CSS px at 100 percent Text size ("layout px") where it is a length:
 * - step: the root font size the page drew at, in percent; frame: header height and whether it
 *   stacked, and the route's form (docked sidebar, over the page, strip or rail);
 * - sideways: the document, `main` and any other element that scrolls sideways;
 * - clipped: text cut by an ancestor that hides overflow, or ended with an ellipsis;
 * - overlaps: text or controls in view drawn over each other;
 * - small: text below 12 layout px;
 * - contrast: text in view below 4.5:1 against what is drawn behind it. Listed apart as exempt:
 *   disabled controls and text under a scrim (WCAG does not count them), and text drawn in a
 *   colour DESIGN.md marks as state, not text to read (Grey Dim, Orange Dim, Placeholder);
 * - far: a row's action more than 360 layout px from the row's last text before it;
 * - bright: the brightest text colours in view and what they say, and the solid fills, for the
 *   judgement of what out-shines the page's main value (the sidebar's apart).
 * A finding is a lead to look at in the PNG, not a verdict.
 */
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { execSync } from "node:child_process";
import puppeteer from "puppeteer-core";

const CHROMES = [process.env.CHROME, "/usr/bin/google-chrome-stable", "/usr/bin/google-chrome", "/usr/bin/chromium"].filter(Boolean);

function args(argv) {
  const o = { base: null, plan: null, only: null };
  for (let i = 0; i < argv.length; i++) {
    const v = () => argv[++i];
    if (argv[i] === "--base") o.base = v();
    else if (argv[i] === "--plan") o.plan = v();
    else if (argv[i] === "--only") o.only = v();
    else throw new Error(`unknown flag ${argv[i]}`);
  }
  if (!o.base || !o.plan) throw new Error("usage: audit.mjs --base URL --plan plan.json [--only name]");
  return o;
}

/** Runs in the page: everything measured on one capture. */
function measure() {
  const step = parseFloat(getComputedStyle(document.documentElement).fontSize) / 16;
  const L = (px) => Math.round((px / step) * 10) / 10;
  const vw = innerWidth;
  const vh = innerHeight;
  const cv = document.createElement("canvas").getContext("2d", { willReadFrequently: true });
  const cache = new Map();
  /** Any CSS colour (oklch, color-mix…) as sRGB 0–255 and alpha 0–1, through a canvas. */
  const rgba = (c) => {
    if (cache.has(c)) return cache.get(c);
    cv.clearRect(0, 0, 1, 1);
    cv.fillStyle = "#000";
    cv.fillStyle = c;
    cv.fillRect(0, 0, 1, 1);
    const [r, g, b, a] = cv.getImageData(0, 0, 1, 1).data;
    const out = a === 0 ? [0, 0, 0, 0] : [r, g, b, a / 255];
    cache.set(c, out);
    return out;
  };
  const over = (top, under) => {
    const a = top[3];
    return [0, 1, 2].map((i) => top[i] * a + under[i] * (1 - a)).concat(1);
  };
  const lum = ([r, g, b]) => {
    const f = (v) => ((v /= 255) <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4);
    return 0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b);
  };
  const ratio = (a, b) => {
    const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p);
    return (x + 0.05) / (y + 0.05);
  };
  /** The colour drawn behind `el`: its ancestors' backgrounds composited, and whether an image is among them. */
  const behind = (el) => {
    const layers = [];
    let image = false;
    for (let e = el; e; e = e.parentElement) {
      const cs = getComputedStyle(e);
      if (cs.backgroundImage !== "none") image = true;
      const c = rgba(cs.backgroundColor);
      if (c[3] > 0) layers.push(c);
      if (c[3] >= 1) break;
    }
    let bg = [0, 0, 0, 1];
    for (const l of layers.reverse()) bg = over(l, bg);
    return { bg, image };
  };
  const opacity = (el) => {
    let o = 1;
    for (let e = el; e; e = e.parentElement) o *= parseFloat(getComputedStyle(e).opacity);
    return o;
  };
  const hex = (c) => "#" + c.slice(0, 3).map((v) => Math.round(v).toString(16).padStart(2, "0")).join("");
  const short = (s) => s.replace(/\s+/g, " ").trim().slice(0, 60);
  const path = (el) => {
    const parts = [];
    for (let e = el; e && e !== document.body && parts.length < 4; e = e.parentElement) {
      parts.unshift(e.tagName.toLowerCase() + (e.classList.length ? "." + [...e.classList].filter((c) => !c.startsWith("s-")).join(".") : ""));
    }
    return parts.join(" > ");
  };
  /** The part of `r` actually in view: inside the window and every ancestor that clips. */
  const inView = (el, r) => {
    let [l, t, ri, b] = [r.left, r.top, r.right, r.bottom];
    for (let e = el.parentElement; e; e = e.parentElement) {
      const cs = getComputedStyle(e);
      if (cs.overflowX === "visible" && cs.overflowY === "visible") continue;
      const q = e.getBoundingClientRect();
      if (cs.overflowX !== "visible") [l, ri] = [Math.max(l, q.left), Math.min(ri, q.right)];
      if (cs.overflowY !== "visible") [t, b] = [Math.max(t, q.top), Math.min(b, q.bottom)];
    }
    [l, t, ri, b] = [Math.max(l, 0), Math.max(t, 0), Math.min(ri, vw), Math.min(b, vh)];
    return ri - l > 1 && b - t > 1 ? { l, t, r: ri, b } : null;
  };
  const srOnly = (el) => {
    for (let e = el; e; e = e.parentElement) {
      const q = e.getBoundingClientRect();
      if (q.width <= 1 && q.height <= 1) return true;
      const cs = getComputedStyle(e);
      if (cs.clipPath !== "none" && cs.position === "absolute") return true;
    }
    return false;
  };
  /** What hides `el` at the centre of a rect: the element on top, if it is not `el`'s own. */
  const coveredBy = (el, v) => {
    const top = document.elementFromPoint((v.l + v.r) / 2, (v.t + v.b) / 2);
    if (!top || el.contains(top) || top.contains(el)) return null;
    // A label's text and its control, a cell and its own row: drawn by the same piece.
    if (top.closest("label") && top.closest("label") === el.closest("label")) return null;
    return top;
  };

  // DESIGN.md's state colours: below 4.5:1 on purpose, never text the commander needs.
  const rootCs = getComputedStyle(document.documentElement);
  const stateTokens = new Map(
    ["grey-dim", "orange-dim", "mark-off"].map((t) => {
      const c = rgba(rootCs.getPropertyValue(`--${t}`).trim());
      return [hex(c) + c[3], t];
    }),
  );

  // Text runs: each text node's lines, with its element.
  const runs = [];
  const walk = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
  for (let n = walk.nextNode(); n; n = walk.nextNode()) {
    if (!n.nodeValue.trim()) continue;
    const el = n.parentElement;
    if (!el || el.closest("script, style, noscript, svg, [hidden]")) continue;
    if (!el.checkVisibility({ opacityProperty: true, visibilityProperty: true })) continue;
    if (srOnly(el)) continue;
    const range = document.createRange();
    range.selectNodeContents(n);
    const lines = [...range.getClientRects()].filter((r) => r.width > 0.5 && r.height > 0.5);
    if (lines.length) runs.push({ el, text: n.nodeValue, lines });
  }
  // Inputs' own text: the value, or the placeholder while empty.
  const fields = [...document.querySelectorAll("input:not([type=checkbox]):not([type=radio]):not([type=range]):not([type=hidden]), textarea")]
    .filter((el) => el.checkVisibility({ opacityProperty: true, visibilityProperty: true }) && !srOnly(el))
    .map((el) => ({ el, text: el.value || el.placeholder || "", placeholder: !el.value, lines: [el.getBoundingClientRect()] }))
    .filter((f) => f.text.trim());

  const small = [];
  const clipped = [];
  const contrast = [];
  const exempt = [];
  const cutEls = new Set();
  const colours = new Map();
  const seen = new Set();
  for (const run of [...runs, ...fields]) {
    const { el } = run;
    const cs = getComputedStyle(el);
    const fs = parseFloat(cs.fontSize);
    if (L(fs) < 12 && !seen.has(el)) small.push({ text: short(run.text), px: L(fs), at: path(el) });
    // Clipped: a line past an ancestor that hides overflow, or an ellipsis that cut the text.
    if (!run.placeholder && run.lines.length) {
      const lr = run.lines;
      let cut = null;
      if (cs.textOverflow === "ellipsis" && el.scrollWidth > el.clientWidth + 1) cut = "ellipsis";
      for (let e = el; e && !cut; e = e.parentElement) {
        const ecs = getComputedStyle(e);
        const hx = ecs.overflowX === "hidden" || ecs.overflowX === "clip";
        const hy = ecs.overflowY === "hidden" || ecs.overflowY === "clip";
        // Past a scroll container the text is reached by scrolling; an ancestor's clip beyond it is moot.
        if (/auto|scroll/.test(ecs.overflowX + ecs.overflowY)) break;
        if (!hx && !hy) continue;
        const q = e.getBoundingClientRect();
        for (const r of lr) {
          // Sideways past the edge is cut; up or down, a glyph box taller than its line box
          // spills a few px without losing anything, so only a quarter of the line counts.
          const slack = (r.bottom - r.top) * 0.25;
          if (hx && (r.left < q.left - 1 || r.right > q.right + 1)) cut = `overflow ${path(e)}`;
          if (hy && (r.top < q.top - slack || r.bottom > q.bottom + slack)) cut = `overflow ${path(e)}`;
        }
      }
      if (cut) cutEls.add(el);
      if (cut && !seen.has(el)) clipped.push({ text: short(run.text), by: cut, at: path(el) });
    }
    // Contrast and brightness, on what is in view.
    const v = run.lines.map((r) => inView(el, r)).find(Boolean);
    if (!v) {
      seen.add(el);
      continue;
    }
    const cover = run.placeholder ? null : coveredBy(el, v);
    const fieldCs = run.placeholder ? getComputedStyle(el, "::placeholder") : cs;
    const fg0 = rgba(fieldCs.color);
    const { bg, image } = behind(el);
    const fg = over([fg0[0], fg0[1], fg0[2], fg0[3] * opacity(el)], bg);
    const cr = Math.round(ratio(fg, bg) * 100) / 100;
    const entry = { text: short(run.text), ratio: cr, fg: hex(fg), bg: hex(bg), px: L(fs), at: path(el) };
    const disabled = !!el.closest(":disabled, [aria-disabled=true], .disabled");
    const token = run.placeholder ? "placeholder" : stateTokens.get(hex(rgba(fieldCs.color)) + rgba(fieldCs.color)[3]);
    if (token) entry.token = token;
    const scrim = cover && cover.closest(".scrim, dialog::backdrop");
    if (cover) entry.under = path(cover);
    if (image) entry.image = true;
    if (cr < 4.5 && !seen.has(el)) (disabled || scrim || token ? exempt : contrast).push(disabled ? { ...entry, disabled } : entry);
    if (!scrim) {
      const key = hex(fg);
      const side = !!el.closest("[role=complementary][aria-label=Route]");
      const c = colours.get(key + side) ?? { colour: key, lum: Math.round(lum(fg) * 1000) / 1000, sidebar: side, weight: 0, texts: [] };
      c.weight += 1;
      const t = short(run.text);
      if (c.texts.length < 14 && !c.texts.includes(t)) c.texts.push(t);
      colours.set(key + side, c);
    }
    seen.add(el);
  }

  // Overlaps: text lines and controls in view, drawn over one another.
  // Under a scrim the page is dimmed on purpose and whatever is opened over it may cover it.
  const dimmed = (el, v) => {
    const o = coveredBy(el, v)?.closest(".scrim, .over");
    return !!o && !o.contains(el);
  };
  const ctl = "button, a[href], input, select, textarea, [role=button], [role=tab]";
  const boxes = [];
  for (const run of runs) for (const r of run.lines) {
    const v = inView(run.el, r);
    if (v && !cutEls.has(run.el) && !dimmed(run.el, v)) boxes.push({ el: run.el.closest(ctl) ?? run.el, own: run.el, v, full: r.bottom - r.top, text: short(run.text) });
  }
  for (const el of document.querySelectorAll(ctl)) {
    if (!el.checkVisibility({ opacityProperty: true, visibilityProperty: true }) || srOnly(el)) continue;
    const r = el.getBoundingClientRect();
    const v = inView(el, r);
    if (v && !dimmed(el, v)) boxes.push({ el, own: el, v, full: r.bottom - r.top, text: short(el.textContent || el.getAttribute("aria-label") || el.tagName), control: true });
  }
  boxes.sort((a, b) => a.v.t - b.v.t);
  const overlaps = [];
  for (let i = 0; i < boxes.length; i++) {
    const a = boxes[i];
    for (let j = i + 1; j < boxes.length && boxes[j].v.t < a.v.b; j++) {
      const b = boxes[j];
      if (a.el === b.el || a.el.contains(b.el) || b.el.contains(a.el)) continue;
      const w = Math.min(a.v.r, b.v.r) - Math.max(a.v.l, b.v.l);
      const h = Math.min(a.v.b, b.v.b) - Math.max(a.v.t, b.v.t);
      // Line boxes or a control's hit area sharing a sliver is leading or padding, not overlap;
      // a tight title's wrapped lines share up to a third of their glyph boxes. Measured on the
      // whole boxes: a line cut by the edge of the view is a sliver, and the leading it shares
      // with the line above would otherwise read as overlap.
      const need = Math.min(a.full, b.full) * (a.control || b.control ? 0.25 : 0.5);
      if (w > 2 && h > need && overlaps.length < 40) overlaps.push({ a: a.text, b: b.text, w: L(w), h: L(h), at: [path(a.own), path(b.own)] });
    }
  }

  // Far: in a row, an action more than 360 layout px after the row's last text before it.
  const far = [];
  for (const act of document.querySelectorAll(ctl)) {
    const row = act.parentElement?.closest("tr, [role=row], li");
    if (!row || !act.checkVisibility() || srOnly(act)) continue;
    const ar = act.getBoundingClientRect();
    if (ar.width === 0) continue;
    let last = -Infinity;
    for (const run of runs) {
      if (!row.contains(run.el) || act.contains(run.el)) continue;
      for (const r of run.lines) if (r.right <= ar.left + 1 && r.bottom > ar.top - 40 && r.top < ar.bottom + 40) last = Math.max(last, r.right);
    }
    if (last > -Infinity && L(ar.left - last) > 360) far.push({ action: short(act.textContent || act.getAttribute("aria-label") || ""), gap: L(ar.left - last), at: path(row) });
  }

  // Sideways scroll anywhere.
  const de = document.documentElement;
  const sideways = [];
  if (de.scrollWidth > de.clientWidth + 1) sideways.push({ at: "document", by: de.scrollWidth - de.clientWidth });
  for (const e of document.querySelectorAll("body *")) {
    const cs = getComputedStyle(e);
    if ((cs.overflowX === "auto" || cs.overflowX === "scroll") && e.scrollWidth > e.clientWidth + 1 && e.clientWidth > 0) sideways.push({ at: path(e), by: L(e.scrollWidth - e.clientWidth) });
  }

  // Solid fills (an opaque, coloured background behind text), and the frame's state.
  const fills = [];
  for (const e of document.querySelectorAll("body *")) {
    const cs = getComputedStyle(e);
    const c = rgba(cs.backgroundColor);
    if (c[3] < 0.9) continue;
    const [r, g, b] = c;
    if (Math.max(r, g, b) - Math.min(r, g, b) < 60) continue; // greys and near-blacks are surfaces
    const q = e.getBoundingClientRect();
    if (q.width < 4 || q.height < 4 || !inView(e, q)) continue;
    fills.push({ colour: hex(c), text: short(e.textContent || e.getAttribute("aria-label") || ""), sidebar: !!e.closest("[role=complementary][aria-label=Route]"), at: path(e) });
  }
  const header = document.querySelector("header");
  const side = document.querySelector("[role=complementary][aria-label=Route]");
  const frame = {
    header: header ? L(header.getBoundingClientRect().height) : null,
    stacked: !!header?.classList.contains("stacked"),
    route: side ? (side.closest(".over") ? "over" : "docked") : document.querySelector("nav.rail[aria-label='No route']") ? "rail" : "strip",
    sidebar: side ? L(side.getBoundingClientRect().width) : null,
    main: (() => {
      const m = document.querySelector("main");
      return m ? { w: L(m.clientWidth), h: L(m.clientHeight) } : null;
    })(),
  };
  const bright = [...colours.values()].sort((a, b) => b.lum - a.lum).slice(0, 8);
  return { step: Math.round(step * 100), frame, sideways, clipped, overlaps, small, contrast, exempt, far, bright, fills: fills.slice(0, 20) };
}

async function rowsInView(page, sel) {
  return page.evaluate((sel) => {
    const m = document.querySelector("main").getBoundingClientRect();
    const rows = [...document.querySelectorAll(sel)];
    const whole = rows.filter((r) => {
      const q = r.getBoundingClientRect();
      return q.top >= m.top - 0.5 && q.bottom <= m.bottom + 0.5;
    }).length;
    const pitch = rows.length > 1 ? rows[1].getBoundingClientRect().top - rows[0].getBoundingClientRect().top : null;
    return { whole, total: rows.length, pitch, column: m.height };
  }, sel);
}

async function capture(browser, base, plan, shot, size, outDir) {
  const ctx = await browser.createBrowserContext();
  const page = await ctx.newPage();
  await page.setViewport({ width: size.w, height: size.h, deviceScaleFactor: 1 });
  const query = [shot.query ?? plan.query, `textsize=${size.step}`].filter(Boolean).join("&");
  const url = `${base}/?${query}#/${shot.hash}`;
  await page.goto(url, { waitUntil: "networkidle0" });
  const want = (16 * size.step) / 100;
  await page.waitForFunction((px) => parseFloat(getComputedStyle(document.documentElement).fontSize) === px, {}, want);
  await page.waitForFunction(() => document.querySelector("main")?.innerText.trim().length > 0);
  const extra = {};
  for (const s of shot.steps ?? []) {
    if (s.waitSel) await page.waitForSelector(s.waitSel);
    if (s.waitText) await page.waitForFunction((t) => document.body.innerText.includes(t), {}, s.waitText);
    if (s.click) {
      await page.waitForSelector(s.click);
      await page.$eval(s.click, (el) => el.click());
    }
    if (s.clickText) {
      const find = (t) => [...document.querySelectorAll("button, a")].find((el) => el.textContent.trim().toLowerCase() === t.toLowerCase());
      await page.waitForFunction(find, {}, s.clickText);
      await page.evaluate((t) => [...document.querySelectorAll("button, a")].find((el) => el.textContent.trim().toLowerCase() === t.toLowerCase()).click(), s.clickText);
    }
    if (s.rows) extra[s.label ?? "rows"] = await rowsInView(page, s.rows);
    if (s.scrollTop)
      await page.$eval(s.scrollTop, (el) => {
        const m = document.querySelector("main");
        m.scrollTop += el.getBoundingClientRect().top - m.getBoundingClientRect().top;
      });
  }
  await page.evaluate(() => document.fonts.ready);
  // Quiet: no DOM change for two frames running after the last action.
  await page.evaluate(
    () =>
      new Promise((done) => {
        let frames = 0;
        const mo = new MutationObserver(() => (frames = 0));
        mo.observe(document.body, { subtree: true, childList: true, attributes: true, characterData: true });
        const tick = () => (++frames >= 3 ? (mo.disconnect(), done()) : requestAnimationFrame(tick));
        requestAnimationFrame(tick);
      }),
  );
  await page.mouse.move(0, size.h - 1);
  const name = `${shot.name}-${size.w}x${size.h}`;
  const file = join(outDir, `${name}.png`);
  await page.screenshot({ path: file });
  const m = await page.evaluate(measure);
  await ctx.close();
  if (m.step !== size.step) throw new Error(`${name}: drew at ${m.step} percent, wanted ${size.step}`);
  return [name, { url, file, ...m, ...extra }];
}

async function main() {
  const o = args(process.argv.slice(2));
  const plan = JSON.parse(readFileSync(o.plan, "utf8"));
  const root = resolve(dirname(o.plan), "..", "..", "..");
  const outDir = resolve(root, plan.out);
  mkdirSync(outDir, { recursive: true });
  const reportPath = join(outDir, "report.json");
  let report = {};
  if (o.only) {
    try {
      report = JSON.parse(readFileSync(reportPath, "utf8")).captures ?? {};
    } catch {}
  }
  const chrome = CHROMES.find((p) => {
    try {
      readFileSync(p);
      return true;
    } catch {
      return false;
    }
  });
  const browser = await puppeteer.launch({
    executablePath: chrome,
    userDataDir: mkdtempSync(`${tmpdir()}/astromech-audit-`),
    args: ["--no-sandbox", "--disable-dev-shm-usage", "--disable-gpu", "--hide-scrollbars", "--font-render-hinting=none"],
  });
  try {
    for (const shot of plan.shots) {
      if (o.only && shot.name !== o.only) continue;
      for (const size of shot.sizes ? plan.sizes.filter((s) => shot.sizes.includes(`${s.w}x${s.h}`)) : plan.sizes) {
        const [name, m] = await capture(browser, o.base, plan, shot, size, outDir);
        m.file = m.file.slice(root.length + 1);
        report[name] = m;
        const n = (k) => m[k].length;
        console.error(`${name}: ${m.frame.route}, header ${m.frame.header}${m.frame.stacked ? " stacked" : ""}; sideways ${n("sideways")} clipped ${n("clipped")} overlaps ${n("overlaps")} small ${n("small")} contrast ${n("contrast")} far ${n("far")}`);
      }
    }
  } finally {
    await browser.close();
  }
  const commit = execSync("git rev-parse --short HEAD", { cwd: root }).toString().trim();
  writeFileSync(reportPath, JSON.stringify({ commit, captures: report }, null, 1) + "\n");
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
