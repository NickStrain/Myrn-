//! Attach outcomes: test runs pass/fail from tool results, and commits made
//! during the session. Reverts and user corrections come with validity tracking.

use std::path::Path;

use git2::{Repository, Sort};

/// Test runners recognised in a shell command. Matched per command segment
/// (`cd x && cargo test` counts), on whole words.
const RUNNERS: &[&[&str]] = &[
    &["cargo", "test"],
    &["cargo", "nextest"],
    &["pytest"],
    &["py.test"],
    &["python", "-m", "pytest"],
    &["python3", "-m", "pytest"],
    &["python", "-m", "unittest"],
    &["python3", "-m", "unittest"],
    &["uv", "run", "pytest"],
    &["poetry", "run", "pytest"],
    &["tox"],
    &["nox"],
    &["npm", "test"],
    &["npm", "run", "test"],
    &["npm", "t"],
    &["yarn", "test"],
    &["pnpm", "test"],
    &["pnpm", "run", "test"],
    &["bun", "test"],
    &["npx", "jest"],
    &["npx", "vitest"],
    &["jest"],
    &["vitest"],
    &["mocha"],
    &["deno", "test"],
    &["go", "test"],
    &["make", "test"],
    &["make", "check"],
    &["just", "test"],
    &["mvn", "test"],
    &["gradle", "test"],
    &["./gradlew", "test"],
    &["dotnet", "test"],
];

/// Is this shell command a test run?
pub fn is_test_command(command: &str) -> bool {
    command.split(['&', ';', '|', '\n', '(']).any(|segment| {
        let words: Vec<&str> = segment
            .split_whitespace()
            // leading env assignments: RUST_LOG=debug cargo test
            .skip_while(|w| w.contains('=') && !w.starts_with('-'))
            .collect();
        RUNNERS
            .iter()
            .any(|r| words.len() >= r.len() && words[..r.len()] == **r)
    })
}

/// Commits reachable from HEAD made at or after `since` (unix seconds),
/// newest first, as `sha summary`.
pub fn commits_since(root: &Path, since: i64, limit: usize) -> Vec<String> {
    let Ok(repo) = Repository::discover(root) else {
        return Vec::new();
    };
    let Ok(mut walk) = repo.revwalk() else {
        return Vec::new();
    };
    if walk.push_head().is_err() || walk.set_sorting(Sort::TIME).is_err() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for oid in walk.flatten() {
        let Ok(c) = repo.find_commit(oid) else {
            continue;
        };
        if c.time().seconds() < since || out.len() >= limit {
            break;
        }
        let summary = c.summary().ok().flatten().unwrap_or("").to_string();
        out.push(format!("{} {}", oid, summary).trim_end().to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_test_runs() {
        for c in [
            "cargo test",
            "cargo test -p myrn-engine -- --nocapture",
            "cd crates/x && cargo test",
            "RUST_BACKTRACE=1 cargo nextest run",
            "python -m pytest tests/test_auth.py -x",
            "pytest -k refresh",
            "npm test",
            "pnpm run test -- --watch=false",
            "go test ./...",
            "(cd web; npx vitest run)",
        ] {
            assert!(is_test_command(c), "{c}");
        }
        for c in [
            "cargo build",
            "cargo testify",
            "git commit -m 'cargo test passes'",
            "ls tests/",
            "cat pytest.ini",
            "echo npm",
            "rg 'go test'",
        ] {
            assert!(!is_test_command(c), "{c}");
        }
    }
}
