# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- SQL and tag injections for cfscript (`queryExecute("...")` bodies and ` ``` ` template blocks)
- `cf_comment` highlighting, overrides, and text objects for cfscript

### Fixed

- cfscript: unary operators (`!`, `~`, `NOT`), shift and Lucee inequality operators (`<<`, `>>`, `>>>`, `<>`)
- cfscript: `abstract`, `final`, `interface`, `property` and `required` keywords
- cfscript: colons in switch cases, struct pairs, slices, labels and namespaced component attributes
- cfscript: `queryExecute` builtin, regex pattern/flag scopes, docblock comments, destructured property names
- cfquery: quote delimiters on quoted values; backtick- and bracket-quoted identifiers
- cfquery: `cfoutput`/`cfreturn` tag delimiters and spread operators
- cfml: `#var#` hash delimiters in tag bodies, `cfsavecontent` tag delimiters, CDATA sections
- cfml and cfquery: `<=`, `>=`, `<>`, `<<`, `>>` and `>>>` operators
- All three: destructured property names; `default`/`in` keywords in cfquery
- cfscript: removed the `target` keyword, which no longer exists in the grammar

## [0.2.23]

- Update tree-sitter-grammar and .scm files

## [0.2.22]

- Update `tree-sitter-cfml` grammar

## [0.2.21]

### Added

- Text objects (`textobjects.scm`) for Vim-style selection in all languages
- Scope overrides (`overrides.scm`) to suppress autocomplete in comments and strings
- `cargo xtask lint` standalone command
- `--dry-run` flag for `cargo xtask release`
- CI workflow to create GitHub Release with changelog notes on tag push
- Old LSP version cleanup after downloading a new version

### Changed

- Refined indentation rules for cfscript and cfml
- Release command now checks for uncommitted changes, existing tags, version ordering, and remote sync
- Release command prompts for confirmation before proceeding

### Fixed

- Clippy lint warnings in `src/lib.rs`

## [0.2.20]

- Update tree-sitter-cfml grammar
- Update folding and indentation

## [0.2.19]

- Update to latest grammars

## [0.2.18]

### Added

- `cargo xtask release` command with changelog promotion
- `cargo xtask update-grammar` command

### Changed

- Update tree-sitter-cfml grammar

## [0.2.17]

### Changed

- Updated tree-sitter-cfml grammar

## [0.2.16]

### Changed

- Updated tree-sitter-cfml grammar
