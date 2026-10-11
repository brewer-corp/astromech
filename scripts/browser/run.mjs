#!/usr/bin/env node
/**
 * Drive the Vite app shell (or any URL) with headless Chrome.
 *
 *   scripts/ui --url http://127.0.0.1:5173/#/materials --wait Iron --screenshot /tmp/m.png --text
 *
 * Flags before the first action: --url --viewport WxH --timeout MS --headed
 * Actions, in order: --wait TEXT --wait-sel CSS --click CSS --click-text TEXT
 * --screenshot PATH --eval JS --text --html PATH
 *
 * Uses CHROME / PUPPETEER_EXECUTABLE_PATH, else a typical Google Chrome path.
 * JSON result on stdout; progress on stderr.
 */
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import puppeteer from "puppeteer-core";

const CHROME_CANDIDATES = [
  process.env.CHROME,
  process.env.PUPPETEER_EXECUTABLE_PATH,
  "/usr/bin/google-chrome-stable",
  "/usr/bin/google-chrome",
  "/usr/bin/chromium",
  "/usr/bin/chromium-browser",
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
  "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
].filter(Boolean);

function die(msg, code = 1) {
  console.error(msg);
  process.exit(code);
}

function findChrome() {
  return CHROME_CANDIDATES.find((p) => existsSync(p));
}

function parseArgs(argv) {
  const opts = {
    url: null,
    viewport: { width: 1280, height: 800 },
    timeout: 20000,
    headed: false,
    actions: [],
  };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    const next = () => {
      const v = argv[++i];
      if (v === undefined || v.startsWith("--")) die(`flag ${a} needs a value`);
      return v;
    };
    switch (a) {
      case "--url":
        opts.url = next();
        break;
      case "--viewport": {
        const [w, h] = next().split("x").map(Number);
        if (!w || !h) die("--viewport wants WxH, e.g. 1280x800");
        opts.viewport = { width: w, height: h };
        break;
      }
      case "--timeout":
        opts.timeout = Number(next());
        break;
      case "--headed":
        opts.headed = true;
        break;
      case "--wait":
        opts.actions.push({ kind: "wait", text: next() });
        break;
      case "--wait-sel":
        opts.actions.push({ kind: "wait-sel", sel: next() });
        break;
      case "--click":
        opts.actions.push({ kind: "click", sel: next() });
        break;
      case "--click-text":
        opts.actions.push({ kind: "click-text", text: next() });
        break;
      case "--screenshot":
        opts.actions.push({ kind: "screenshot", path: resolve(next()) });
        break;
      case "--eval":
        opts.actions.push({ kind: "eval", js: next() });
        break;
      case "--text":
        opts.actions.push({ kind: "text" });
        break;
      case "--html":
        opts.actions.push({ kind: "html", path: resolve(next()) });
        break;
      case "-h":
      case "--help":
        opts.help = true;
        break;
      default:
        die(`unknown flag: ${a}`);
    }
  }
  return opts;
}

function norm(s) {
  return s.replace(/\s+/g, " ").trim().toLowerCase();
}

async function waitText(page, text, timeout) {
  await page.waitForFunction(
    (t) => {
      const body = document.body && document.body.innerText;
      return !!(body && body.replace(/\s+/g, " ").toLowerCase().includes(t));
    },
    { timeout },
    norm(text),
  );
}

async function clickText(page, text) {
  const found = await page.evaluate((t) => {
    const nrm = (s) => s.replace(/\s+/g, " ").trim().toLowerCase();
    const nodes = [...document.querySelectorAll("button, a, [role='button'], [role='tab']")];
    const el = nodes.find((n) => nrm(n.textContent) === t) ?? nodes.find((n) => nrm(n.textContent).includes(t));
    if (!el) return false;
    el.click();
    return true;
  }, norm(text));
  if (!found) throw new Error(`no clickable element with text ${JSON.stringify(text)}`);
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  if (opts.help || !opts.url) {
    console.error(
      "usage: scripts/ui --url URL [--viewport 1280x800] [--timeout 20000] [--headed]\n" +
        "       [--wait TEXT] [--wait-sel CSS] [--click CSS] [--click-text TEXT]\n" +
        "       [--screenshot PATH] [--eval JS] [--text] [--html PATH]",
    );
    process.exit(opts.help ? 0 : 2);
  }

  const chrome = findChrome();
  if (!chrome) {
    die(
      "Chrome not found. Install Google Chrome, or set CHROME to the binary.\n" +
        "On Debian/Ubuntu: sudo apt-get install ./google-chrome-stable_current_amd64.deb",
    );
  }

  const userDataDir = mkdtempSync(`${tmpdir()}/astromech-chrome-`);
  const result = {
    ok: true,
    chrome,
    url: opts.url,
    finalUrl: null,
    title: null,
    screenshots: [],
  };

  const browser = await puppeteer.launch({
    headless: opts.headed ? false : true,
    executablePath: chrome,
    userDataDir,
    args: [
      "--no-sandbox",
      "--disable-dev-shm-usage",
      "--disable-gpu",
      "--hide-scrollbars",
      "--font-render-hinting=none",
    ],
    defaultViewport: opts.viewport,
  });

  try {
    const page = await browser.newPage();
    page.setDefaultTimeout(opts.timeout);
    console.error(`open ${opts.url}`);
    await page.goto(opts.url, { waitUntil: "domcontentloaded", timeout: opts.timeout });

    for (const act of opts.actions) {
      switch (act.kind) {
        case "wait":
          console.error(`wait ${JSON.stringify(act.text)}`);
          await waitText(page, act.text, opts.timeout);
          break;
        case "wait-sel":
          console.error(`wait-sel ${act.sel}`);
          await page.waitForSelector(act.sel, { timeout: opts.timeout });
          break;
        case "click":
          console.error(`click ${act.sel}`);
          await page.click(act.sel);
          break;
        case "click-text":
          console.error(`click-text ${JSON.stringify(act.text)}`);
          await waitText(page, act.text, opts.timeout);
          await clickText(page, act.text);
          break;
        case "screenshot":
          console.error(`screenshot ${act.path}`);
          await page.screenshot({ path: act.path, fullPage: true });
          result.screenshots.push(act.path);
          break;
        case "eval":
          result.eval = await page.evaluate(act.js);
          break;
        case "text":
          result.text = await page.evaluate(() => document.body.innerText);
          break;
        case "html":
          writeFileSync(act.path, await page.content());
          result.html = act.path;
          break;
        default:
          throw new Error(`unknown action ${act.kind}`);
      }
    }

    result.finalUrl = page.url();
    result.title = await page.title();
    console.log(JSON.stringify(result, null, 2));
  } catch (e) {
    result.ok = false;
    result.error = String(e && e.message ? e.message : e);
    try {
      const pages = await browser.pages();
      const page = pages.at(-1);
      if (page) {
        result.finalUrl = page.url();
        result.title = await page.title();
        result.text = await page.evaluate(() => document.body && document.body.innerText);
      }
    } catch {
      /* keep the original error */
    }
    console.log(JSON.stringify(result, null, 2));
    throw e;
  } finally {
    await browser.close();
    rmSync(userDataDir, { recursive: true, force: true });
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
