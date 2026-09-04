use serde::Deserialize;
use std::{env, fs, path::PathBuf, process};
use toml_edit::DocumentMut;

#[derive(Deserialize)]
struct Tag {
    commit: TagCommit,
    name: String,
}

#[derive(Deserialize)]
struct TagCommit {
    sha: String,
}

#[derive(Deserialize)]
struct Commit {
    sha: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("update-grammar") => {
            update_grammar(args.get(2).map(|s| s.as_str()) == Some("--commit"))
        }
        Some("lint") => lint(),
        Some("gen-tasks") => gen_tasks(args.iter().any(|a| a == "--check")),
        Some("release") => {
            let version = args.get(2).unwrap_or_else(|| {
                eprintln!("Usage: cargo xtask release <version>");
                eprintln!("  e.g. cargo xtask release 0.2.18");
                process::exit(1);
            });
            let dry_run = args.iter().any(|a| a == "--dry-run");
            release(version, dry_run);
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  cargo xtask update-grammar [--commit]");
            eprintln!("  cargo xtask lint");
            eprintln!("  cargo xtask gen-tasks [--check]");
            eprintln!("  cargo xtask release <version> [--dry-run]");
            process::exit(1);
        }
    }
}

fn update_grammar(use_commit: bool) {
    let toml_path = workspace_root().join("extension.toml");
    let content = fs::read_to_string(&toml_path).expect("failed to read extension.toml");
    let mut doc = content.parse::<DocumentMut>().expect("invalid TOML");

    let grammars = doc["grammars"].as_table().expect("no [grammars] table");
    let repos: Vec<(String, String)> = grammars
        .iter()
        .map(|(name, item)| {
            let repo = item["repository"].as_str().unwrap().to_string();
            (name.to_string(), repo)
        })
        .collect();

    for (name, repo_url) in &repos {
        let owner_repo = repo_url.trim_start_matches("https://github.com/");
        let new_rev = resolve_rev(owner_repo, use_commit);
        doc["grammars"][name.as_str()]["rev"] = toml_edit::value(&new_rev);
        println!("{name}: {new_rev}");
    }

    fs::write(&toml_path, doc.to_string()).expect("failed to write extension.toml");
    println!("\nUpdated {}", toml_path.display());

    // Sync query files from tree-sitter-cfml at the new rev
    let cfml_rev = doc["grammars"]["cfml"]["rev"].as_str().unwrap().to_string();
    sync_queries(&cfml_rev);
}

fn sync_queries(rev: &str) {
    let root = workspace_root();
    let base_url = format!("https://raw.githubusercontent.com/cfmleditor/tree-sitter-cfml/{rev}");

    // Map: (grammar, source query file, destination file) → destination in languages/
    let copies: &[(&str, &str, &str)] = &[
        ("cfml", "highlights.scm", "highlights.scm"),
        ("cfml", "indents-zed.scm", "indents.scm"),
        ("cfml", "brackets-zed.scm", "brackets.scm"),
        ("cfml", "injections.scm", "injections.scm"),
        ("cfml", "outline.scm", "outline.scm"),
        ("cfml", "overrides.scm", "overrides.scm"),
        ("cfml", "textobjects.scm", "textobjects.scm"),
        ("cfscript", "highlights.scm", "highlights.scm"),
        ("cfscript", "indents-zed.scm", "indents.scm"),
        ("cfscript", "brackets-zed.scm", "brackets.scm"),
        ("cfscript", "outline.scm", "outline.scm"),
        ("cfscript", "overrides.scm", "overrides.scm"),
        ("cfscript", "textobjects.scm", "textobjects.scm"),
        ("cfquery", "highlights.scm", "highlights.scm"),
        ("cfquery", "indents-zed.scm", "indents.scm"),
        ("cfquery", "brackets-zed.scm", "brackets.scm"),
        ("cfquery", "outline.scm", "outline.scm"),
        ("cfquery", "overrides.scm", "overrides.scm"),
        ("cfquery", "textobjects.scm", "textobjects.scm"),
    ];

    for (grammar, src_file, dest_file) in copies {
        let url = format!("{base_url}/{grammar}/queries/{src_file}");
        let Some(content) = try_fetch_text(&url) else {
            println!("  skipped {grammar}/{src_file} (not found upstream)");
            continue;
        };

        let dest = root.join("languages").join(grammar).join(dest_file);
        let output = if *dest_file == "injections.scm" {
            convert_injection_languages(&content)
        } else {
            content
        };
        let output = output.replace("@dedent", "@outdent");
        let output = output.replace("@indent.begin", "@indent");
        let output = output.replace("@indent.end", "@end");
        // Convert highlight captures to Zed-compatible names
        let output = output.replace("@comment.documentation", "@comment.doc");
        let output = output.replace("@string.regexp", "@string.regex");
        let output = output.replace("@variable.builtin", "@variable.special");
        let output = output.replace("@doctype", "@tag.doctype");
        let output = output.replace("@keyword.directive", "@preproc");
        let output = output.replace("@keyword.conditional.ternary", "@keyword");
        let output = output.replace("@function.method.call", "@function");
        let output = output.replace("@function.method", "@function");
        let output = output.replace("@function.call", "@function");
        let output = output.replace("@tag.error", "@tag");
        let output = output.replace("@character.special", "@string.special");
        let output = output.replace("@text", "@text.literal");

        fs::write(&dest, output)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", dest.display()));
        println!("  synced {grammar}/{dest_file}");
    }

    println!("\nQuery files synced from rev {}", &rev[..8]);
}

/// Convert grammar names to Zed language names in injections.scm
fn convert_injection_languages(content: &str) -> String {
    content
        .replace("\"cfscript\"", "\"CFML (Script)\"")
        .replace("\"cfquery\"", "\"CFML (Query)\"")
}

fn try_fetch_text(url: &str) -> Option<String> {
    let mut response = ureq::get(url)
        .header("User-Agent", "zed-cfml-xtask")
        .call()
        .ok()?;
    response.body_mut().read_to_string().ok()
}

fn resolve_rev(owner_repo: &str, use_commit: bool) -> String {
    if !use_commit {
        if let Some((sha, tag)) = latest_tag(owner_repo) {
            println!("  → tag {tag}");
            return sha;
        }
    }
    let sha = latest_commit(owner_repo);
    println!("  → latest commit");
    sha
}

fn latest_tag(owner_repo: &str) -> Option<(String, String)> {
    let url = format!("https://api.github.com/repos/{owner_repo}/tags?per_page=1");
    let body: String = ureq::get(&url)
        .header("User-Agent", "zed-cfml-xtask")
        .call()
        .ok()?
        .body_mut()
        .read_to_string()
        .ok()?;
    let tags: Vec<Tag> = serde_json::from_str(&body).ok()?;
    tags.into_iter().next().map(|t| (t.commit.sha, t.name))
}

fn latest_commit(owner_repo: &str) -> String {
    let url = format!("https://api.github.com/repos/{owner_repo}/commits?per_page=1");
    let body: String = ureq::get(&url)
        .header("User-Agent", "zed-cfml-xtask")
        .call()
        .expect("failed to fetch commits")
        .body_mut()
        .read_to_string()
        .expect("failed to read response");
    let commits: Vec<Commit> = serde_json::from_str(&body).expect("invalid JSON");
    commits.into_iter().next().expect("no commits found").sha
}

fn lint() {
    let root = workspace_root();
    run_cmd(
        &root,
        "cargo",
        &[
            "clippy",
            "--target",
            "wasm32-wasip2",
            "--",
            "-D",
            "warnings",
        ],
    );
    gen_tasks(true);
}

fn release(version: &str, dry_run: bool) {
    if dry_run {
        println!("DRY RUN: no files will be modified, no git operations will run\n");
    }

    // Validate semver format
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 || parts.iter().any(|p| p.parse::<u32>().is_err()) {
        eprintln!("Error: version must be in format X.Y.Z (e.g. 0.2.18)");
        process::exit(1);
    }

    let root = workspace_root();

    // Check for uncommitted changes
    let status = process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(&root)
        .output()
        .expect("failed to run git status");
    if !status.stdout.is_empty() {
        eprintln!("Error: working directory has uncommitted changes");
        process::exit(1);
    }

    // Check tag doesn't already exist
    let tag = format!("v{version}");
    let tag_check = process::Command::new("git")
        .args(["tag", "-l", &tag])
        .current_dir(&root)
        .output()
        .expect("failed to run git tag");
    if !tag_check.stdout.is_empty() {
        eprintln!("Error: tag {tag} already exists");
        process::exit(1);
    }

    // Check version is greater than current
    let ext_path = root.join("extension.toml");
    let ext_content = fs::read_to_string(&ext_path).expect("failed to read extension.toml");
    let ext_doc = ext_content.parse::<DocumentMut>().expect("invalid TOML");
    let current_version = ext_doc["version"].as_str().unwrap();
    if !version_greater_than(version, current_version) {
        eprintln!("Error: version {version} is not greater than current {current_version}");
        process::exit(1);
    }

    // Fetch and check we're not behind remote
    println!("Fetching from remote...");
    run_cmd(&root, "git", &["fetch"]);
    let behind = process::Command::new("git")
        .args(["rev-list", "--count", "HEAD..@{u}"])
        .current_dir(&root)
        .output()
        .expect("failed to check remote status");
    let behind_count = String::from_utf8_lossy(&behind.stdout).trim().to_string();
    if behind_count != "0" {
        eprintln!("Error: local branch is {behind_count} commit(s) behind remote");
        process::exit(1);
    }

    // Move [Unreleased] content into a new versioned section
    let changelog_path = root.join("CHANGELOG.md");
    let changelog = fs::read_to_string(&changelog_path).expect("failed to read CHANGELOG.md");
    let unreleased_heading = "## [Unreleased]";
    if !changelog.contains(unreleased_heading) {
        eprintln!("Error: CHANGELOG.md missing ## [Unreleased] section");
        process::exit(1);
    }
    let after_unreleased = changelog.split_once(unreleased_heading).unwrap().1;
    let unreleased_content = if let Some((content, _)) = after_unreleased.split_once("\n## [") {
        content
    } else {
        after_unreleased
    };
    if unreleased_content.trim().is_empty() {
        eprintln!("Error: CHANGELOG.md has no content under ## [Unreleased]");
        process::exit(1);
    }

    if !dry_run {
        println!("\nThis will:");
        println!("  - Update versions in extension.toml and Cargo.toml to {version}");
        println!("  - Move CHANGELOG.md [Unreleased] content to [{version}]");
        println!("  - Lint, build, and test");
        println!("  - Commit, tag v{version}, and push\n");
        eprint!("Proceed? [y/N] ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Aborted.");
            process::exit(0);
        }
    }

    if !dry_run {
        let new_changelog = changelog.replace(
            unreleased_heading,
            &format!("{unreleased_heading}\n\n## [{version}]"),
        );
        fs::write(&changelog_path, new_changelog).expect("failed to write CHANGELOG.md");
        println!("Updated CHANGELOG.md: moved unreleased to [{version}]");
    } else {
        println!("Would update CHANGELOG.md: move unreleased to [{version}]");
    }

    // Update version in extension.toml
    if !dry_run {
        let mut ext_doc = ext_content.parse::<DocumentMut>().expect("invalid TOML");
        ext_doc["version"] = toml_edit::value(version);
        fs::write(&ext_path, ext_doc.to_string()).expect("failed to write extension.toml");
        println!("Updated extension.toml version to {version}");
    } else {
        println!("Would update extension.toml version to {version}");
    }

    // Update version in Cargo.toml
    if !dry_run {
        let cargo_path = root.join("Cargo.toml");
        let cargo_content = fs::read_to_string(&cargo_path).expect("failed to read Cargo.toml");
        let mut cargo_doc = cargo_content.parse::<DocumentMut>().expect("invalid TOML");
        cargo_doc["package"]["version"] = toml_edit::value(version);
        fs::write(&cargo_path, cargo_doc.to_string()).expect("failed to write Cargo.toml");
        println!("Updated Cargo.toml version to {version}");
    } else {
        println!("Would update Cargo.toml version to {version}");
    }

    // Run lints
    println!("\nLinting...");
    lint();

    // Run build
    println!("\nBuilding...");
    run_cmd(&root, "cargo", &["build", "--target", "wasm32-wasip2"]);

    // Run tests
    println!("Testing...");
    run_cmd(&root, "cargo", &["test", "--package", "xtask"]);

    // Git commit, tag, push
    if !dry_run {
        println!("\nCommitting...");
        run_cmd(
            &root,
            "git",
            &[
                "add",
                "extension.toml",
                "Cargo.toml",
                "Cargo.lock",
                "CHANGELOG.md",
            ],
        );
        run_cmd(
            &root,
            "git",
            &["commit", "-m", &format!("Release v{version}")],
        );

        println!("Tagging {tag}...");
        run_cmd(&root, "git", &["tag", &tag]);

        println!("Pushing...");
        run_cmd(&root, "git", &["push"]);
        run_cmd(&root, "git", &["push", "origin", &tag]);

        println!("\nReleased v{version}");
    } else {
        println!("\nWould commit, tag {tag}, and push");
        println!("\nDry run complete. All checks passed.");
    }
}

fn run_cmd(dir: &PathBuf, cmd: &str, args: &[&str]) {
    let status = process::Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap_or_else(|e| panic!("failed to run {cmd}: {e}"));
    if !status.success() {
        eprintln!("Command failed: {cmd} {}", args.join(" "));
        process::exit(1);
    }
}

fn version_greater_than(new: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> { v.split('.').map(|p| p.parse().unwrap_or(0)).collect() };
    parse(new) > parse(current)
}

fn workspace_root() -> PathBuf {
    let output = process::Command::new(env!("CARGO"))
        .args(["locate-project", "--workspace", "--message-format=plain"])
        .output()
        .expect("failed to run cargo locate-project");
    let path = String::from_utf8(output.stdout).unwrap();
    PathBuf::from(path.trim()).parent().unwrap().to_path_buf()
}

// ─── tasks.json generation ───────────────────────────────────────────────────

/// The languages that get a copy of tasks.json. Zed loads tasks per language
/// directory, and whichever language owns the active buffer is the only
/// tasks.json in scope, so all three need the same set.
const TASK_LANGUAGES: [&str; 3] = ["cfml", "cfscript", "cfquery"];

/// The resolver every task opens with, in POSIX shell.
///
/// `command` cannot simply be `cfmleditor-lsp`: the extension only uses a PATH
/// binary when `worktree.which()` finds one, and otherwise downloads its own
/// copy into Zed's extension work directory, which is not on PATH. See the
/// "Every task resolves the binary itself" section of CLAUDE.md for why each
/// piece is shaped the way it is.
///
/// Two guards that are easy to lose when editing this by hand:
///
///   - `command -v` reports aliases and shell functions too, and tasks run in
///     an interactive shell that has sourced the user's rc file. Requiring an
///     absolute path and then an executable file rejects `cfmleditor-lsp:
///     aliased to ...` instead of trying to exec it.
///   - the find matches a symlink as well as a regular file, which is what
///     `make link` in the cfmleditor-lsp repo puts on PATH.
const TASK_RESOLVER: &str = concat!(
    r#"CFLSP="$(command -v cfmleditor-lsp 2>/dev/null)"; "#,
    r#"case "$CFLSP" in /*) ;; *) CFLSP="" ;; esac; "#,
    r#"[ -x "$CFLSP" ] || CFLSP="$(find "$HOME/Library/Application Support/Zed/extensions/work/cfml" "#,
    r#""$HOME/.local/share/zed/extensions/work/cfml" -name cfmleditor-lsp \( -type f -o -type l \) "#,
    r#"2>/dev/null | head -1)"; "#,
    r#"[ -x "$CFLSP" ] || { echo "cfmleditor-lsp not found on PATH or in Zed's extension work dir. "#,
    r#"Open a CFML file to let the extension download it, or install it on PATH." >&2; exit 127; }; "#,
    r#"exec "$CFLSP""#,
);

/// One entry per task: the label Zed shows, and the arguments appended after
/// the resolver's `exec`. Zed escapes `args`, so paths with spaces survive;
/// that is why the real arguments live here rather than in `command`.
const TASKS: &[(&str, &[&str])] = &[
    (
        "CFML: Scan Workspace for Parse Errors",
        &["scan", "$ZED_WORKTREE_ROOT"],
    ),
    (
        "CFML: Scan Current File for Parse Errors",
        &["scan", "$ZED_FILE"],
    ),
    ("CFML: Format Current File", &["format", "-w", "$ZED_FILE"]),
    (
        "CFML: Explain Call Resolution at Cursor",
        &[
            "explain",
            "--root",
            "$ZED_WORKTREE_ROOT",
            "$ZED_FILE",
            "$ZED_ROW",
        ],
    ),
    (
        "CFML: Unresolved Calls in Workspace",
        &["unresolved", "$ZED_WORKTREE_ROOT"],
    ),
    (
        "CFML: References to Symbol at Cursor",
        &["refs", "$ZED_SYMBOL", "$ZED_WORKTREE_ROOT"],
    ),
    (
        "CFML: Dependency Graph for Current File",
        &["deps", "--mermaid", "$ZED_FILE"],
    ),
];

fn tasks_json() -> String {
    let tasks: Vec<serde_json::Value> = TASKS
        .iter()
        .map(|(label, args)| {
            serde_json::json!({
                "label": label,
                "command": TASK_RESOLVER,
                "args": args,
            })
        })
        .collect();

    let mut out = serde_json::to_string_pretty(&tasks).expect("failed to serialise tasks");
    out.push('\n');
    out
}

/// Writes languages/<lang>/tasks.json for every language, or with `check`
/// reports which ones are stale without touching them.
///
/// The three files used to be maintained by hand and copied with `cp`, which
/// left 21 copies of the resolver in the tree - seven tasks times three
/// languages - and no way to notice when one copy drifted.
fn gen_tasks(check: bool) {
    let root = workspace_root();
    let want = tasks_json();
    let mut stale = Vec::new();

    for lang in TASK_LANGUAGES {
        let path = root.join("languages").join(lang).join("tasks.json");
        let current = fs::read_to_string(&path).unwrap_or_default();

        if current == want {
            continue;
        }

        if check {
            stale.push(format!("languages/{lang}/tasks.json"));
            continue;
        }

        fs::write(&path, &want)
            .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
        println!("wrote languages/{lang}/tasks.json");
    }

    if !stale.is_empty() {
        eprintln!("Error: tasks.json is out of date in:");
        for s in &stale {
            eprintln!("  {s}");
        }
        eprintln!("These files are generated. Edit TASKS in xtask/src/main.rs, then run:");
        eprintln!("  cargo xtask gen-tasks");
        process::exit(1);
    }

    if check {
        println!(
            "tasks.json is in sync across all {} languages",
            TASK_LANGUAGES.len()
        );
    }
}
