# zed-cfml

CFML language support for [Zed](https://github.com/zed-industries/zed). Ships three
languages backed by [tree-sitter-cfml](https://github.com/cfmleditor/tree-sitter-cfml).

## Layout

| Path | Purpose |
| --- | --- |
| `extension.toml` | Extension manifest; pins the grammar revisions |
| `languages/<lang>/` | **The queries Zed actually loads.** Edit these. |
| `grammars/<name>/` | Vendored checkouts of the grammar repo (git-ignored build inputs) |
| `src/`, `xtask/` | Rust extension entry point and release tooling |

The three languages, by directory and Zed display name (`config.toml` → `name`):

| Directory | Display name | Grammar | Files |
| --- | --- | --- | --- |
| `languages/cfml` | `CFML (Tag)` | `cfml` | `.cfm`, `.cfc`, `.cfml` |
| `languages/cfscript` | `CFML (Script)` | `cfscript` | `.cfs` |
| `languages/cfquery` | `CFML (Query)` | `cfquery` | injection-only, no suffixes |

`cfquery` has no `path_suffixes` on purpose: it exists to be injected into `cfquery`
tag bodies and `queryExecute()` strings. Use the **display name**, not the grammar
name, in `#set! injection.language`.

## Two copies of every query file

Queries exist in two places and they are not interchangeable:

- `languages/<lang>/*.scm` — what Zed loads. Authoritative for this repo.
- `grammars/cfml/<lang>/queries/*.scm` — upstream's copies, vendored with the grammar.

They hold the **same patterns with capture names remapped** to Zed's scope vocabulary.
When pulling a fix down from upstream, apply this remap:

| Upstream (nvim-treesitter style) | Zed |
| --- | --- |
| `@doctype` | `@tag.doctype` |
| `@text` | `@text.literal` |
| `@tag.error` | `@tag` |
| `@variable.builtin` | `@variable.special` |
| `@function.method`, `@function.call`, `@function.method.call` | `@function` |
| `@comment.documentation` | `@comment.doc` |
| `@keyword.directive` | `@preproc` |
| `@string.regexp` | `@string.regex` |
| `@character.special` | `@string.special` |
| `@keyword.conditional.ternary` | `@keyword` |

`languages/cfml/highlights.scm` still carries `(program) @document`,
`(array) @expression` and `@definition.function` from upstream, and cfml and cfquery
both use `@spell`. None is a Zed theme scope, so they are no-ops: the query parses, the
token counts as captured, and it renders unstyled. That makes them invisible to
`check_coverage.py`, which is how `(access_type) @access_type` sat in cfml leaving
`public`/`private`/`remote` unstyled while the other two languages used `@keyword`.
Run `compare_languages.py --captures` to list them; don't copy the pattern into new rules.

**The three languages must keep two big rules in sync**: the CFML scopes list
(`APPLICATION|ARGUMENTS|CGI|...` → `@variable.special`) and the Lucee builtin-function
list (→ `@function.builtin`). Both are reachable in all three languages, since cfquery
embeds CFML expressions in `#...#`. cfquery had neither until they were ported across,
and cfml's builtin list had drifted 166 functions behind cfscript's. cfscript is the
canonical copy; when updating, copy from there.

All grammar checkouts under `grammars/` must sit at the same revision as the three
`[grammars.*]` entries in `extension.toml`. Verify before trusting any analysis of
node types:

```bash
for d in cfml cfscript cfquery; do echo "$d: $(git -C grammars/$d rev-parse HEAD)"; done
```

## Three copies of tasks.json

`languages/<lang>/tasks.json` is how this extension exposes `cfmleditor-lsp`'s CLI
subcommands, and it is the *only* route available: the extension API (`zed_extension_api`)
has no way to contribute command-palette entries, so a task is the one thing that can be
shipped and still land in front of the user. The server's `workspace/executeCommand`
handlers are reachable only as code actions, which is a `cfmleditor-lsp` release rather
than an extension one.

Zed loads tasks per language directory, so the file is triplicated **byte-identically** —
a CFML buffer is `CFML (Tag)`, `CFML (Script)` or `CFML (Query)` depending on the file, and
whichever one is active is the only `tasks.json` in scope. A task added to one directory
silently does not exist in the other two. Edit `languages/cfml/tasks.json`, then copy:

```bash
cp languages/cfml/tasks.json languages/cfscript/tasks.json
cp languages/cfml/tasks.json languages/cfquery/tasks.json
md5 -q languages/*/tasks.json | sort -u | wc -l   # must print 1
```

Task variables come from Zed, not from us; the useful ones here are `$ZED_FILE`,
`$ZED_ROW`, `$ZED_WORKTREE_ROOT`, `$ZED_SYMBOL` and `$ZED_SELECTED_TEXT`. `$ZED_ROW` is
what makes `explain <file> <line>` work from the cursor. Note `$ZED_SYMBOL` is the
*selected symbol* from Zed's outline context, not the word under the cursor, so it can be
empty or resolve to the enclosing function — `$ZED_SELECTED_TEXT` is the fallback when a
command needs an exact name.

`CFML: Format Current File` runs `format -w`, which writes the file on disk behind Zed's
back; with unsaved changes in the buffer the two diverge. The server implements
`textDocument/formatting`, so `editor: format` is the safer path — the task is kept for
formatting a file without opening it.

## Checking for missing highlights

Use the `highlight-coverage` skill (`.claude/skills/highlight-coverage/`). It carries
two scripts: `check_coverage.py` replays the grammar's test corpus and reports tokens no
pattern captures, and `compare_languages.py` reports rules one language has that another
never received. Reading the query files alone will not find these gaps.

Baseline as of the current pinned revision: **0 across all three languages.** Anything
above 0 is a regression.

One trap worth knowing before you trust a coverage number: a capture on a parent node
covers everything inside it. `(program) @document` at `languages/cfml/highlights.scm:2`
spans the whole file, and while it paints nothing (`@document` is not a Zed scope) it
made cfml read as fully covered when it had 25 real gaps. The script now discounts no-op
captures, injected regions and whitespace-only nodes. Whole-tag rules such as
`(cf_output_tag) @tag` still blanket their bodies; `--strict` exposes what they hide, at
the cost of noisier output.

## Grammar facts that are easy to get wrong

Learned the hard way; all verified against the pinned revision.

- **`queryExecute("...")` is not a `call_expression`.** The double-quoted form parses as
  a dedicated `query_expression` node with a `query_text` child, so the builtin-function
  `#match?` regex in `highlights.scm` does not reach it. It needs its own rule.
- **The single-quoted form is different again.** `queryExecute('...')` parses as an
  ordinary `call_expression` with a `string` argument. Only the double-quoted form
  produces `query_expression`/`query_text`.
- **`property` has two parses.** `property name="a" type="string";` yields
  `property_declaration`; `property string b;` can yield `tag_statement` instead.
  Rules keyed on only one of them will miss the other.
- **`cfml_template` needs its backtick fences on their own lines.** The inline form
  `x = ```<h1>hi</h1>```;` is a parse error, so test it multi-line.
- **`!`, `~` and Lucee's `NOT` all live inside `unary_operator`**, not as bare tokens.
  One `(unary_operator) @operator` rule covers all three.
- **Colons come from six different nodes** in cfscript: `pair`, `pair_pattern`,
  `switch_case`, `switch_default`, `labeled_statement`, `slice_expression`, plus
  namespaced `component_attribute` (`component test:displayLabel="..."`). Each needs
  its own rule.
- **cfquery quoting splits by intent.** `quoted_query_value` and
  `double_quoted_query_value` are string literals; `backtick_quoted_query_value` and
  `bracketed_query_value` are quoted *identifiers* and should be `@variable`. Capture
  the whole node for strings so the delimiters get styled too.
- **`<cfscript>` is a parse error inside a cfquery body, but that proves nothing about
  individual nodes.** The shared expression grammar still reaches cfquery through
  `#...#`, and an *expression* can be script-shaped: `#f( function( string[] v ) { … } )#`
  parses cleanly with `parameter_type` and `array_return_suffix` both present. Reasoning
  "no `<cfscript>`, therefore no script nodes" is how cfquery went without those rules.
  Probe the node you care about; do not generalise from the tag.
- **`hash_single` is not `hash_expression`.** The `#` delimiters around a bare `#var#`
  in a tag body are their own node, so a rule on `hash_expression` misses them. cfml
  only; cfscript and cfquery have `hash_empty` but not `hash_single`.
- **A parent constraint stops matching mid-typing.** An unterminated string collapses
  the whole statement into one `ERROR` node, so `(query_expression "queryExecute" ...)`
  goes dead exactly while you are typing the call. Where a token is unambiguous on its
  own, prefer an unparented rule: the anonymous `"queryExecute"` token only exists in
  `query_expression`, so a bare rule cannot over-match.

## Query ordering

`languages/cfml/highlights.scm` deliberately places `(regex "/" @punctuation.bracket)`
before the bare `"/"` in the operator list, accepting the overlap. `cfscript` mirrors
this exactly. If you change the resolution strategy, change both files together rather
than letting them drift.

## Common commands

```bash
cargo xtask update-grammar            # bump grammar revs in extension.toml to latest tag
cargo xtask update-grammar --commit   # ...to latest default-branch commit instead
cargo xtask lint                      # clippy against the wasm target
cargo xtask release 0.2.24            # version bump, build, test, commit, tag, push
```

Releases fire on `v*` tags; `.github/workflows/main.yml` pulls the body of the matching
`## [x.y.z]` section out of `CHANGELOG.md`, so add an entry under `## [Unreleased]` as
you go.
