# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

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
