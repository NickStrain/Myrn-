//! Markdown notes and note indexing (Step 15): notes in the private and
//! shared folders become nodes, typed edges, `related` edges, anchors and
//! symbol summaries; hand edits are picked up; malformed notes and dangling
//! links are indexed as far as possible and flagged; `rebuild` recreates the
//! same index from the code and the notes.

use std::path::{Path, PathBuf};

use myrn_engine::api::Engine;
use myrn_model::Visibility;
use myrn_store::graph;
use myrn_store::notes::{Link, Note};

const PY: &[(&str, &str)] = &[
    ("pkg/__init__.py", "from .more import *\nfrom .recipes import *\n"),
    (
        "pkg/more.py",
        "def chunked(it, n):\n    \"\"\"Break it into lists of length n.\"\"\"\n    return [it]\n",
    ),
    (
        "pkg/recipes.py",
        "def unique_everseen(iterable):\n    seen = set()\n    for x in iterable:\n        if x not in seen:\n            seen.add(x)\n            yield x\n",
    ),
    (
        "tests/test_recipes.py",
        "import pkg as mi\n\ndef test_everseen():\n    assert list(mi.unique_everseen('aab')) == ['a', 'b']\n",
    ),
];

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

struct Repo {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    engine: Engine,
    head: String,
}

fn repo() -> Repo {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("repo");
    write(&root, PY);
    let r = git2::Repository::init(&root).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    let head = r
        .commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap()
        .to_string();
    let (engine, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    Repo {
        _tmp: tmp,
        root,
        engine,
        head,
    }
}

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/notes")
}

/// Copy the fixture notes into `dest`, filling in the commit and body hash.
fn copy_fixtures(dest: &Path, head: &str, body_hash: &str) {
    fn rec(from: &Path, to: &Path, head: &str, hash: &str) {
        std::fs::create_dir_all(to).unwrap();
        for e in std::fs::read_dir(from).unwrap().flatten() {
            let (src, dst) = (e.path(), to.join(e.file_name()));
            if src.is_dir() {
                rec(&src, &dst, head, hash);
            } else {
                let text = std::fs::read_to_string(&src)
                    .unwrap()
                    .replace("HEAD_SHA", &head[..7])
                    .replace("BODY_HASH", hash);
                std::fs::write(dst, text).unwrap();
            }
        }
    }
    rec(&fixtures(), dest, head, body_hash);
}

fn node(e: &Engine, id: &str) -> Option<graph::Node> {
    graph::node(&e.db().conn, id).unwrap()
}

fn has_edge(e: &Engine, src: &str, rel: &str, dst: &str) -> bool {
    e.db()
        .conn
        .query_row(
            "SELECT 1 FROM edges WHERE src = ?1 AND rel = ?2 AND dst = ?3",
            [src, rel, dst],
            |_| Ok(()),
        )
        .is_ok()
}

fn body_hash(e: &Engine, id: &str) -> String {
    node(e, id).unwrap().body_hash.unwrap()
}

/// Everything the notes contribute to the index, in a comparable form.
fn note_snapshot(e: &Engine) -> Vec<String> {
    let c = &e.db().conn;
    let mut out = Vec::new();
    let mut q = |sql: &str| {
        let mut stmt = c.prepare(sql).unwrap();
        let rows: Vec<String> = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        out.extend(rows);
    };
    q("SELECT id || '|' || type || '|' || ifnull(title,'') || '|' || ifnull(status,'') || '|' || ifnull(doc,'')
       FROM nodes WHERE origin = 'earned' AND type != 'episode' ORDER BY id");
    q("SELECT src || ' ' || rel || ' ' || dst FROM edges WHERE evidence LIKE 'note:%' ORDER BY 1");
    q("SELECT note_id || ' ' || ifnull(symbol_id,'-') || ' ' || ifnull(body_hash,'-') FROM anchors ORDER BY 1");
    q("SELECT id || ' summary: ' || summary FROM nodes WHERE summary IS NOT NULL ORDER BY id");
    q("SELECT path || ' ' || kind || ' ' || detail FROM note_issues ORDER BY 1");
    q("SELECT id FROM nodes WHERE rowid IN (SELECT rowid FROM search WHERE search MATCH 'unhashable') ORDER BY 1");
    out
}

fn indexed_repo() -> Repo {
    let mut r = repo();
    let hash = body_hash(&r.engine, "sym:pkg/recipes.py#pkg.recipes.unique_everseen");
    copy_fixtures(&r.engine.layout.notes_dir(), &r.head, &hash[..6]);
    let report = r.engine.refresh().unwrap();
    assert_eq!(report.notes.notes, 5, "{:?}", report.notes);
    assert_eq!(report.notes.indexed, 5);
    r
}

#[test]
fn notes_become_nodes_edges_anchors_and_summaries() {
    let r = indexed_repo();
    let e = &r.engine;

    let bug = node(e, "bug-7f3a91").expect("bug node");
    assert_eq!((bug.ntype.as_str(), bug.origin.as_str()), ("bug", "earned"));
    assert_eq!(bug.status.as_deref(), Some("verified"));
    assert!(bug
        .title
        .unwrap()
        .starts_with("`unique_everseen` raised TypeError"));
    assert!(bug.extra.unwrap().contains("\"visibility\":\"private\""));

    // Typed header relations, including the inverse `fixed_in`.
    assert!(has_edge(e, "bug-7f3a91", "caused_by", "rootcause-a12b08"));
    assert!(has_edge(
        e,
        &format!("commit:{}", r.head),
        "fixes",
        "bug-7f3a91"
    ));
    assert!(has_edge(
        e,
        "rootcause-a12b08",
        "located_in",
        "sym:pkg/recipes.py#pkg.recipes.unique_everseen"
    ));
    assert!(has_edge(
        e,
        "fact-c0ffee",
        "about",
        "sym:pkg/more.py#pkg.more.chunked"
    ));
    // Body wikilinks are `related`; a link repeated in the header is still related in the body.
    assert!(has_edge(e, "bug-7f3a91", "related", "fact-c0ffee"));
    assert!(has_edge(e, "fact-c0ffee", "related", "bug-7f3a91"));
    assert!(has_edge(e, "hand-written", "related", "fact-c0ffee"));

    // Anchor resolved to the symbol row by qualified name.
    let anchored: String = e
        .db()
        .conn
        .query_row(
            "SELECT symbol_id FROM anchors WHERE note_id = 'bug-7f3a91'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(anchored, "sym:pkg/recipes.py#pkg.recipes.unique_everseen");

    // A summary note sets the symbol's summary; it is not a node itself.
    assert!(node(e, "summary-5e1f00").is_none());
    assert!(node(e, "sym:pkg/more.py#pkg.more.chunked")
        .unwrap()
        .summary
        .unwrap()
        .starts_with("Splits an iterable"));

    // Found by id, and by full-text search over the body.
    assert!(matches!(
        e.show("bug-7f3a91").unwrap(),
        myrn_engine::api::Lookup::Found(_)
    ));
    assert!(note_snapshot(e).iter().any(|l| l == "bug-7f3a91"));

    // The status counts.
    let s = e.status().unwrap();
    assert_eq!((s.notes.private, s.notes.shared), (5, 0));
    let unverified = s
        .records_by_state
        .iter()
        .find(|(k, _)| k == "unverified")
        .unwrap()
        .1;
    assert_eq!(unverified, 3); // rootcause, fact, hand-written
}

#[test]
fn malformed_notes_and_dangling_links_are_flagged_not_dropped() {
    let mut r = indexed_repo();
    let s = r.engine.status().unwrap();
    assert_eq!(s.notes.malformed, 1, "hand-written.md has no header");
    assert_eq!(s.notes.dangling, 1, "[[missing-note]]");
    let hand = node(&r.engine, "hand-written").expect("headerless note still indexed");
    assert_eq!(hand.ntype, "fact");

    let doctor = r.engine.doctor();
    let notes = doctor.iter().find(|c| c.name == "notes").unwrap();
    assert!(notes.detail.contains("missing-note"), "{}", notes.detail);
    assert!(notes.detail.contains("no YAML header"), "{}", notes.detail);

    // The dangling link resolves once its target appears (as from a git pull).
    write(
        &r.engine.layout.knowledge_dir(),
        &[(
            "facts/missing-note.md",
            "---\nid: missing-note\ntype: fact\n---\nNow here.\n",
        )],
    );
    let rep = r.engine.refresh().unwrap();
    assert_eq!(rep.notes.indexed, 1);
    assert!(has_edge(
        &r.engine,
        "fact-c0ffee",
        "related",
        "missing-note"
    ));
    let s = r.engine.status().unwrap();
    assert_eq!((s.notes.dangling, s.notes.shared), (0, 1));
}

#[test]
fn only_changed_notes_are_reindexed_including_hand_edits() {
    let mut r = indexed_repo();
    let rep = r.engine.refresh().unwrap();
    assert_eq!((rep.notes.indexed, rep.notes.relinked), (0, false));

    // Hand edit: new status and a new relation.
    let p = r.engine.layout.notes_dir().join("bugs/rootcause-a12b08.md");
    let text = std::fs::read_to_string(&p).unwrap();
    std::fs::write(
        &p,
        text.replace(
            "status: unverified",
            "status: suspect\nrelated: '[[fact-c0ffee]]'",
        ),
    )
    .unwrap();
    let rep = r.engine.refresh().unwrap();
    assert_eq!((rep.notes.indexed, rep.notes.relinked), (1, true));
    assert_eq!(
        node(&r.engine, "rootcause-a12b08")
            .unwrap()
            .status
            .as_deref(),
        Some("suspect")
    );
    assert!(has_edge(
        &r.engine,
        "rootcause-a12b08",
        "related",
        "fact-c0ffee"
    ));

    // Touched but identical content: not re-parsed.
    std::fs::write(&p, std::fs::read(&p).unwrap()).unwrap();
    assert_eq!(r.engine.refresh().unwrap().notes.indexed, 0);

    // Deleted: the node and its edges go.
    std::fs::remove_file(&p).unwrap();
    let rep = r.engine.refresh().unwrap();
    assert_eq!(rep.notes.removed, 1);
    assert!(node(&r.engine, "rootcause-a12b08").is_none());
    assert!(!has_edge(
        &r.engine,
        "rootcause-a12b08",
        "located_in",
        "sym:pkg/recipes.py#pkg.recipes.unique_everseen"
    ));
    // ...and the bug's link to it is now dangling.
    assert!(r.engine.status().unwrap().notes.dangling >= 1);
}

#[test]
fn links_and_summaries_survive_code_changes() {
    let mut r = indexed_repo();
    write(
        &r.root,
        &[(
            "pkg/more.py",
            "def chunked(it, n):\n    \"\"\"Break it into lists of length n.\"\"\"\n    return [it, n]\n",
        )],
    );
    let rep = r.engine.refresh().unwrap();
    assert!(rep.parsed > 0 && rep.notes.relinked);
    let chunked = node(&r.engine, "sym:pkg/more.py#pkg.more.chunked").unwrap();
    assert!(
        chunked.summary.is_some(),
        "re-parse must not lose the summary"
    );
    assert!(has_edge(
        &r.engine,
        "fact-c0ffee",
        "about",
        "sym:pkg/more.py#pkg.more.chunked"
    ));
}

#[test]
fn rebuild_recreates_the_same_index_from_code_and_notes() {
    let mut r = indexed_repo();
    let before = note_snapshot(&r.engine);
    assert!(before.len() > 15, "{before:#?}");
    let rep = r.engine.rebuild().unwrap();
    assert_eq!(rep.notes.indexed, 5);
    assert_eq!(note_snapshot(&r.engine), before);
}

#[test]
fn write_note_writes_first_then_indexes_and_shared_wins() {
    let mut r = repo();
    let mut n = Note::new("decision", "Keep the refresh window in config, not code.\n");
    n.links.push(Link {
        rel: myrn_model::Relation::About,
        target: "chunked".into(),
        inverse: false,
    });
    let path = r.engine.write_note(&mut n, Visibility::Private).unwrap();
    assert!(n.id.starts_with("decision-") && n.id.len() == "decision-".len() + 6);
    assert!(path.ends_with(format!("decisions/{}.md", n.id)));
    assert!(std::fs::read_to_string(&path)
        .unwrap()
        .contains("about: chunked"));
    assert!(has_edge(
        &r.engine,
        &n.id,
        "about",
        "sym:pkg/more.py#pkg.more.chunked"
    ));

    // A link to another note is written as an Obsidian wikilink.
    let mut f = Note::new("fact", "Windows are measured in seconds.\n");
    f.links.push(Link {
        rel: myrn_model::Relation::Related,
        target: n.id.clone(),
        inverse: false,
    });
    let fpath = r.engine.write_note(&mut f, Visibility::Private).unwrap();
    assert!(std::fs::read_to_string(fpath)
        .unwrap()
        .contains(&format!("'[[{}]]'", n.id)));

    // The same note published to the shared folder: one node, the shared copy wins,
    // the private copy is flagged as a duplicate.
    let mut shared = n.clone();
    shared.status = "verified".into();
    r.engine
        .write_note(&mut shared, Visibility::Shared)
        .unwrap();
    let node = node(&r.engine, &n.id).unwrap();
    assert_eq!(node.status.as_deref(), Some("verified"));
    assert!(node.extra.unwrap().contains("\"visibility\":\"shared\""));
    assert_eq!(r.engine.status().unwrap().notes.duplicates, 1);
}
