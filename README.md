# zed-cfml

CFML language support for [Zed](https://github.com/zed-industries/zed).

## Development

### Updating Grammars

Update the tree-sitter grammar revisions in `extension.toml` to the latest tagged release:

```bash
cargo xtask update-grammar
```

To use the latest commit on the default branch instead:

```bash
cargo xtask update-grammar --commit
```

### Linting

Run clippy against the wasm target:

```bash
cargo xtask lint
```

### Releasing

Create a release (updates versions, builds, tests, commits, tags, and pushes):

```bash
cargo xtask release 0.2.18
```