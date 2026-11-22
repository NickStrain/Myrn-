//! The project map: `summarize` writes linked summary notes, only asks the
//! model about code that changed, and search finds code by its summary.

#![cfg(unix)]

use std::path::Path;

use myrn_engine::api::Engine;
use myrn_engine::summarize::Options;

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

fn repo(files: &[(&str, &str)]) -> (tempfile::TempDir, Engine) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    write(&root, files);
    let r = git2::Repository::init(&root).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    r.commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap();
    let (engine, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    (tmp, engine)
}

/// A stand-in model: one fixed answer that serves files and directories alike,
/// and a count of calls in `calls.log`.
fn model(tmp: &Path) -> Vec<String> {
    let log = tmp.join("calls.log");
    let answer = r#"{"file": "Expiry arithmetic for bearer credentials.", "symbols": {"grace": "Adds the leeway window to a deadline so near-expired credentials still pass.", "Vault": "Keeps issued credentials in memory."}, "summary": "Credential handling."}"#;
    vec![
        "sh".into(),
        "-c".into(),
        format!(
            "cat > /dev/null; echo x >> '{}'; echo '{answer}'",
            log.display()
        ),
    ]
}

fn calls(tmp: &Path) -> usize {
    std::fs::read_to_string(tmp.join("calls.log"))
        .map(|s| s.lines().count())
        .unwrap_or(0)
}

#[test]
fn map_is_written_linked_incremental_and_searchable() {
    let (tmp, mut e) = repo(&[
        ("src/lib.rs", "pub mod auth;\n"),
        (
            "src/auth.rs",
            "pub struct Vault { items: Vec<u64> }\n\npub fn grace(deadline: u64) -> u64 {\n    deadline + 30\n}\n",
        ),
    ]);
    let opts = Options {
        dry_run: false,
        limit: None,
        jobs: 2,
        command: model(tmp.path()),
    };
    let r = e.summarize(&opts, &|_| {}).unwrap();
    assert!(r.failures.is_empty(), "{:?}", r.failures);
    assert_eq!(r.updated_symbols, 2);
    let first = calls(tmp.path());
    assert!(first >= 2, "{first}");

    // Obsidian vault: one note per symbol, file and directory, linked by [[id]].
    let vault = e.layout.knowledge_dir().join("summaries");
    let sym = std::fs::read_to_string(vault.join("symbols/auth-grace.md")).unwrap();
    assert!(sym.contains("leeway window"), "{sym}");
    assert!(sym.contains("[[file-src-auth-rs]]"), "{sym}");
    assert!(sym.contains("about: sym:src/auth.rs#auth::grace"), "{sym}");
    let file = std::fs::read_to_string(vault.join("files/file-src-auth-rs.md")).unwrap();
    assert!(
        file.contains("[[auth-grace]]") && file.contains("[[dir-src]]"),
        "{file}"
    );
    assert!(vault.join("project/project.md").exists());

    // The summary is on the code node (without the links section) ...
    let card = e.explain("grace").unwrap().render(|x| x.render());
    assert!(
        card.contains("leeway window") && !card.contains("Connections"),
        "{card}"
    );
    // ... and a task matches it by what it does, not by its name.
    let pack = e
        .context("near-expired credentials should still pass", None)
        .unwrap();
    assert!(pack.text.contains("auth::grace"), "{}", pack.text);
    // The brief carries the project summary.
    assert!(e
        .brief(None)
        .unwrap()
        .text
        .contains("project: Credential handling."));

    // Nothing changed: no model calls.
    let r = e.summarize(&opts, &|_| {}).unwrap();
    assert_eq!(calls(tmp.path()), first);
    assert_eq!(r.notes_written, 0);

    // One function changed: only its file is asked about again.
    write(
        e.root(),
        &[(
            "src/auth.rs",
            "pub struct Vault { items: Vec<u64> }\n\npub fn grace(deadline: u64) -> u64 {\n    deadline + 60\n}\n",
        )],
    );
    let r = e.summarize(&opts, &|_| {}).unwrap();
    assert_eq!(r.updated_symbols, 1, "only the changed function");
    assert!(calls(tmp.path()) > first);

    // Removed code loses its note.
    write(
        e.root(),
        &[("src/auth.rs", "pub struct Vault { items: Vec<u64> }\n")],
    );
    let r = e.summarize(&opts, &|_| {}).unwrap();
    assert_eq!(r.notes_removed, 1);
    assert!(!vault.join("symbols/auth-grace.md").exists());
}
