#!/usr/bin/env python3
"""Check the user guide: every relative link in README.md, CONTRIBUTING.md and docs/guide/*.md
points at a file that exists (and, with #anchor, at a heading that exists), and every guide page
is linked from docs/guide/index.md. Prints each problem; exits 1 if there are any."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GUIDE = ROOT / "docs" / "guide"
LINK = re.compile(r"(?<!\!)\[[^\]]*\]\(([^)\s]+)\)")


def slug(heading: str) -> str:
    """GitHub's heading anchor: lower case, punctuation dropped, spaces to hyphens."""
    s = re.sub(r"[`*_]", "", heading.strip().lower())
    s = re.sub(r"[^\w\- ]", "", s)
    return s.replace(" ", "-")


def anchors(path: Path) -> set[str]:
    out, seen, fence = set(), {}, False
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("```"):
            fence = not fence
        m = None if fence else re.match(r"#{1,6}\s+(.*)", line)
        if m:
            s = slug(m.group(1))
            n = seen.get(s, 0)
            seen[s] = n + 1
            out.add(s if n == 0 else f"{s}-{n}")
    return out


def links(path: Path):
    fence = False
    for no, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if line.startswith("```"):
            fence = not fence
        if fence:
            continue
        for m in LINK.finditer(re.sub(r"`[^`]*`", "", line)):
            yield no, m.group(1)


def main() -> int:
    problems = []
    pages = sorted(GUIDE.glob("*.md"))
    for page in [ROOT / "README.md", ROOT / "CONTRIBUTING.md", *pages]:
        for no, target in links(page):
            if re.match(r"[a-z][a-z0-9+.-]*:", target):
                continue  # http(s), mailto
            file, _, frag = target.partition("#")
            dest = page if not file else (page.parent / file)
            if not dest.exists():
                problems.append(f"{page.relative_to(ROOT)}:{no}: {target}: no such file")
            elif frag and dest.suffix == ".md" and frag not in anchors(dest):
                problems.append(f"{page.relative_to(ROOT)}:{no}: {target}: no such heading")
    index = {t.partition("#")[0] for _, t in links(GUIDE / "index.md")}
    for page in pages:
        if page.name != "index.md" and page.name not in index:
            problems.append(f"docs/guide/index.md does not link {page.name}")
    for p in problems:
        print(p)
    print(f"checked {len(pages)} guide pages: {len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
