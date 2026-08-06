#!/usr/bin/env python3
"""Find tokens that no highlights.scm pattern meaningfully highlights.

Usage:
    check_coverage.py                     # replay every grammar test corpus
    check_coverage.py cfscript            # ...one language only
    check_coverage.py cfscript foo.cfs    # check specific probe files instead

Method: parse each sample twice. `tree-sitter parse -c` enumerates every leaf token,
`tree-sitter query` enumerates the ranges the highlight patterns capture. A leaf no
capture claims is an unhighlighted token.

The subtlety is what "claims" means. A capture on a parent node is only counted when
that parent is a shallow wrapper -- see WRAPPER_MAX_DEPTH. Without that rule a single
broad pattern hides everything beneath it, which is how `(program) @document` made all
of cfml read as fully covered when it in fact had 25 gaps.
"""
import collections
import os
import re
import subprocess
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
GRAMMAR = os.path.join(REPO, "grammars", "cfml")
BUILD = os.path.join(REPO, "target", "highlight-coverage")
LANGS = ["cfml", "cfscript", "cfquery"]
EXT = {"cfml": "cfm", "cfscript": "cfs", "cfquery": "cfq"}
ANSI = re.compile(r"\x1b\[[0-9;]*m")

# Captures that paint nothing in Zed. `@document` and `@expression` are inherited from
# upstream and are not Zed theme scopes; `@spell` is always paired with a real capture.
NO_OP_CAPTURES = {"document", "expression", "spell"}
NO_OP_PREFIXES = ("definition.", "reference.", "injection.", "local.")

# --strict only. A capture on a parent node is ambiguous: `(cf_output_tag) @tag` both
# legitimately colours the tag's own `<cf` / `</cf` delimiters AND incidentally blankets
# every expression in the tag body, so a mis-scoped token inside reads as covered.
# Strict mode counts a parent capture only when the parent is a shallow wrapper -- a
# `string` or `regex` is one level deep (delimiters plus fragment) and really does
# colour its children. It surfaces mis-scopes at the cost of false positives on every
# whole-tag rule, so it is a review aid, not the baseline.
WRAPPER_MAX_DEPTH = 2


def build_parsers(langs):
    """Compile each grammar to a shared library. Always rebuild, so bumping the grammar
    revision cannot leave a stale .so behind and report a stale result."""
    os.makedirs(BUILD, exist_ok=True)
    for lang in langs:
        r = subprocess.run(["tree-sitter", "build", lang, "-o",
                            os.path.join(BUILD, f"{lang}.so")],
                           cwd=GRAMMAR, capture_output=True, text=True)
        if r.returncode:
            sys.exit(f"failed to build {lang}:\n{r.stderr}")
    return {lang: os.path.join(BUILD, f"{lang}.so") for lang in langs}


def extract_snippets(corpus_dir):
    """Pull the source half out of tree-sitter corpus files (the part above `---`)."""
    out = []
    for fn in sorted(os.listdir(corpus_dir)):
        if not fn.endswith(".txt"):
            continue
        text = open(os.path.join(corpus_dir, fn), encoding="utf-8").read()
        parts = re.split(r"(?m)^={3,}[^\n]*\n(.*?)\n={3,}[^\n]*\n", text)
        for i in range(1, len(parts) - 1, 2):
            src = re.split(r"(?m)^-{3,}\s*$", parts[i + 1])[0]
            if src.strip():
                out.append((f"{fn}:{parts[i].strip()}", src.rstrip() + "\n"))
    return out


def parse_cst(cst):
    """`tree-sitter parse -c` output -> rows of (start, end, indent, kind, named).

    Indentation is read from where the node text begins, since the range column is
    fixed width.
    """
    rows = []
    for ln in cst.splitlines():
        ln = ANSI.sub("", ln)
        m = re.match(r"^\s*(\d+):(\d+)\s+-\s+(\d+):(\d+)\s+(\s*)(.*)$", ln)
        if not m:
            continue
        sr, sc, er, ec, _, rest = m.groups()
        indent = ln.index(rest) if rest else 0
        rest = re.sub(r"^\w+: ", "", rest)          # drop any `field:` prefix
        named = not rest.startswith('"')            # anonymous nodes are quoted
        kind = rest.split(" ")[0] if named else rest.strip('"')
        kind = kind.lstrip("•")                # `-c` marks error-containing nodes
        rows.append(((int(sr), int(sc)), (int(er), int(ec)), indent, kind, named))
    return rows


def subtree_depths(rows):
    """Height of the subtree below each row. Leaves are 0."""
    parent, stack, depth = [-1] * len(rows), [], [0] * len(rows)
    for i, row in enumerate(rows):
        while stack and rows[stack[-1]][2] >= row[2]:
            stack.pop()
        parent[i] = stack[-1] if stack else -1
        stack.append(i)
    for i in reversed(range(len(rows))):
        if parent[i] >= 0:
            depth[parent[i]] = max(depth[parent[i]], depth[i] + 1)
    return depth


def leaves(rows):
    """(start, end, kind, named) for every row with no children."""
    out = []
    for i, (start, end, indent, kind, named) in enumerate(rows):
        nxt = rows[i + 1] if i + 1 < len(rows) else None
        if nxt is None or nxt[2] <= indent:
            out.append((start, end, kind, named))
    return out


def broken_ranges(rows):
    """Ranges of ERROR/MISSING nodes. Tokens *inside* a broken region are skipped too:
    a rule cannot key on a parent the parser failed to build, and highlighting
    syntactically invalid text is not a goal."""
    return [(start, end) for start, end, _, kind, _ in rows
            if kind in ("ERROR", "MISSING") or kind.startswith("UNEXPECTED")]


def wrapper_ranges(rows):
    """Ranges a capture may legitimately claim on behalf of the tokens inside them."""
    return {(start, end) for (start, end, _, _, _), d
            in zip(rows, subtree_depths(rows)) if d <= WRAPPER_MAX_DEPTH}


def captured_ranges(query_out, keep=()):
    """Ranges of captures that actually paint something.

    `keep` retains capture names that are otherwise treated as no-ops, used so
    injections.scm's @injection.content counts as covered: text handed to another
    language is highlighted by that layer, not this one.
    """
    out = []
    for name, a, b, c, d in re.findall(
            r"capture: (?:\d+ - )?([A-Za-z._]+), start: \((\d+), (\d+)\), "
            r"end: \((\d+), (\d+)\)", query_out):
        if name not in keep and (name in NO_OP_CAPTURES
                                 or name.startswith(NO_OP_PREFIXES)):
            continue
        out.append(((int(a), int(b)), (int(c), int(d))))
    return out


def slice_text(lines, start, end):
    """Source text between two (row, col) points."""
    (sr, sc), (er, ec) = start, end
    if sr >= len(lines):
        return ""
    if sr == er:
        return lines[sr][sc:ec]
    return "\n".join([lines[sr][sc:]] + lines[sr + 1:er]
                     + ([lines[er][:ec]] if er < len(lines) else []))


def uncovered(path, lang, lib, keep_errors=False, strict=False):
    """Leaves in `path` that no highlight pattern meaningfully covers."""
    def run(subcmd, *extra):
        r = subprocess.run(["tree-sitter", subcmd, *extra, "--lib-path", lib,
                            "--lang-name", lang, path],
                           cwd=GRAMMAR, capture_output=True, text=True, timeout=60)
        # Empty output would silently read as "everything is covered", so fail loud.
        if not r.stdout.strip():
            sys.exit(f"tree-sitter {subcmd} produced no output for {path}:\n{r.stderr}")
        return r.stdout

    hl = os.path.join(REPO, "languages", lang, "highlights.scm")
    inj = os.path.join(REPO, "languages", lang, "injections.scm")
    rows = parse_cst(run("parse", "-c"))
    ranges = captured_ranges(run("query", hl))
    if strict:
        allowed = wrapper_ranges(rows)
        ranges = [r for r in ranges if r in allowed]
    if os.path.exists(inj):
        ranges += captured_ranges(run("query", inj), keep=("injection.content",))

    src = open(path, encoding="utf-8").read().splitlines()
    broken = [] if keep_errors else broken_ranges(rows)
    out, total = [], 0
    for start, end, kind, named in leaves(rows):
        if any(a <= start and end <= b for a, b in broken):
            continue
        if not slice_text(src, start, end).strip():
            continue  # whitespace-only node: nothing to paint, so never a gap
        total += 1
        if not any(a <= start and end <= b for a, b in ranges):
            out.append((start, end, kind, named))
    return out, total


def check_corpus(lang, lib, strict=False):
    corpus = os.path.join(GRAMMAR, lang, "test", "corpus")
    scratch = os.path.join(BUILD, f"snippet.{EXT[lang]}")
    snippets = extract_snippets(corpus)
    misses, examples, total = collections.Counter(), {}, 0

    for name, src in snippets:
        open(scratch, "w", encoding="utf-8").write(src)
        try:
            found, n = uncovered(scratch, lang, lib, strict=strict)
        except subprocess.TimeoutExpired:
            print(f"  ! timed out on {name}")
            continue
        total += n
        lines = src.splitlines()
        for start, _, kind, named in found:
            key = ("named" if named else "anon", kind)
            misses[key] += 1
            examples.setdefault(key, f"{name} :: "
                                f"{lines[start[0]].strip()[:80] if start[0] < len(lines) else ''}")

    print(f"\n{'=' * 78}\n{lang}: {len(snippets)} snippets, {total} leaf tokens, "
          f"{sum(misses.values())} uncaptured\n{'=' * 78}")
    for (kind_of, kind), n in misses.most_common():
        print(f"  [{kind_of:5}] {kind:34} x{n:<5} e.g. {examples[(kind_of, kind)]}")
    return sum(misses.values())


def check_probes(lang, lib, paths, strict=False):
    for path in paths:
        found, _ = uncovered(path, lang, lib, keep_errors=True, strict=strict)
        print(f"--- {path} ---")
        if not found:
            print("  (fully covered)")
            continue
        src = open(path, encoding="utf-8").read().splitlines()
        for (row, col), _, kind, named in found:
            line = src[row].strip()[:70] if row < len(src) else ""
            print(f"  {row + 1}:{col:<3} {'named' if named else 'anon ':6} {kind:30} | {line}")


if __name__ == "__main__":
    args = [a for a in sys.argv[1:] if a != "--strict"]
    strict = "--strict" in sys.argv[1:]
    if args and args[0] in LANGS:
        lang, probes = args[0], args[1:]
        lib = build_parsers([lang])[lang]
        if probes:
            check_probes(lang, lib, probes, strict)
        else:
            check_corpus(lang, lib, strict)
    elif args:
        sys.exit(f"first argument must be one of {', '.join(LANGS)}")
    else:
        libs = build_parsers(LANGS)
        for lang in LANGS:
            check_corpus(lang, libs[lang], strict)
