#!/usr/bin/env python3
"""Mechanical wiki-lint: wikilinks, orphans, index drift, frontmatter, page caps.
Reader-agent raw-findings script. Reports only; no fixes.
"""
import os, re, sys, json
from pathlib import Path

WIKI = Path(r"D:\GitHub Repo\map-analyzer-custom\wiki\wiki")

# ---------------- collect pages ----------------
pages = {}  # basename (case-insensitive key) -> path
all_files = []
for p in WIKI.rglob("*.md"):
    all_files.append(p)
    pages.setdefault(p.stem.lower(), []).append(p)

# ---------------- extract wikilinks ----------------
LINK_RE = re.compile(r"\[\[([^\[\]]+?)\]\]")
def extract_links(text):
    out = []
    for m in LINK_RE.finditer(text):
        raw = m.group(1)
        # strip display-text forms: [[page|text]] or [[page#anchor]]
        name = raw.split("|")[0].split("#")[0].strip()
        out.append((raw, name))
    return out

# ---------------- log.md [gap] names ----------------
log_text = (WIKI / "log.md").read_text(encoding="utf-8", errors="replace")
gap_names = set()
for m in re.finditer(r"\[gap\]", log_text, re.IGNORECASE):
    # capture nearby [[name]] on the same line, and quoted names
    line_start = log_text.rfind("\n", 0, m.start()) + 1
    line_end = log_text.find("\n", m.start())
    if line_end == -1: line_end = len(log_text)
    line = log_text[line_start:line_end]
    for raw, name in extract_links(line):
        gap_names.add(name.lower())
    for q in re.findall(r"`([^`]+)`", line):
        gap_names.add(q.lower())

print("== GAP NAMES from log.md ==")
for g in sorted(gap_names):
    print(" ", g)

# ---------------- check 1: broken wikilinks ----------------
print("\n== CHECK 1: BROKEN WIKILINKS ==")
broken = []
for p in all_files:
    if p.name == "log.md":
        continue  # log exempt? no - links in log still checked, but gaps are pending
    text = p.read_text(encoding="utf-8", errors="replace")
    for raw, name in extract_links(text):
        if name.lower() not in pages:
            if name.lower() in gap_names:
                print(f"  PENDING (gap): {p}  [[{raw}]]")
            else:
                broken.append((p, raw))
for p, raw in broken:
    print(f"  BROKEN: {p}  [[{raw}]]")
print(f"  total broken: {len(broken)}")

# ---------------- check 2: orphans ----------------
print("\n== CHECK 2: ORPHANS ==")
# reachability from index.md and hub pages via wikilinks
reachable = set()
def crawl(start):
    stack = [start]
    while stack:
        cur = stack.pop()
        if cur in reachable: continue
        reachable.add(cur)
        text = cur.read_text(encoding="utf-8", errors="replace")
        for raw, name in extract_links(text):
            if name.lower() in pages:
                for cand in pages[name.lower()]:
                    if cand not in reachable:
                        stack.append(cand)
crawl(WIKI / "index.md")
for p in all_files:
    if p.parent.name == "hubs":
        crawl(p)

for p in sorted(all_files):
    if p == WIKI / "index.md" or p == WIKI / "log.md": continue
    if p.parent.name == "archive": continue
    if p not in reachable:
        print(f"  ORPHAN: {p}")
print("  (done)")

# ---------------- check 3: index drift ----------------
print("\n== CHECK 3: INDEX DRIFT ==")
index_text = (WIKI / "index.md").read_text(encoding="utf-8", errors="replace")
index_links = set()
for raw, name in extract_links(index_text):
    index_links.add(name.lower())
listed_but_missing = [n for n in sorted(index_links) if n not in pages and n != "log"]
for n in listed_but_missing:
    print(f"  LISTED BUT MISSING: {n}")
existing_not_listed = []
for p in sorted(all_files):
    if p == WIKI / "index.md" or p == WIKI / "log.md": continue
    if p.stem.lower() not in index_links:
        existing_not_listed.append(p)
for p in existing_not_listed:
    print(f"  EXISTS BUT NOT LISTED: {p}")

# ---------------- check 4: frontmatter ----------------
print("\n== CHECK 4: FRONTMATTER ==")
FM_RE = re.compile(r"^---\n(.*?)\n---", re.DOTALL)
def parse_fm(text):
    m = FM_RE.match(text)
    if not m: return None
    fm = {}
    for line in m.group(1).splitlines():
        if ":" in line:
            k, v = line.split(":", 1)
            fm[k.strip()] = v.strip()
    return fm

allowed_types = {"entity","concept","module","research","decision","issue","hub","session"}
for p in sorted(all_files):
    rel = p.relative_to(WIKI)
    folder = rel.parts[0]
    text = p.read_text(encoding="utf-8", errors="replace")
    fm = parse_fm(text)
    if fm is None:
        if p.name in ("index.md","log.md","overview.md"):
            print(f"  {rel}: no frontmatter (root page - ok per SCHEMA? SCHEMA silent)")
        else:
            print(f"  {rel}: NO FRONTMATTER")
        continue
    t = fm.get("type")
    problems = []
    if t is None:
        problems.append("missing type")
    elif t not in allowed_types:
        problems.append(f"invalid type {t!r}")
    elif t != folder.rstrip('s') and not (folder == "hubs" and t == "hub") and not (folder == "archive"):
        # entity/decision/module/research/concept/issue/session folders
        if t == "issue" and folder == "issue": pass
        elif folder == "archive": pass
        else:
            problems.append(f"type {t!r} vs folder {folder!r}")
    if t == "entity" and "updated" not in fm: problems.append("entity missing updated")
    if t == "module":
        if "updated" not in fm: problems.append("module missing updated")
        if "status" not in fm: problems.append("module missing status")
        elif fm["status"] not in ("stable","stale"): problems.append(f"module status {fm['status']!r}")
    if t == "research":
        if "sources" not in fm: problems.append("research missing sources")
        if "status" not in fm: problems.append("research missing status")
        else:
            s = fm["status"]
            base = re.sub(r"\s*\((deferred|abandoned)\)\s*$", "", s).strip()
            if base not in ("idea","designed","prototyped","implemented"):
                problems.append(f"research status {s!r} (base {base!r})")
    if t == "decision":
        if "status" not in fm: problems.append("decision missing status")
        elif fm["status"] not in ("accepted","superseded"): problems.append(f"decision status {fm['status']!r}")
    if t == "issue":
        if "github" not in fm: problems.append("issue missing github")
        if "status" not in fm: problems.append("issue missing status")
        elif fm["status"] not in ("open","in_progress","closed"): problems.append(f"issue status {fm['status']!r}")
    if t == "hub":
        if "area" not in fm: problems.append("hub missing area")
    if t == "session":
        if "status" not in fm: problems.append("session missing status")
        elif fm["status"] not in ("open","resolved"): problems.append(f"session status {fm['status']!r}")
    if problems:
        print(f"  {rel}: {fm}")
        for pr in problems:
            print(f"      -> {pr}")

# ---------------- check 5: stale / superseded ----------------
print("\n== CHECK 5: STALE / SUPERSEDED ==")
for p in sorted(all_files):
    text = p.read_text(encoding="utf-8", errors="replace")
    fm = parse_fm(text) or {}
    if fm.get("status") == "stale":
        print(f"  STALE: {p.relative_to(WIKI)}")
    if fm.get("status") == "superseded":
        print(f"  SUPERSEDED: {p.relative_to(WIKI)}")

# ---------------- check 6: page caps ----------------
print("\n== CHECK 6: PAGE CAPS ==")
for p in sorted(all_files):
    lines = p.read_text(encoding="utf-8", errors="replace").splitlines()
    raw = len(lines)
    nonempty = sum(1 for l in lines if l.strip())
    if raw > 380:
        print(f"  {p.relative_to(WIKI)}: raw={raw} nonempty={nonempty}")

# ---------------- summary ----------------
print("\n== SUMMARY ==")
print(f"pages: {len(all_files)}")
