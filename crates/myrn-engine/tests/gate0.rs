//! Gate 0: are `explain` and `impact` answers correct?
//!
//! Runs the engine over `tests/fixtures/repos/gate0`, a small project written
//! in Rust, Python and TypeScript whose call graph is known, and compares every
//! answer with the expected one exactly: no missing links, no extra links.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use myrn_engine::api::{Engine, Explained, Lookup};
use myrn_engine::pack::cards::{ImpactReport, SymbolCard};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/repos/gate0")
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for e in std::fs::read_dir(from).unwrap() {
        let e = e.unwrap();
        let dst = to.join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            copy_dir(&e.path(), &dst);
        } else {
            std::fs::copy(e.path(), dst).unwrap();
        }
    }
}

fn commit_all(dir: &Path, message: &str) {
    let repo = git2::Repository::open(dir).unwrap_or_else(|_| git2::Repository::init(dir).unwrap());
    let mut index = repo.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.update_all(["*"], None).unwrap();
    index.write().unwrap();
    let tree = repo.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("Gate Zero", "gate0@example.invalid").unwrap();
    let parent = repo.head().ok().and_then(|h| h.peel_to_commit().ok());
    let parents: Vec<&git2::Commit> = parent.iter().collect();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &parents)
        .unwrap();
}

struct Fixture {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    home: PathBuf,
    engine: Engine,
}

fn setup() -> Fixture {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    let home = tmp.path().join("home");
    copy_dir(&fixture(), &root);
    commit_all(&root, "Add token store and session refresh");
    let (engine, report) = Engine::init(&root, Some(&home)).unwrap();
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    Fixture {
        _tmp: tmp,
        root,
        home,
        engine,
    }
}

fn card(e: &Engine, id: &str) -> SymbolCard {
    match e.explain(id).unwrap() {
        Lookup::Found(Explained::Symbol(c)) => c,
        other => panic!(
            "explain({id}) gave {}",
            serde_json::to_string(&other).unwrap()
        ),
    }
}

fn impact(e: &Engine, id: &str) -> ImpactReport {
    match e.impact(id).unwrap() {
        Lookup::Found(r) => r,
        other => panic!(
            "impact({id}) gave {}",
            serde_json::to_string(&other).unwrap()
        ),
    }
}

fn ids<'a>(
    links: impl IntoIterator<Item = &'a myrn_engine::pack::cards::Link>,
) -> BTreeSet<String> {
    links.into_iter().map(|l| l.id.clone()).collect()
}

fn set(v: &[&str]) -> BTreeSet<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// Expected answers for one symbol.
struct Expect<'a> {
    id: &'a str,
    calls: &'a [&'a str],
    called_by: &'a [&'a str],
    tests: &'a [&'a str],
    /// Every test that reaches the symbol, directly or through callers.
    all_tests: &'a [&'a str],
}

fn check(e: &Engine, x: &Expect) {
    let c = card(e, x.id);
    assert_eq!(ids(&c.calls), set(x.calls), "calls of {}", x.id);
    assert_eq!(ids(&c.called_by), set(x.called_by), "callers of {}", x.id);
    assert_eq!(ids(&c.tests), set(x.tests), "tests of {}", x.id);
    for l in c.calls.iter().chain(&c.called_by) {
        assert_eq!(
            l.status.as_deref(),
            Some("resolved"),
            "{} -> {} should resolve through scope",
            x.id,
            l.id
        );
    }
    let r = impact(e, x.id);
    assert_eq!(
        ids(&r.direct_callers),
        set(x.called_by),
        "impact callers of {}",
        x.id
    );
    let all: BTreeSet<String> = ids(&r.direct_tests)
        .into_iter()
        .chain(ids(r.indirect_tests.iter().map(|(l, _)| l)))
        .collect();
    assert_eq!(all, set(x.all_tests), "impact tests of {}", x.id);
}

// Rust
const R_REFRESH: &str = "sym:src/auth/session.rs#auth::session::refresh";
const R_SESSION_NEW: &str = "sym:src/auth/session.rs#auth::session::Session::new";
const R_FRESH: &str = "sym:src/auth/session.rs#auth::session::Session::is_fresh";
const R_LOOKUP: &str = "sym:src/store.rs#store::TokenStore::lookup";
const R_STORE_NEW: &str = "sym:src/store.rs#store::TokenStore::new";
const R_INSERT: &str = "sym:src/store.rs#store::TokenStore::insert";
const R_NOW: &str = "sym:src/store.rs#store::now";
const R_UNIT: &str = "test:src/auth/session.rs#auth::session::tests::refresh_unknown_token";
const R_INTEG: &str = "test:tests/refresh.rs#refreshes_known_token";

// Python
const P_REFRESH: &str = "sym:py/app/session.py#py.app.session.refresh";
const P_SESSION: &str = "sym:py/app/session.py#py.app.session.Session";
const P_INIT: &str = "sym:py/app/session.py#py.app.session.Session.__init__";
const P_FRESH: &str = "sym:py/app/session.py#py.app.session.Session.is_fresh";
const P_STORE: &str = "sym:py/app/store.py#py.app.store.TokenStore";
const P_LOOKUP: &str = "sym:py/app/store.py#py.app.store.TokenStore.lookup";
const P_INSERT: &str = "sym:py/app/store.py#py.app.store.TokenStore.insert";
const P_NOW: &str = "sym:py/app/store.py#py.app.store.now";
const P_TEST: &str = "test:py/tests/test_session.py#py.tests.test_session.test_refresh_known_token";

// TypeScript
const T_REFRESH: &str = "sym:web/src/session.ts#refresh";
const T_SESSION: &str = "sym:web/src/session.ts#Session";
const T_CTOR: &str = "sym:web/src/session.ts#Session.constructor";
const T_FRESH: &str = "sym:web/src/session.ts#Session.isFresh";
const T_STORE: &str = "sym:web/src/store.ts#TokenStore";
const T_LOOKUP: &str = "sym:web/src/store.ts#TokenStore.lookup";
const T_INSERT: &str = "sym:web/src/store.ts#TokenStore.insert";
const T_NOW: &str = "sym:web/src/store.ts#now";
const T_TEST: &str = "test:web/src/session.test.ts#refresh.returns a session for a known token";

#[test]
fn gate0_rust() {
    let f = setup();
    let e = &f.engine;
    check(
        e,
        &Expect {
            id: R_REFRESH,
            calls: &[R_LOOKUP, R_SESSION_NEW, R_FRESH],
            called_by: &[],
            tests: &[R_UNIT, R_INTEG],
            all_tests: &[R_UNIT, R_INTEG],
        },
    );
    check(
        e,
        &Expect {
            id: R_LOOKUP,
            calls: &[],
            called_by: &[R_REFRESH],
            tests: &[],
            all_tests: &[R_UNIT, R_INTEG],
        },
    );
    check(
        e,
        &Expect {
            id: R_NOW,
            calls: &[],
            called_by: &[R_SESSION_NEW, R_FRESH],
            tests: &[],
            all_tests: &[R_UNIT, R_INTEG],
        },
    );
    check(
        e,
        &Expect {
            id: R_STORE_NEW,
            calls: &[],
            called_by: &[],
            tests: &[R_UNIT, R_INTEG],
            all_tests: &[R_UNIT, R_INTEG],
        },
    );
    check(
        e,
        &Expect {
            id: R_INSERT,
            calls: &[],
            called_by: &[],
            tests: &[R_INTEG],
            all_tests: &[R_INTEG],
        },
    );
    let r = impact(e, R_NOW);
    assert_eq!(
        ids(r.indirect_callers.iter().map(|(l, _)| l)),
        set(&[R_REFRESH])
    );
}

#[test]
fn gate0_python() {
    let f = setup();
    let e = &f.engine;
    check(
        e,
        &Expect {
            id: P_REFRESH,
            calls: &[P_LOOKUP, P_SESSION, P_FRESH],
            called_by: &[],
            tests: &[P_TEST],
            all_tests: &[P_TEST],
        },
    );
    // now is reached by the test through is_fresh -> refresh -> test.
    check(
        e,
        &Expect {
            id: P_NOW,
            calls: &[],
            called_by: &[P_INIT, P_FRESH],
            tests: &[],
            all_tests: &[P_TEST],
        },
    );
    check(
        e,
        &Expect {
            id: P_LOOKUP,
            calls: &[],
            called_by: &[P_REFRESH],
            tests: &[],
            all_tests: &[P_TEST],
        },
    );
    check(
        e,
        &Expect {
            id: P_INSERT,
            calls: &[],
            called_by: &[],
            tests: &[P_TEST],
            all_tests: &[P_TEST],
        },
    );
    let t = card(e, P_TEST);
    assert_eq!(ids(&t.calls), set(&[P_STORE, P_INSERT, P_REFRESH]));
}

#[test]
fn gate0_typescript() {
    let f = setup();
    let e = &f.engine;
    check(
        e,
        &Expect {
            id: T_REFRESH,
            calls: &[T_LOOKUP, T_SESSION, T_FRESH],
            called_by: &[],
            tests: &[T_TEST],
            all_tests: &[T_TEST],
        },
    );
    check(
        e,
        &Expect {
            id: T_NOW,
            calls: &[],
            called_by: &[T_CTOR, T_FRESH],
            tests: &[],
            all_tests: &[T_TEST],
        },
    );
    check(
        e,
        &Expect {
            id: T_LOOKUP,
            calls: &[],
            called_by: &[T_REFRESH],
            tests: &[],
            all_tests: &[T_TEST],
        },
    );
    let t = card(e, T_TEST);
    assert_eq!(ids(&t.calls), set(&[T_STORE, T_INSERT, T_REFRESH]));
}

#[test]
fn gate0_whole_graph_is_clean() {
    let f = setup();
    let c = &f.engine.db().conn;
    // No call crosses languages, and nothing in this fixture needs guessing.
    let crossing: i64 = c
        .query_row(
            "SELECT COUNT(*) FROM edges e JOIN nodes a ON a.id = e.src JOIN nodes b ON b.id = e.dst
             WHERE e.rel = 'calls' AND a.lang != b.lang",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(crossing, 0);
    let mut stmt = c
        .prepare("SELECT src, dst FROM edges WHERE rel = 'calls' AND status = 'inferred'")
        .unwrap();
    let inferred: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert!(
        inferred.is_empty(),
        "unexpected inferred edges: {inferred:?}"
    );
    let total: i64 = c
        .query_row("SELECT COUNT(*) FROM edges WHERE rel = 'calls'", [], |r| {
            r.get(0)
        })
        .unwrap();
    // 3 languages x (refresh: 3, Session ctor/new: 1, is_fresh: 1, tests' calls) — counted exactly:
    // Rust 3+1+1 + unit(2) + integ(3) = 10; Python 3+1+1 + test(3) = 8; TS 3+1+1 + test(3) = 8.
    assert_eq!(total, 26);
}

#[test]
fn explain_lookup_forms() {
    let f = setup();
    let e = &f.engine;
    // Qualified name with either separator, path:line, and a bare name that is ambiguous.
    for q in [
        "auth::session::refresh",
        "app.session.Session.is_fresh",
        "src/auth/session.rs:21",
    ] {
        assert!(
            matches!(e.explain(q).unwrap(), Lookup::Found(Explained::Symbol(_))),
            "{q}"
        );
    }
    assert!(matches!(
        e.explain("refresh").unwrap(),
        Lookup::Ambiguous { .. }
    ));
    // Same qualified tail in Rust and Python: ambiguous, not a guess.
    assert!(matches!(
        e.explain("session.Session.is_fresh").unwrap(),
        Lookup::Ambiguous { .. }
    ));
    assert!(matches!(
        e.explain("src/store.rs").unwrap(),
        Lookup::Found(Explained::File(_))
    ));
    assert!(matches!(
        e.explain("no_such_symbol").unwrap(),
        Lookup::NotFound { .. }
    ));
    let c = card(e, R_REFRESH);
    assert_eq!(
        c.node.doc.as_deref(),
        Some("Issues a new session when the token is known.")
    );
    assert_eq!(
        c.node.signature.as_deref(),
        Some("pub fn refresh(store: &TokenStore, token: &str) -> Option<Session>")
    );
    assert_eq!(c.commits.len(), 1);
}

#[test]
fn context_pack_is_relevant_and_bounded() {
    let f = setup();
    let e = &f.engine;
    let p = e
        .context(
            "refresh returns None for an unknown token in the TokenStore",
            Some(600),
        )
        .unwrap();
    assert!(p.tokens <= 600, "{} tokens", p.tokens);
    assert!(
        p.items
            .iter()
            .any(|i| i == R_REFRESH || i == P_REFRESH || i == T_REFRESH),
        "{:?}",
        p.items
    );
    assert!(p.text.starts_with("Myrn context"));
    // Nothing relevant: nothing injected.
    let empty = e
        .context("translate the marketing copy into French", None)
        .unwrap();
    assert!(empty.text.is_empty(), "{}", empty.text);
    let brief = e.brief(None).unwrap();
    assert!(
        brief.tokens <= 500 && brief.text.contains("entry points"),
        "{}",
        brief.text
    );
}

#[test]
fn incremental_refresh_tracks_edits_and_deletes() {
    let mut f = setup();
    // Add a caller of store::now in a new file.
    std::fs::write(
        f.root.join("src/clock.rs"),
        "pub fn tick() -> u64 {\n    crate::store::now() + 1\n}\n",
    )
    .unwrap();
    let r = f.engine.refresh().unwrap();
    assert_eq!((r.parsed, r.removed), (1, 0));
    let callers = ids(&card(&f.engine, R_NOW).called_by);
    assert!(
        callers.contains("sym:src/clock.rs#clock::tick"),
        "{callers:?}"
    );

    // Editing only formatting re-parses the file but keeps the body hash.
    let before = card(&f.engine, "sym:src/clock.rs#clock::tick")
        .node
        .body_hash;
    std::fs::write(
        f.root.join("src/clock.rs"),
        "// comment\npub fn tick() -> u64 { crate::store::now()   + 1 }\n",
    )
    .unwrap();
    f.engine.refresh().unwrap();
    let after = card(&f.engine, "sym:src/clock.rs#clock::tick")
        .node
        .body_hash;
    assert_eq!(before, after);

    // Deleting the file removes its symbols and the edges into store::now.
    std::fs::remove_file(f.root.join("src/clock.rs")).unwrap();
    let r = f.engine.refresh().unwrap();
    assert_eq!(r.removed, 1);
    assert_eq!(
        ids(&card(&f.engine, R_NOW).called_by),
        set(&[R_SESSION_NEW, R_FRESH])
    );

    // No changes: nothing re-parsed.
    let r = f.engine.refresh().unwrap();
    assert_eq!((r.parsed, r.removed, r.commits), (0, 0, 0));

    // A new commit is picked up by history.
    std::fs::write(
        f.root.join("src/store.rs"),
        std::fs::read_to_string(fixture().join("src/store.rs")).unwrap() + "\npub fn later() {}\n",
    )
    .unwrap();
    commit_all(&f.root, "Add later()");
    let r = f.engine.refresh().unwrap();
    assert_eq!(r.commits, 1);
    match f.engine.history("src/store.rs", 10).unwrap() {
        Lookup::Found(h) => assert_eq!(
            h.iter().map(|c| c.summary.as_str()).collect::<Vec<_>>(),
            ["Add later()", "Add token store and session refresh"]
        ),
        _ => panic!("no history"),
    }

    // Rebuild gives the same graph.
    let before = f.engine.status().unwrap();
    f.engine.rebuild().unwrap();
    let after = f.engine.status().unwrap();
    assert_eq!(
        (before.symbols, before.edges.clone()),
        (after.symbols, after.edges.clone())
    );
    let _ = &f.home;
}
