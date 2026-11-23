//! The learning loop (Step 20): injected items are marked used when the agent
//! opens their file or asks about them, sessions attach their outcome, and the
//! learned ranking moves items by past usefulness, with the fixed ranking as a
//! switch. Embeddings stay out of the way when not enabled.

use std::path::Path;

use myrn_engine::api::Engine;
use myrn_store::episodes::{self, Entry};

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

fn repo() -> (tempfile::TempDir, Engine) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    // Two functions that match "token" equally well, in different files.
    write(
        &root,
        &[
            ("auth/issue.py", "def issue_token(user):\n    \"\"\"Create a session token.\"\"\"\n    return str(user)\n"),
            ("auth/check.py", "def check_token(token):\n    \"\"\"Validate a session token.\"\"\"\n    return bool(token)\n"),
        ],
    );
    let r = git2::Repository::init(&root).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    r.commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap();
    let (e, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    (tmp, e)
}

const ISSUE: &str = "sym:auth/issue.py#auth.issue.issue_token";
const CHECK: &str = "sym:auth/check.py#auth.check.check_token";

fn used(e: &Engine, id: &str, session: &str) -> i64 {
    e.db()
        .conn
        .query_row(
            "SELECT coalesce(SUM(used), 0) FROM usage WHERE note_id = ?1 AND session_id = ?2",
            [id, session],
            |r| r.get(0),
        )
        .unwrap()
}

#[test]
fn opening_the_file_or_asking_about_it_counts_as_use() {
    let (_t, e) = repo();
    e.log_injection("s1", &[ISSUE.to_string(), CHECK.to_string()])
        .unwrap();
    assert_eq!(
        e.mark_used_files("s1", &["auth/issue.py".into()]).unwrap(),
        1
    );
    assert_eq!(used(&e, ISSUE, "s1"), 1);
    assert_eq!(used(&e, CHECK, "s1"), 0);
    // Another session's injections are not affected by this one's reads.
    e.log_injection("s2", &[CHECK.to_string()]).unwrap();
    e.mark_used_files("s1", &["auth/check.py".into()]).unwrap();
    assert_eq!(used(&e, CHECK, "s2"), 0);
    // Asking about it (explain, recall, citing its id) counts for recent injections.
    e.mark_used_ids(&[CHECK.to_string()]).unwrap();
    assert_eq!(used(&e, CHECK, "s2"), 1);
}

#[test]
fn sessions_attach_their_outcome() {
    let (_t, e) = repo();
    e.log_injection("s1", &[ISSUE.to_string()]).unwrap();
    let dir = e.layout.episodes_dir();
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
            command: Some("pytest".into()),
            test: Some("pass".into()),
            ..Default::default()
        },
    ] {
        episodes::append(&dir, "s1", &en).unwrap();
    }
    e.attach_outcome("s1").unwrap();
    let outcome: String = e
        .db()
        .conn
        .query_row(
            "SELECT outcome FROM usage WHERE session_id = 's1'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(outcome, "tests_passed");
}

fn order(e: &Engine, task: &str) -> Vec<String> {
    e.context(task, None).unwrap().items
}

#[test]
fn learned_ranking_moves_items_by_past_use_and_fixed_ignores_it() {
    let (_t, mut e) = repo();
    let task = "session token handling";
    let before = order(&e, task);
    assert!(
        before.contains(&ISSUE.to_string()) && before.contains(&CHECK.to_string()),
        "{before:?}"
    );

    // History: whichever came first was injected many times and never used;
    // the other was used every time, in sessions whose tests passed.
    let (ignored, helpful) = (before[0].clone(), before[1].clone());
    for i in 0..8 {
        let s = format!("past-{i}");
        e.log_injection(&s, &[ignored.clone(), helpful.clone()])
            .unwrap();
        let path = if helpful == ISSUE {
            "auth/issue.py"
        } else {
            "auth/check.py"
        };
        e.mark_used_files(&s, &[path.into()]).unwrap();
        e.db()
            .conn
            .execute(
                "UPDATE usage SET outcome = 'tests_passed' WHERE session_id = ?1",
                [&s],
            )
            .unwrap();
    }
    let u = e.usage(&[&ignored, &helpful]).unwrap();
    assert!(
        u[&helpful].factor(0.5) > 1.0 && u[&ignored].factor(0.5) < 1.0,
        "{u:?}"
    );

    let learned = order(&e, task);
    assert_eq!(learned.first(), Some(&helpful), "{learned:?}");

    // The fixed ranking is the switch back.
    e.config.ranking.mode = "fixed".into();
    assert_eq!(order(&e, task), before);
}

#[test]
fn without_embeddings_nothing_changes() {
    let (_t, e) = repo();
    assert!(!e.embeddings_on());
    assert!(e.vector_seeds("anything at all").unwrap().is_empty());
    let s = e.embeddings_status().unwrap();
    assert!(!s.enabled && !s.installed && s.vectors == 0);
    // The model is not here: the client says so instead of failing.
    assert!(myrn_embed::client::embed(&e.data_home(), &["x".into()], None).is_none());
}
