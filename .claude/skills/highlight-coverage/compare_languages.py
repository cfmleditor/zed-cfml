#!/usr/bin/env python3
"""Find highlight patterns one language has and another is missing.

Usage:
    compare_languages.py cfml cfquery     # patterns in cfml but not cfquery
    compare_languages.py                  # all six ordered pairs

The three grammars share most of their rules, so a pattern present in one file and
absent from another is usually an oversight rather than a decision. Only patterns whose
referenced node types all exist in the target grammar are reported, so genuinely
language-specific rules (cdata_section, query_keyword, ...) are filtered out.

This complements check_coverage.py: coverage finds tokens nothing highlights, this
finds rules that were never ported across. Neither proves a gap is real -- a node type
can exist in a grammar and still be unreachable. Confirm with a probe file before
adding a rule, or you will add dead code.
"""
import itertools
import json
import os
import re
import sys

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
LANGS = ["cfml", "cfscript", "cfquery"]
# Predicates and directives, not node types.
NOT_NODES = {"match", "eq", "any-of", "not-any-of", "is", "is-not", "set", "lua-match"}


def patterns(lang):
    """Split a highlights.scm into top-level patterns: (line number, source text)."""
    lines = open(f"{REPO}/languages/{lang}/highlights.scm").read().splitlines()
    out, buf, depth, start = [], [], 0, 0
    for i, ln in enumerate(lines, 1):
        stripped = ln.strip()
        if not buf and (not stripped or stripped.startswith(";")):
            continue
        if not buf:
            start = i
        buf.append(ln)
        # Ignore brackets inside string literals and inside #match? regexes.
        bare = re.sub(r'"(?:[^"\\]|\\.)*"', '""', ln)
        bare = re.sub(r"#\w+\?[^)]*", "", bare)
        depth += bare.count("(") + bare.count("[") - bare.count(")") - bare.count("]")
        if depth <= 0:
            out.append((start, "\n".join(buf)))
            buf, depth = [], 0
    if buf:
        out.append((start, "\n".join(buf)))
    return out


def grammar_nodes(lang):
    types = json.load(open(f"{REPO}/grammars/cfml/{lang}/src/node-types.json"))
    return ({t["type"] for t in types if t.get("named")},
            {t["type"] for t in types if not t.get("named")})


def referenced(text):
    """Node types a pattern mentions, named and anonymous."""
    body = re.sub(r"(?m);.*$", "", text)
    named = set(re.findall(r"\(\s*([a-z_][a-z0-9_]*)\b", body)) - NOT_NODES
    return named, set(re.findall(r'"((?:[^"\\]|\\.)*)"', body))


def normalise(text):
    """Key for equality: strip comments, capture names and all whitespace, so the same
    rule written across several lines still matches."""
    body = re.sub(r"(?m);.*$", "", text)
    return re.sub(r"\s+", "", re.sub(r"@[a-z._]+", "@", body))


def compare(base, other):
    have = {normalise(t) for _, t in patterns(other)}
    named, anon = grammar_nodes(other)
    hits = []
    for line, text in patterns(base):
        if normalise(text) in have:
            continue
        want_named, want_anon = referenced(text)
        if want_named - named:
            continue  # references a node type the other grammar does not have
        if {a for a in want_anon if a not in anon and a.replace("\\", "") not in anon}:
            continue
        hits.append((line, text))

    print(f"\n{'=' * 78}\nin {base}, not in {other} ({len(hits)} patterns)\n{'=' * 78}")
    for line, text in hits:
        print(f"--- languages/{base}/highlights.scm:{line}")
        print("\n".join("    " + l for l in text.splitlines()))
    if not hits:
        print("  (none)")
    return hits


if __name__ == "__main__":
    args = sys.argv[1:]
    if len(args) == 2:
        compare(*args)
    elif args:
        sys.exit("usage: compare_languages.py [<base> <other>]")
    else:
        for a, b in itertools.permutations(LANGS, 2):
            compare(a, b)
