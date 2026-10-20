#!/usr/bin/env python3
"""Check a release notes file: every open needs-human issue ID appears in it exactly once, no
closed or unknown ID is listed as a test step for one, and every guide link points at a guide page
(and heading) that exists. Usage: scripts/check-release-notes.py [docs/releases/vX.md]. Exits 1 on problems."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
from importlib import util  # noqa: E402

spec = util.spec_from_file_location("check_guide", ROOT / "scripts" / "check-guide.py")
guide = util.module_from_spec(spec)
spec.loader.exec_module(guide)

GUIDE_URL = re.compile(r"https://github\.com/(?:brewer-corp|spadaval)/astromech/blob/main/docs/guide/([a-z0-9-]+\.md)(?:#([\w-]+))?")
ANY_GUIDE_OWNER = re.compile(r"https://github\.com/([\w.-]+)/astromech/blob/main/docs/guide/")
ID = re.compile(r"astromech-[0-9a-z]{4}\b")


def open_needs_human():
    out = set()
    for f in (ROOT / ".atelier" / "issues").glob("*.md"):
        m = re.match(r"---\n(.*?)\n---\n", f.read_text(encoding="utf-8"), re.S)
        fm = m.group(1)
        status = re.search(r'^status: "?(\w+)"?', fm, re.M).group(1)
        labels = re.findall(r'^- "([^"]+)"', (fm.split("labels:")[1:] or [""])[0].split("priority:")[0], re.M)
        if "needs-human" in labels and status not in ("done", "superseded"):
            out.add(f.stem)
    return out


def main():
    path = Path(sys.argv[1]) if len(sys.argv) > 1 else max((ROOT / "docs" / "releases").glob("v*.md"), key=lambda f: f.stat().st_mtime)
    text = path.read_text(encoding="utf-8")
    problems = []
    start = text.find("## What has not been checked")
    end = text.find("## Known limits")
    if start < 0 or end < start:
        print("missing the '## What has not been checked' or '## Known limits' heading")
        return 1
    ids = ID.findall(text[start:end])
    outside = ID.findall(text[:start] + text[end:])
    wanted = open_needs_human()
    for i in sorted(wanted):
        n = ids.count(i)
        if n != 1:
            problems.append(f"{i} appears {n} times (want 1)")
    known = {f.stem for f in (ROOT / ".atelier" / "issues").glob("*.md")}
    for i in sorted(set(ids) - known - {"astromech-core"}):
        problems.append(f"{i} is not an issue")
    for i in sorted((set(ids) & known) - wanted):
        problems.append(f"{i} is in the notes but is not an open needs-human issue")
    for page, anchor in GUIDE_URL.findall(text):
        f = ROOT / "docs" / "guide" / page
        if not f.exists():
            problems.append(f"guide page {page} does not exist")
        elif anchor and anchor not in guide.anchors(f):
            problems.append(f"guide page {page} has no heading #{anchor}")
    for owner in sorted(set(ANY_GUIDE_OWNER.findall(text)) - {"brewer-corp", "spadaval"}):
        problems.append(f"guide link points at {owner}/astromech, not brewer-corp/astromech")
    for i in sorted({i for i in outside if i in wanted}):
        problems.append(f"{i} is mentioned outside the test script")
    for p in problems:
        print(p)
    print(f"{path.name}: {len(wanted)} open needs-human issues, {len(problems)} problems")
    return 1 if problems else 0


sys.exit(main())
