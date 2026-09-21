import os, re, sys

WIKI = r"D:\GitHub Repo\map-analyzer-custom\wiki\wiki"

files = {}
for root, dirs, names in os.walk(WIKI):
    for n in names:
        if n.endswith(".md"):
            p = os.path.join(root, n)
            rel = os.path.relpath(p, WIKI).replace("\\", "/")
            files[os.path.splitext(n)[0]] = rel

def read(p):
    with open(p, encoding="utf-8") as f:
        return f.read()

contents = {name: read(os.path.join(WIKI, rel)) for name, rel in files.items()}

issues = {"broken": [], "orphan": [], "drift_index": [], "drift_pages": [], "frontmatter": [], "cap": []}

# 1. broken wikilinks
link_re = re.compile(r"\[\[([^\]]+)\]\]")
for name, rel in files.items():
    for m in link_re.finditer(contents[name]):
        target = m.group(1).split("|")[0].split("#")[0].strip()
        if target not in files:
            issues["broken"].append(f"{rel}: [[{target}]]")

# 2/3. reachability from index (orphan + index drift pages)
seen = set()
def bfs(start):
    stack = [start]
    while stack:
        cur = stack.pop()
        if cur in seen:
            continue
        seen.add(cur)
        for m in link_re.finditer(contents[cur]):
            t = m.group(1).split("|")[0].split("#")[0].strip()
            if t in files and t not in seen:
                stack.append(t)
bfs("index")

for name, rel in files.items():
    if rel.startswith("archive/"):
        continue
    if name == "log":
        continue
    if name == "index":
        continue
    if name not in seen:
        issues["orphan"].append(rel)

# index drift: index lines resolve (already covered by broken check) + pages listed exist
idx = contents["index"]
listed = set()
for m in link_re.finditer(idx):
    t = m.group(1).split("|")[0].split("#")[0].strip()
    listed.add(t)
for t in listed:
    if t not in files:
        issues["drift_index"].append(f"index.md lists [[{t}]] but no page exists")

# 4. frontmatter: type matches folder
folder_types = {"concept": "concept", "decision": "decision", "entities": "entity", "hubs": "hub",
                "issue": "issue", "module": "module", "research": "research", "session": "session"}
for name, rel in files.items():
    txt = contents[name]
    m = re.match(r"^---\n(.*?)\n---", txt, re.S)
    if not m:
        issues["frontmatter"].append(f"{rel}: no frontmatter")
        continue
    fm = m.group(1)
    t = re.search(r"^type:\s*(.+)$", fm, re.M)
    folder = rel.split("/")[0]
    if t:
        tv = t.group(1).strip()
        if folder in folder_types and tv != folder_types[folder]:
            issues["frontmatter"].append(f"{rel}: type '{tv}' in folder '{folder}'")
    else:
        issues["frontmatter"].append(f"{rel}: no type field")
    if "status:" not in fm and folder in ("research", "session", "issue"):
        issues["frontmatter"].append(f"{rel}: missing status (folder {folder})")
    if folder == "research" and "sources:" not in fm:
        issues["frontmatter"].append(f"{rel}: research missing sources")
    if folder == "issue" and "github:" not in fm:
        issues["frontmatter"].append(f"{rel}: issue missing github")

# 5. stale
stale = [rel for name, rel in files.items() if "status: stale" in contents[name]]

# 6. page caps (>400 lines)
for name, rel in files.items():
    if contents[name].count("\n") > 400:
        issues["cap"].append(f"{rel}: {contents[name].count(chr(10))} lines")

print(f"files scanned: {len(files)}")
for k in ("broken", "drift_index", "frontmatter", "cap"):
    print(f"{k}: {len(issues[k])}")
    for i in issues[k]:
        print("  ", i)
print(f"orphan: {len(issues['orphan'])}")
for i in issues["orphan"]:
    print("  ", i)
print(f"stale: {stale}")
print("VERDICT:", "GREEN" if not any(issues.values()) else "ISSUES FOUND")
