---
name: highlight-coverage
description: Find tokens that no tree-sitter highlight query captures in the zed-cfml extension. Use when asked to look for missing, wrong, or unstyled syntax highlighting, when auditing languages/*/highlights.scm, after bumping the grammar revision in extension.toml, or before a release that touches .scm query files.
---

# Highlight coverage

Reading `highlights.scm` will not tell you what it misses. Measure instead. Two scripts
live beside this file; they answer different questions and you usually want both.

## check_coverage.py -- what nothing highlights

Parses a sample twice: `tree-sitter parse -c` for its leaf tokens, `tree-sitter query`
for the captured ranges. A leaf no capture claims is an unhighlighted token. Inputs come
from the grammar's own test corpus (~300 snippets), the best available sample of real
CFML.

```bash
python3 .claude/skills/highlight-coverage/check_coverage.py              # all three
python3 .claude/skills/highlight-coverage/check_coverage.py cfscript     # one language
python3 .claude/skills/highlight-coverage/check_coverage.py cfscript a.cfs b.cfs
python3 .claude/skills/highlight-coverage/check_coverage.py --strict     # see below
```

Needs the `tree-sitter` CLI on PATH. Parsers rebuild into `target/highlight-coverage/`
(gitignored) every run, so a grammar bump cannot leave a stale library reporting a stale
result.

**Baseline at the currently pinned grammar revision: 0 across all three languages.**
Anything above 0 is a regression. Update this if a grammar bump legitimately changes it.

The script already discounts three classes of false positive, which is why the baseline
is meaningful:

- **No-op captures.** `@document`, `@expression` and `@spell` are not Zed theme scopes.
  This matters enormously: `(program) @document` in cfml spans the entire file, so
  counting it made cfml report 0 uncaptured while it actually had 25 real gaps. If you
  add a capture name Zed does not theme, add it to `NO_OP_CAPTURES`.
- **Injected regions.** If `languages/<lang>/injections.scm` exists it is queried too,
  and `@injection.content` counts as covered, since another layer paints that text.
- **Whitespace-only nodes.** `implicit_cf_end_tag` and friends have no visible text.
- **Everything inside an `ERROR`/`MISSING` node,** not just the error node itself. When
  the parser fails to build a parent, no rule keyed on that parent can match, so those
  tokens are not query bugs. Note `parse -c` prefixes error-containing nodes with `•`,
  which has to be stripped before comparing the node kind.

Probe mode keeps error regions, so you can tell a real gap from a snippet that simply
does not parse. It is also how you check the mid-typing experience: an unterminated
string puts the whole statement inside an `ERROR`, and any rule with a parent constraint
stops matching. `"queryExecute" @function.builtin` in cfscript is deliberately written
without a parent for exactly this reason.

### The parent-capture limitation, and --strict

A capture on a parent node covers everything inside it. That is correct for
`(string) @string` claiming its own quotes, and misleading for `(cf_output_tag) @tag`,
which spans a whole tag *body*: a mis-scoped operator inside `<cfoutput>#a <= 1#</cfoutput>`
reads as covered, when in truth it renders tag-coloured rather than operator-coloured.

`--strict` counts a parent capture only when the parent is a shallow wrapper
(`WRAPPER_MAX_DEPTH`). It surfaces those mis-scopes -- it is how the missing `<=` in
cfml's operator list was found -- but it also reports every whole-tag rule as a false
positive, so the output needs reading rather than trusting. Use it as a review aid; keep
the default mode as the baseline.

## compare_languages.py -- what was never ported across

The three grammars share most of their rules, so a pattern in one file and absent from
another is usually an oversight. Reports only patterns whose node types all exist in the
target grammar.

```bash
python3 .claude/skills/highlight-coverage/compare_languages.py cfml cfquery
python3 .claude/skills/highlight-coverage/compare_languages.py            # all pairs
```

Two known false positives in its output, both harmless once you know them:

- cfml spells alternations as separate patterns (`value: (function_expression)` then
  `value: (arrow_function)`) where cfquery uses one `[(function_expression) (arrow_function)]`.
  The normaliser does not see those as equal.
- A node type can exist in `node-types.json` and still be unreachable. `parameter_type`,
  `catch_clause` and `import_path` all appear in the cfquery grammar, but `<cfscript>`
  is a parse error inside a cfquery body, so rules for them would be dead code.

**Always confirm with a probe file before adding a rule.** Write the construct, run
`check_coverage.py <lang> probe.<ext>`, and only add the rule if the token actually shows
up uncovered.

## Working a finding

1. Get the real node shape first. Guessing costs more than checking:
   ```bash
   tree-sitter parse -c --lib-path target/highlight-coverage/cfscript.so \
     --lang-name cfscript sample.cfs
   ```
   Run it from `grammars/cfml/`. `-c` prints anonymous nodes, which is the whole point:
   those are usually what is missing.
2. Edit `languages/<lang>/highlights.scm`, never `grammars/cfml/<lang>/queries/`. The
   latter is upstream's vendored copy and Zed does not load it. `CLAUDE.md` has the
   capture-name remap between the two.
3. Confirm the query still compiles. `tree-sitter query` errors on an unknown node type
   or field name, the fastest way to catch a typo:
   ```bash
   tree-sitter query --lib-path target/highlight-coverage/cfscript.so \
     --lang-name cfscript ../../languages/cfscript/highlights.scm sample.cfs
   ```
4. Re-run the checker. Fixing one gap often exposes another it was masking, so iterate
   until the count settles.

## What neither script checks

A token can be captured and still be wrong: anything falling through to
`(identifier) @variable` looks fine to both. Skim `tree-sitter query` output for scopes
that do not match meaning -- a type rendered as a variable, a quoted SQL identifier
rendered as a string. `CLAUDE.md` lists the grammar quirks already found this way,
including cases where the same source text has two different parses.
