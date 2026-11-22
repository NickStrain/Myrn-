//! History and housekeeping (Step 19): the record list, trace, diff between
//! two points, forget and gc.

use std::path::Path;

use myrn_engine::api::Engine;
use myrn_engine::knowledge::RecordFilter;
use myrn_store::episodes::{self, Entry};
use serde_json::json;

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
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

fn setup() -> (tempfile::TempDir, Engine, String) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    write(
        &root,
        &[
            ("pkg/__init__.py", "from .more import *\n"),
            ("pkg/more.py", "def chunked(it, n):\n    return [it]\n\ndef first(it):\n    return next(iter(it))\n"),
        ],
    );
    git2::Repository::init(&root).unwrap();
    let head = commit_all(&root, "fix chunked");
    let (e, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    (tmp, e, head)
}

fn session(e: &mut Engine) {
    let dir = e.layout.episodes_dir();
    for en in [
        Entry {
            t: 100,
            agent: "claude".into(),
            event: "session_start".into(),
            ..Default::default()
        },
        Entry {
            t: 101,
            agent: "claude".into(),
            event: "prompt".into(),
            text: Some("chunked drops items".into()),
            ..Default::default()
        },
        Entry {
            t: 102,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("file_edit".into()),
            files: vec!["pkg/more.py".into()],
            ..Default::default()
        },
    ] {
        episodes::append(&dir, "sess-1", &en).unwrap();
    }
    e.reindex_episodes().unwrap();
}

#[test]
fn log_lists_records_and_trace_shows_where_they_came_from() {
    let (_t, mut e, head) = setup();
    session(&mut e);
    let bug = e
        .record_json(&json!({"type": "bug", "body": "chunked wrapped the iterator instead of splitting it.",
            "error": "AssertionError: [[1, 2, 3]] != [[1, 2], [3]]", "root_cause": "it never sliced the input",
            "located_in": "chunked", "fixed_in": &head[..8], "session": "sess-1",
            "evidence": [{"symbol": "chunked"}]}))
        .unwrap();
    assert!(bug.accepted, "{}", bug.render());
    let bug_id = bug.ids[0].clone();
    let fact = e
        .record_json(
            &json!({"type": "fact", "body": "first raises StopIteration on an empty input.",
            "about": "first", "evidence": [{"symbol": "first"}]}),
        )
        .unwrap();
    assert!(fact.accepted, "{}", fact.render());

    let all = e.records(&RecordFilter::default(), 50).unwrap();
    assert_eq!(all.len(), 3, "bug, its root cause and the fact: {all:?}");
    let bugs = e
        .records(
            &RecordFilter {
                ntype: Some("bug".into()),
                ..Default::default()
            },
            50,
        )
        .unwrap();
    assert_eq!(bugs.len(), 1);
    assert_eq!(bugs[0].id, bug_id);
    let suspect = e
        .records(
            &RecordFilter {
                status: Some("suspect".into()),
                ..Default::default()
            },
            50,
        )
        .unwrap();
    assert!(suspect.is_empty());

    let t = e.trace(&bug_id).unwrap();
    assert_eq!(t.session.as_ref().unwrap().session, "sess-1");
    assert_eq!(t.commits.len(), 1);
    assert_eq!(t.commits[0].rel, "fixed in");
    assert_eq!(t.commits[0].sha, head);
    assert_eq!(t.anchors.len(), 1);
    assert_eq!(t.anchors[0].state, "current");
    assert!(t.body.contains("wrapped the iterator"));

    // After the code changes, trace shows the anchor as changed.
    write(e.root(), &[("pkg/more.py", "def chunked(it, n):\n    return [list(it)]\n\ndef first(it):\n    return next(iter(it))\n")]);
    e.refresh().unwrap();
    let t = e.trace(&bug_id).unwrap();
    assert_eq!(t.anchors[0].state, "changed");
    assert_eq!(t.record.status, "suspect");
    assert_eq!(t.record.written_status, "unverified");
    assert!(e.trace("bug-nope").is_err());
}

#[test]
fn diff_compares_shared_knowledge_between_points() {
    let (_t, mut e, _head) = setup();
    let root = e.root().to_path_buf();
    let k = root.join(".myrn/knowledge/facts");
    write(
        &root,
        &[(
            ".myrn/knowledge/facts/fact-aaaaaa.md",
            "---\nid: fact-aaaaaa\ntype: fact\nstatus: unverified\nabout: chunked\n---\nchunked wraps its input.\n",
        )],
    );
    commit_all(&root, "add a fact");
    // Working tree: the fact is verified and another one is added.
    std::fs::write(
        k.join("fact-aaaaaa.md"),
        "---\nid: fact-aaaaaa\ntype: fact\nstatus: verified\nabout: chunked\n---\nchunked wraps its input.\n",
    )
    .unwrap();
    std::fs::write(
        k.join("fact-bbbbbb.md"),
        "---\nid: fact-bbbbbb\ntype: fact\nstatus: unverified\nabout: first\n---\nfirst needs a non-empty input.\n",
    )
    .unwrap();
    e.refresh().unwrap();

    let d = e.diff_knowledge("HEAD", None).unwrap();
    assert_eq!(d.added.len(), 1, "{d:?}");
    assert_eq!(d.added[0].id, "fact-bbbbbb");
    assert_eq!(d.changed.len(), 1);
    assert_eq!(
        d.changed[0].detail.as_deref(),
        Some("unverified -> verified")
    );
    assert!(d.removed.is_empty());

    // Between two revisions: the first commit had no knowledge at all.
    let d = e.diff_knowledge("HEAD~1", Some("HEAD")).unwrap();
    assert_eq!(d.added.len(), 1);
    assert_eq!(d.added[0].id, "fact-aaaaaa");
    // A date before everything: nothing then.
    let d = e.diff_knowledge("2100-01-01", None).unwrap();
    assert!(d.removed.is_empty());
    assert!(e.diff_knowledge("1990-01-01", None).is_err());
}

#[test]
fn forget_removes_the_record_and_what_only_it_used() {
    let (_t, mut e, head) = setup();
    let bug = e
        .record_json(
            &json!({"type": "bug", "body": "chunked wrapped the iterator instead of splitting it.",
            "error": "AssertionError: wrong chunks", "root_cause": "it never sliced the input",
            "located_in": "chunked", "fixed_in": &head[..8], "evidence": [{"symbol": "chunked"}]}),
        )
        .unwrap();
    let bug_id = bug.ids[0].clone();
    let cause_id = bug.ids[1].clone();
    // Another record that links to the bug.
    let other = e
        .record_json(
            &json!({"type": "fact", "body": "chunked is the hot path; see the related bug.",
            "about": "chunked", "evidence": [{"symbol": "chunked"}],
            "relations": [{"rel": "related", "target": &bug_id}]}),
        )
        .unwrap();
    assert!(other.accepted, "{}", other.render());
    let other_id = other.ids[0].clone();
    e.log_injection("s1", std::slice::from_ref(&bug_id))
        .unwrap();

    let plan = e.forget_plan(&bug_id).unwrap();
    assert_eq!(plan.ids, vec![bug_id.clone(), cause_id.clone()]);
    assert_eq!(plan.unlinked_from, vec![other_id.clone()]);
    let files = plan.files.clone();
    assert!(files.iter().all(|f| f.exists()));

    e.forget(&bug_id).unwrap();
    assert!(files.iter().all(|f| !f.exists()));
    let c = &e.db().conn;
    let count = |sql: &str, id: &str| -> i64 { c.query_row(sql, [id], |r| r.get(0)).unwrap() };
    assert_eq!(
        count("SELECT COUNT(*) FROM nodes WHERE id = ?1", &bug_id),
        0
    );
    assert_eq!(
        count("SELECT COUNT(*) FROM nodes WHERE id = ?1", &cause_id),
        0
    );
    assert_eq!(
        count("SELECT COUNT(*) FROM usage WHERE note_id = ?1", &bug_id),
        0
    );
    assert_eq!(
        count("SELECT COUNT(*) FROM edges WHERE dst = ?1", &bug_id),
        0
    );
    // The other record stays, without the link, and is not flagged as dangling.
    assert_eq!(
        count("SELECT COUNT(*) FROM nodes WHERE id = ?1", &other_id),
        1
    );
    assert_eq!(
        count(
            "SELECT COUNT(*) FROM note_issues WHERE note_id = ?1",
            &other_id
        ),
        0
    );
}

#[test]
fn gc_merges_copies_drops_stale_rows_and_compacts_old_sessions() {
    let (_t, mut e, _head) = setup();
    let fact = e
        .record_json(
            &json!({"type": "fact", "body": "chunked wraps its whole input in one list.",
            "about": "chunked", "evidence": [{"symbol": "chunked"}]}),
        )
        .unwrap();
    let id = fact.ids[0].clone();
    // A hand copy of the same note in the shared folder, and a differing one.
    let private = e.layout.notes_dir().join("facts").join(format!("{id}.md"));
    let text = std::fs::read_to_string(&private).unwrap();
    let shared_dir = e.layout.knowledge_dir().join("facts");
    std::fs::create_dir_all(&shared_dir).unwrap();
    std::fs::write(shared_dir.join(format!("{id}.md")), &text).unwrap();
    std::fs::create_dir_all(e.layout.notes_dir().join("facts/old")).unwrap();
    std::fs::write(
        e.layout
            .notes_dir()
            .join("facts/old")
            .join(format!("{id}.md")),
        text.replace("one list", "a list of one"),
    )
    .unwrap();
    e.refresh().unwrap();

    // An index row pointing at nothing.
    e.log_injection("s1", &["sym:gone.py#gone".to_string()])
        .unwrap();
    // An old session with plain reads.
    let dir = e.layout.episodes_dir();
    for en in [
        Entry {
            t: 1_000,
            agent: "claude".into(),
            event: "session_start".into(),
            ..Default::default()
        },
        Entry {
            t: 1_001,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("other".into()),
            files: vec!["pkg/more.py".into()],
            ..Default::default()
        },
        Entry {
            t: 1_002,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("other".into()),
            files: vec!["pkg/more.py".into()],
            ..Default::default()
        },
        Entry {
            t: 1_003,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("file_edit".into()),
            files: vec!["pkg/more.py".into()],
            ..Default::default()
        },
    ] {
        episodes::append(&dir, "old", &en).unwrap();
    }

    let dry = e.gc(30, true).unwrap();
    assert_eq!(dry.duplicates_removed.len(), 1, "{dry:?}");
    assert_eq!(dry.duplicates_differing.len(), 1, "{dry:?}");
    assert!(dry.index_rows_dropped >= 1);
    assert_eq!(dry.episodes_compacted, 1);
    assert!(e
        .layout
        .notes_dir()
        .join("facts")
        .join(format!("{id}.md"))
        .exists());

    let r = e.gc(30, false).unwrap();
    assert_eq!(r.duplicates_removed.len(), 1);
    // The record itself is never removed.
    let n: i64 = e
        .db()
        .conn
        .query_row("SELECT COUNT(*) FROM nodes WHERE id = ?1", [&id], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(n, 1);
    let lines = episodes::read(&dir.join("old.jsonl")).unwrap();
    assert_eq!(
        lines.len(),
        3,
        "start, the edit, and one folded read line: {lines:?}"
    );
    let again = e.gc(30, false).unwrap();
    assert_eq!(again.index_rows_dropped, 0);
    assert_eq!(again.episodes_compacted, 0);
}
