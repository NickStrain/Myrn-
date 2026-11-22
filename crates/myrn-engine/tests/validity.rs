//! Validity tracking (Step 18): records are checked against the current code
//! by hash, never by commit. One test per row of the design's "Why hashes
//! instead of revisions" table, plus confirm, supersede and verification.

use std::path::{Path, PathBuf};

use myrn_engine::api::Engine;
use myrn_store::episodes::{self, Entry};
use serde_json::json;

const RECIPES: &str = "def unique_everseen(iterable):\n    seen = set()\n    for x in iterable:\n        if x not in seen:\n            seen.add(x)\n            yield x\n";
const RECIPES_REFORMATTED: &str = "# Recipes.\n\ndef unique_everseen(iterable):\n    # remember what we saw\n    seen = set( )\n\n    for x in iterable:\n        if x not in seen:\n            seen.add(x)\n            yield x\n";
const RECIPES_CHANGED: &str = "def unique_everseen(iterable, key=None):\n    seen = []\n    for x in iterable:\n        k = key(x) if key else x\n        if k not in seen:\n            seen.append(k)\n            yield x\n";

fn files() -> Vec<(&'static str, &'static str)> {
    vec![
        ("pkg/__init__.py", "from .recipes import *\nfrom .more import *\n"),
        ("pkg/recipes.py", RECIPES),
        ("pkg/more.py", "def chunked(it, n):\n    return [it]\n"),
        (
            "tests/test_recipes.py",
            "import pkg as mi\n\ndef test_everseen():\n    assert list(mi.unique_everseen('aab')) == ['a', 'b']\n",
        ),
    ]
}

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

struct Repo {
    tmp: tempfile::TempDir,
    root: PathBuf,
    e: Engine,
    head: String,
}

fn commit_all(root: &Path, msg: &str) -> String {
    let r = git2::Repository::open(root).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    let parents: Vec<git2::Commit> = r
        .head()
        .ok()
        .and_then(|h| h.peel_to_commit().ok())
        .into_iter()
        .collect();
    let parents: Vec<&git2::Commit> = parents.iter().collect();
    r.commit(Some("HEAD"), &sig, &sig, msg, &tree, &parents)
        .unwrap()
        .to_string()
}

fn repo() -> Repo {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    write(&root, &files());
    git2::Repository::init(&root).unwrap();
    let head = commit_all(&root, "first");
    let (e, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    Repo { tmp, root, e, head }
}

fn status(e: &Engine, id: &str) -> String {
    e.db()
        .conn
        .query_row("SELECT status FROM nodes WHERE id = ?1", [id], |r| r.get(0))
        .unwrap()
}

fn set(r: &mut Repo, path: &str, text: &str) {
    write(&r.root, &[(path, text)]);
    r.e.refresh().unwrap();
}

fn fact(e: &mut Engine, body: &str, about: &str) -> String {
    let o = e
        .record_json(
            &json!({"type": "fact", "body": body, "about": about, "evidence": [{"symbol": about}]}),
        )
        .unwrap();
    assert!(o.accepted, "{}", o.render());
    o.ids[0].clone()
}

#[test]
fn branch_switch_makes_records_suspect_and_valid_again() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "unique_everseen keeps the first of each item and needs hashable items.",
        "unique_everseen",
    );
    assert_eq!(status(&r.e, &id), "unverified");

    // An older or other branch where the function is different: suspect there.
    set(&mut r, "pkg/recipes.py", RECIPES_CHANGED);
    assert_eq!(status(&r.e, &id), "suspect");
    let s = r.e.suspects().unwrap();
    assert_eq!(s.len(), 1);
    assert!(s[0].changed[0].name.contains("unique_everseen"), "{s:?}");
    assert!(!s[0].changed[0].gone);

    // Back on the code it was written about: valid again, in its earlier state.
    set(&mut r, "pkg/recipes.py", RECIPES);
    assert_eq!(status(&r.e, &id), "unverified");
    assert!(r.e.suspects().unwrap().is_empty());
}

#[test]
fn reformat_or_comment_only_changes_have_no_effect() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "unique_everseen keeps the first of each item and needs hashable items.",
        "unique_everseen",
    );
    set(&mut r, "pkg/recipes.py", RECIPES_REFORMATTED);
    assert_eq!(status(&r.e, &id), "unverified");
}

#[test]
fn records_from_an_unmerged_branch_apply_only_where_that_code_exists() {
    let mut r = repo();
    // On a feature branch: a function that main does not have.
    let branch =
        format!("{RECIPES}\ndef dedupe_all(items):\n    return list(unique_everseen(items))\n");
    set(&mut r, "pkg/recipes.py", &branch);
    let id = fact(
        &mut r.e,
        "dedupe_all returns a list, not an iterator, and keeps order.",
        "dedupe_all",
    );
    assert_eq!(status(&r.e, &id), "unverified");
    // Main, without that code: suspect (its code is gone here).
    set(&mut r, "pkg/recipes.py", RECIPES);
    assert_eq!(status(&r.e, &id), "suspect");
    assert!(r.e.suspects().unwrap()[0].changed[0].gone);
    // Back on the branch.
    set(&mut r, "pkg/recipes.py", &branch);
    assert_eq!(status(&r.e, &id), "unverified");
}

#[test]
fn worktrees_share_notes_but_judge_them_by_their_own_files() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "chunked wraps the whole input in one list.",
        "chunked",
    );
    // A linked worktree of the same repository, with chunked changed there.
    let wt_path = r.tmp.path().join("wt");
    let repo = git2::Repository::open(&r.root).unwrap();
    repo.worktree("wt", &wt_path, None).unwrap();
    write(&wt_path, &[("pkg/more.py", "def chunked(it, n):\n    return [list(it)[i:i + n] for i in range(0, len(list(it)), n)]\n")]);
    let (mut wt, _) = Engine::init(&wt_path, Some(&r.tmp.path().join("home"))).unwrap();
    assert_ne!(wt.layout.db_path, r.e.layout.db_path, "own code index");
    wt.refresh().unwrap();
    assert_eq!(
        status(&wt, &id),
        "suspect",
        "the worktree's chunked differs"
    );
    r.e.refresh().unwrap();
    assert_eq!(status(&r.e, &id), "unverified", "the main tree's matches");
}

#[test]
fn shared_records_from_a_teammate_apply_when_their_code_matches() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "chunked wraps the whole input in one list.",
        "chunked",
    );
    // Publish by hand: move the note into the committed knowledge directory.
    let private =
        r.e.layout
            .notes_dir()
            .join("facts")
            .join(format!("{id}.md"));
    let text = std::fs::read_to_string(&private).unwrap();
    std::fs::remove_file(&private).unwrap();

    // A teammate's checkout with the same chunked: valid.
    let tmp = tempfile::tempdir().unwrap();
    let mate = tmp.path().join("repo");
    write(&mate, &files());
    git2::Repository::init(&mate).unwrap();
    commit_all(&mate, "first");
    let note = mate.join(".myrn/knowledge/facts").join(format!("{id}.md"));
    std::fs::create_dir_all(note.parent().unwrap()).unwrap();
    std::fs::write(&note, &text).unwrap();
    let (m, _) = Engine::init(&mate, Some(&tmp.path().join("home"))).unwrap();
    assert_eq!(status(&m, &id), "unverified");

    // One whose chunked is different: suspect.
    let tmp2 = tempfile::tempdir().unwrap();
    let other = tmp2.path().join("repo");
    write(&other, &files());
    write(
        &other,
        &[("pkg/more.py", "def chunked(it, n):\n    return list(it)\n")],
    );
    git2::Repository::init(&other).unwrap();
    commit_all(&other, "first");
    let note = other.join(".myrn/knowledge/facts").join(format!("{id}.md"));
    std::fs::create_dir_all(note.parent().unwrap()).unwrap();
    std::fs::write(&note, &text).unwrap();
    let (o, _) = Engine::init(&other, Some(&tmp2.path().join("home"))).unwrap();
    assert_eq!(status(&o, &id), "suspect");
}

#[test]
fn suspect_records_are_labelled_ranked_down_and_shown_where_the_agent_works() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "unique_everseen keeps the first of each item and needs hashable items.",
        "unique_everseen",
    );
    set(&mut r, "pkg/recipes.py", RECIPES_CHANGED);

    let (text, ids) =
        r.e.suspect_notice(&["pkg/recipes.py".into()], &Default::default(), 500)
            .unwrap();
    assert_eq!(ids, vec![id.clone()]);
    assert!(
        text.contains("possibly outdated") && text.contains("`confirm`"),
        "{text}"
    );
    assert!(text.contains("unique_everseen changed"), "{text}");
    // Not for other files, and not twice in a session.
    assert!(r
        .e
        .suspect_notice(&["pkg/more.py".into()], &Default::default(), 500)
        .unwrap()
        .1
        .is_empty());
    let seen = ids.into_iter().collect();
    assert!(r
        .e
        .suspect_notice(&["pkg/recipes.py".into()], &seen, 500)
        .unwrap()
        .1
        .is_empty());

    // In a context pack it is quoted with the label and the ask.
    let pack =
        r.e.context("why does `unique_everseen` need hashable items", None)
            .unwrap();
    if pack.items.contains(&id) {
        assert!(
            pack.text.contains("possibly outdated") && pack.text.contains("`supersede`"),
            "{}",
            pack.text
        );
    }
}

#[test]
fn confirm_moves_anchors_to_the_current_code() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "unique_everseen keeps the first of each item it sees.",
        "unique_everseen",
    );
    set(&mut r, "pkg/recipes.py", RECIPES_CHANGED);
    assert_eq!(status(&r.e, &id), "suspect");
    let c = r.e.confirm(&id).unwrap();
    assert_eq!(c.anchors_refreshed, 1);
    assert_eq!(c.status, "unverified");
    // Now anchored to the new code: the old code makes it suspect.
    set(&mut r, "pkg/recipes.py", RECIPES);
    assert_eq!(status(&r.e, &id), "suspect");

    // Code that is gone cannot be confirmed.
    let branch = format!("{RECIPES}\ndef dedupe_all(items):\n    return list(items)\n");
    set(&mut r, "pkg/recipes.py", &branch);
    let gone = fact(
        &mut r.e,
        "dedupe_all returns a list and keeps order.",
        "dedupe_all",
    );
    set(&mut r, "pkg/recipes.py", RECIPES);
    let err = r.e.confirm(&gone).unwrap_err().to_string();
    assert!(err.contains("no longer exists"), "{err}");
}

#[test]
fn supersede_replaces_and_keeps_history() {
    let mut r = repo();
    let id = fact(
        &mut r.e,
        "unique_everseen needs hashable items or it raises TypeError.",
        "unique_everseen",
    );
    set(&mut r, "pkg/recipes.py", RECIPES_CHANGED);
    let o = r
        .e
        .supersede(
            &id,
            &json!({"type": "fact", "body": "unique_everseen accepts unhashable items now; it compares with a list.",
                    "about": "unique_everseen", "evidence": [{"symbol": "unique_everseen"}]}),
        )
        .unwrap();
    assert!(o.accepted, "{}", o.render());
    let new = o.ids[0].clone();
    assert_eq!(status(&r.e, &new), "unverified");
    assert_eq!(status(&r.e, &id), "superseded");
    // Still on disk and in the index (history), but not suspect work any more.
    assert!(r.e.suspects().unwrap().iter().all(|s| s.id != id));
    assert!(r.e.supersede("fact-nope", &json!({})).is_err());
}

#[test]
fn records_become_verified_when_a_cited_test_passes_or_the_fix_lands() {
    let mut r = repo();
    let by_test =
        r.e.record_json(
            &json!({"type": "fact", "body": "unique_everseen yields items in first-seen order.",
            "about": "unique_everseen", "evidence": [{"test": "test_everseen"}]}),
        )
        .unwrap();
    assert!(by_test.accepted, "{}", by_test.render());
    let by_test = by_test.ids[0].clone();
    let by_fix =
        r.e.record_json(
            &json!({"type": "bug", "body": "unique_everseen raised TypeError on lists.",
            "error": "TypeError: unhashable type: 'list'", "fixed_in": &r.head[..8],
            "evidence": [{"symbol": "unique_everseen"}]}),
        )
        .unwrap();
    assert!(by_fix.accepted, "{}", by_fix.render());
    let by_fix = by_fix.ids[0].clone();

    // No passing run yet: only the landed fix counts.
    let p = r.e.promote().unwrap();
    assert_eq!(p.len(), 1, "{p:?}");
    assert_eq!(p[0].id, by_fix);
    assert_eq!(status(&r.e, &by_fix), "verified");
    assert_eq!(status(&r.e, &by_test), "unverified");

    // A recorded session where the test passed.
    let dir = r.e.layout.episodes_dir();
    for en in [
        Entry {
            t: 1,
            agent: "claude".into(),
            event: "session_start".into(),
            ..Default::default()
        },
        Entry {
            t: 2,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("other".into()),
            command: Some("pytest tests/test_recipes.py -q".into()),
            test: Some("pass".into()),
            ..Default::default()
        },
    ] {
        episodes::append(&dir, "s1", &en).unwrap();
    }
    let p = r.e.promote().unwrap();
    assert_eq!(p.len(), 1, "{p:?}");
    assert_eq!(status(&r.e, &by_test), "verified");
    // Verified is written to the note, so it survives a rebuild.
    r.e.rebuild().unwrap();
    assert_eq!(status(&r.e, &by_test), "verified");

    // A suspect record is not promoted, and a verified one returns to verified.
    set(&mut r, "pkg/recipes.py", RECIPES_CHANGED);
    assert_eq!(status(&r.e, &by_test), "suspect");
    set(&mut r, "pkg/recipes.py", RECIPES);
    assert_eq!(status(&r.e, &by_test), "verified");
}

#[test]
fn a_reverted_fix_does_not_verify() {
    let mut r = repo();
    write(
        &r.root,
        &[(
            "pkg/more.py",
            "def chunked(it, n):\n    return [list(it)]\n",
        )],
    );
    let fix = commit_all(&r.root, "fix chunked");
    r.e.refresh().unwrap();
    write(
        &r.root,
        &[("pkg/more.py", "def chunked(it, n):\n    return [it]\n")],
    );
    commit_all(
        &r.root,
        &format!("Revert \"fix chunked\"\n\nThis reverts commit {fix}."),
    );
    r.e.refresh().unwrap();
    let o =
        r.e.record_json(
            &json!({"type": "bug", "body": "chunked returned the iterator itself.",
            "error": "TypeError: 'generator' object is not subscriptable", "fixed_in": &fix[..8],
            "evidence": [{"symbol": "chunked"}]}),
        )
        .unwrap();
    assert!(o.accepted, "{}", o.render());
    assert!(r.e.promote().unwrap().is_empty());
    assert_eq!(status(&r.e, &o.ids[0]), "unverified");
}
