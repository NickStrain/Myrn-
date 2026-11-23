//! The record tool, distiller checks and recall (Step 16): evidence and
//! vocabulary checks reject with actionable reasons; accepted records become
//! private unverified notes with anchors and a produced edge; repeated bugs
//! update the known one; contradictions are flagged and superseded records drop
//! out; recall matches errors by signature; summaries show on cards and are
//! flagged when the code changes; the session-end request is asked once per
//! batch of edits; context packs quote known records.

use std::path::{Path, PathBuf};

use myrn_engine::api::{Engine, Explained, Lookup};
use myrn_store::episodes::{self, Entry};
use myrn_store::graph;
use serde_json::json;

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

const EVERSEEN: &str = "sym:pkg/recipes.py#pkg.recipes.unique_everseen";
const CHUNKED: &str = "sym:pkg/more.py#pkg.more.chunked";

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
    e: Engine,
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
        .commit(Some("HEAD"), &sig, &sig, "fix unhashable items", &tree, &[])
        .unwrap()
        .to_string();
    let (e, _) = Engine::init(&root, Some(&tmp.path().join("home"))).unwrap();
    Repo {
        _tmp: tmp,
        root,
        e,
        head,
    }
}

/// A recorded session: an edit to unique_everseen, a failing then passing test run.
fn session(e: &Engine, id: &str) {
    let dir = e.layout.episodes_dir();
    let mut entries = vec![
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
            outcome: Some("failure".into()),
            command: Some("pytest tests/test_recipes.py".into()),
            error: Some("TypeError: unhashable type: 'list'".into()),
            test: Some("fail".into()),
            ..Default::default()
        },
        Entry {
            t: 3,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("file_edit".into()),
            files: vec!["pkg/recipes.py".into()],
            symbols: vec![EVERSEEN.into()],
            ..Default::default()
        },
    ];
    entries.push(Entry {
        t: 4,
        agent: "claude".into(),
        event: "tool".into(),
        outcome: Some("other".into()),
        command: Some("pytest tests/test_recipes.py".into()),
        test: Some("pass".into()),
        ..Default::default()
    });
    for en in &entries {
        episodes::append(&dir, id, en).unwrap();
    }
    e.reindex_episodes().unwrap();
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

fn node(e: &Engine, id: &str) -> graph::Node {
    graph::node(&e.db().conn, id).unwrap().unwrap()
}

fn bug(head: &str, error: &str) -> serde_json::Value {
    json!({
        "type": "bug",
        "title": "unique_everseen fails on unhashable items",
        "body": "unique_everseen raised TypeError when the iterable held lists.",
        "error": error,
        "root_cause": "The seen set needs hashable elements and nothing falls back to a list.",
        "located_in": "unique_everseen",
        "fix": "Catch TypeError and keep unhashable items in a list.",
        "fixed_in": &head[..8],
        "evidence": [{"test": "test_everseen"}, {"test_run": "pytest tests/test_recipes.py"}]
    })
}

#[test]
fn rejections_say_what_to_fix() {
    let mut r = repo();
    let reject = |r: &mut Repo, v: serde_json::Value| {
        let o = r.e.record_json(&v).unwrap();
        assert!(!o.accepted, "{v}");
        o.messages.join("\n")
    };
    let m = reject(
        &mut r,
        json!({"type": "fact", "body": "Chunks may be short at the end."}),
    );
    assert!(m.contains("`evidence` must cite"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "fact", "body": "Chunks may be short at the end.", "evidence": [{"symbol": "no_such_fn"}]}),
    );
    assert!(m.contains("`no_such_fn` matches no symbol"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "opinion", "body": "x", "evidence": []}),
    );
    assert!(m.contains("`type` must be one of"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "fact", "body": "Chunks may be short.", "evidence": [{"symbol": "chunked"}],
               "relations": [{"rel": "depends_on", "target": "chunked"}]}),
    );
    assert!(m.contains("relation `depends_on` is not allowed"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "fact", "body": "Chunks may be short.", "evidence": [{"symbol": "chunked"}], "severity": "high"}),
    );
    assert!(m.contains("unknown field `severity`"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "bug", "body": "It broke on lists.", "root_cause": "no fallback", "evidence": [{"symbol": "chunked"}]}),
    );
    assert!(m.contains("needs `located_in`"), "{m}");
    let m = reject(
        &mut r,
        json!({"type": "fact", "body": "Chunks may be short.", "evidence": [{"test_run": "cargo test nope"}]}),
    );
    assert!(m.contains("not found among the test commands"), "{m}");
    // Nothing was written.
    assert_eq!(r.e.status().unwrap().notes.private, 0);
}

#[test]
fn bugs_are_stored_deduplicated_and_recalled() {
    let mut r = repo();
    session(&r.e, "s1");
    let o = r
        .e
        .record_json(&bug(
            &r.head,
            "Traceback (most recent call last):\n  File \"/home/a/repo/pkg/recipes.py\", line 4, in unique_everseen\nTypeError: unhashable type: 'list'",
        ))
        .unwrap();
    assert!(o.accepted && !o.updated, "{}", o.render());
    let (bug_id, rc_id) = (o.ids[0].clone(), o.ids[1].clone());
    assert!(bug_id.starts_with("bug-") && rc_id.starts_with("rootcause-"));

    // Written as a private note first, then indexed.
    let file =
        r.e.layout
            .notes_dir()
            .join("bugs")
            .join(format!("{bug_id}.md"));
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("status: unverified"), "{text}");
    assert!(
        text.contains("signature: 'TypeError: unhashable type: ''list'''"),
        "{text}"
    );
    assert!(
        text.contains(&format!("caused_by: '[[{rc_id}]]'")),
        "{text}"
    );
    assert!(text.contains("source: episode:s1"), "{text}");
    let b = node(&r.e, &bug_id);
    assert_eq!(b.status.as_deref(), Some("unverified"));
    assert!(has_edge(&r.e, &bug_id, "caused_by", &rc_id));
    assert!(has_edge(&r.e, &rc_id, "located_in", EVERSEEN));
    assert!(has_edge(
        &r.e,
        &format!("commit:{}", r.head),
        "fixes",
        &bug_id
    ));
    assert!(has_edge(&r.e, "episode:s1", "produced", &bug_id));
    // Anchors capture the current body hash.
    let current = node(&r.e, EVERSEEN).body_hash.unwrap();
    let anchored: String =
        r.e.db()
            .conn
            .query_row(
                "SELECT body_hash FROM anchors WHERE note_id = ?1 AND symbol_id = ?2",
                [&rc_id, EVERSEEN],
                |x| x.get(0),
            )
            .unwrap();
    assert!(current.starts_with(&anchored));

    // The same bug seen again elsewhere updates the record instead of copying it.
    let mut again = bug(
        &r.head,
        "Traceback (most recent call last):\n  File \"C:\\work\\repo\\pkg\\recipes.py\", line 9, in unique_everseen\nTypeError: unhashable type: 'list'",
    );
    again["body"] = json!("Also happens with dicts inside the iterable.");
    again["evidence"] = json!([{"symbol": "unique_everseen"}]);
    let o = r.e.record_json(&again).unwrap();
    assert!(o.accepted && o.updated, "{}", o.render());
    assert_eq!(o.ids[0], bug_id);
    assert_eq!(
        r.e.status().unwrap().notes.private,
        2,
        "one bug, one root cause"
    );
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(text.contains("Also happens with dicts"), "{text}");
    assert!(
        text.contains("symbol: pkg.recipes.unique_everseen"),
        "{text}"
    );

    // Recall with a differently-shaped trace of the same error.
    let (recall, ids) =
        r.e.recall_text(
            "E   TypeError: unhashable type: 'list'\n/tmp/x/pkg/recipes.py:5: TypeError",
            500,
        )
        .unwrap();
    assert_eq!(ids, std::slice::from_ref(&bug_id));
    assert!(recall.contains("same signature"), "{recall}");
    assert!(
        recall.contains("The seen set needs hashable elements"),
        "{recall}"
    );
    assert!(recall.contains("pkg.recipes.unique_everseen"), "{recall}");
    assert!(recall.contains("fix: Catch TypeError"), "{recall}");
    assert!(recall.contains("fixed in:"), "{recall}");
    assert!(r
        .e
        .recall("ValueError: something else", 3)
        .unwrap()
        .is_empty());

    // The card and a context pack show it as known.
    let Lookup::Found(Explained::Symbol(card)) = r.e.explain("unique_everseen").unwrap() else {
        panic!()
    };
    let rendered = card.render(10);
    assert!(
        rendered.contains(&bug_id) && rendered.contains(&rc_id),
        "{rendered}"
    );
    let pack =
        r.e.context("why does `unique_everseen` fail on lists?", None)
            .unwrap();
    assert!(
        pack.text.contains("Known from past sessions"),
        "{}",
        pack.text
    );
    assert!(
        pack.text.contains(&format!(
            "> {rc_id} · root_cause · unverified · private, written by claude on "
        )),
        "{}",
        pack.text
    );
    assert!(pack.items.contains(&bug_id), "{:?}", pack.items);

    // The index can be rebuilt from the notes alone.
    r.e.rebuild().unwrap();
    assert!(has_edge(&r.e, &bug_id, "caused_by", &rc_id));
    assert_eq!(
        r.e.recall("TypeError: unhashable type: 'list'", 3)
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn contradictions_are_flagged_and_superseded_records_drop_out() {
    let mut r = repo();
    let fact = |body: &str| json!({"type": "fact", "body": body, "about": "chunked", "evidence": [{"symbol": "chunked"}]});
    let a =
        r.e.record_json(&fact(
            "chunked keeps a short last chunk when the input does not divide evenly.",
        ))
        .unwrap();
    assert!(a.accepted, "{}", a.render());
    let a_id = a.ids[0].clone();
    assert!(has_edge(&r.e, &a_id, "about", CHUNKED));

    let b =
        r.e.record_json(&fact(
            "chunked never keeps a short last chunk when the input does not divide evenly.",
        ))
        .unwrap();
    assert!(b.accepted, "{}", b.render());
    let b_id = b.ids[0].clone();
    assert!(has_edge(&r.e, &b_id, "contradicts", &a_id));
    assert!(
        b.render().contains(&format!("conflicts with {a_id}")),
        "{}",
        b.render()
    );

    // An unrelated fact about the same code is not a contradiction.
    let c =
        r.e.record_json(&fact(
            "chunked returns lists, not tuples, for every chunk produced.",
        ))
        .unwrap();
    assert!(!has_edge(&r.e, &c.ids[0], "contradicts", &a_id));

    // Superseding: the old record is marked in its file and leaves the card.
    let mut d = fact("chunked keeps the short last chunk; pass strict=True to raise instead.");
    d["relations"] =
        json!([{"rel": "supersedes", "target": a_id}, {"rel": "supersedes", "target": b_id}]);
    let d = r.e.record_json(&d).unwrap();
    assert!(d.accepted, "{}", d.render());
    assert_eq!(node(&r.e, &a_id).status.as_deref(), Some("superseded"));
    let file =
        r.e.layout
            .notes_dir()
            .join("facts")
            .join(format!("{a_id}.md"));
    assert!(std::fs::read_to_string(file)
        .unwrap()
        .contains("status: superseded"));
    let Lookup::Found(Explained::Symbol(card)) = r.e.explain("chunked").unwrap() else {
        panic!()
    };
    let ids: Vec<&str> = card.known.iter().map(|k| k.id.as_str()).collect();
    assert!(
        ids.contains(&d.ids[0].as_str()) && !ids.contains(&a_id.as_str()),
        "{ids:?}"
    );
}

#[test]
fn summaries_show_on_cards_and_go_stale_with_the_code() {
    let mut r = repo();
    let s = json!({"type": "summary", "about": "chunked",
                   "body": "Splits an iterable into lists of n items; the last list may be shorter. Raises nothing on n=0 (returns everything)."});
    let o = r.e.record_json(&s).unwrap();
    assert!(o.accepted, "{}", o.render());
    assert!(o.ids[0].starts_with("summary-"));
    let card = |r: &Repo| {
        let Lookup::Found(Explained::Symbol(c)) = r.e.explain("chunked").unwrap() else {
            panic!()
        };
        c
    };
    let c = card(&r);
    assert!(!c.summary_outdated);
    assert!(
        c.render(5).contains("summary    Splits an iterable"),
        "{}",
        c.render(5)
    );

    // Code changes: the summary is shown as possibly outdated.
    write(&r.root, &[("pkg/more.py", "def chunked(it, n):\n    \"\"\"Break it into lists of length n.\"\"\"\n    return [it, n]\n")]);
    r.e.refresh().unwrap();
    let c = card(&r);
    assert!(c.summary_outdated);
    assert!(c.render(5).contains("possibly outdated"), "{}", c.render(5));

    // A new summary replaces the old one (still one summary note) with a fresh anchor.
    let mut s2 = s.clone();
    s2["body"] = json!("Splits an iterable into pairs of (item, n); placeholder until rewritten.");
    let o = r.e.record_json(&s2).unwrap();
    assert!(o.accepted && o.updated, "{}", o.render());
    let c = card(&r);
    assert!(!c.summary_outdated);
    assert!(c
        .node
        .summary
        .as_deref()
        .unwrap()
        .starts_with("Splits an iterable into pairs"));
    let n: i64 =
        r.e.db()
            .conn
            .query_row(
                "SELECT COUNT(*) FROM notes WHERE type = 'summary'",
                [],
                |x| x.get(0),
            )
            .unwrap();
    assert_eq!(n, 1);

    // Summaries need exactly one target.
    let o = r.e.record_json(&json!({"type": "summary", "about": ["chunked", "unique_everseen"], "body": "Two things at once is not a summary."})).unwrap();
    assert!(!o.accepted);
}

#[test]
fn session_end_asks_once_per_batch_of_edits() {
    let mut r = repo();
    assert!(
        r.e.records_request("nobody", false).unwrap().is_none(),
        "no log, no consent"
    );
    session(&r.e, "s2");
    let ask =
        r.e.records_request("s2", false)
            .unwrap()
            .expect("edited code, so ask");
    assert!(ask.contains("record"), "{ask}");
    assert!(ask.contains("pkg/recipes.py"), "{ask}");
    assert!(ask.contains("TypeError: unhashable type"), "{ask}");
    assert!(
        ask.contains("pkg.recipes.unique_everseen"),
        "summary requested: {ask}"
    );
    // Asked already, no new edits since.
    assert!(r.e.records_request("s2", false).unwrap().is_none());
    // Never while the agent is continuing because of the request.
    episodes::append(
        &r.e.layout.episodes_dir(),
        "s2",
        &Entry {
            t: 9,
            agent: "claude".into(),
            event: "tool".into(),
            outcome: Some("file_edit".into()),
            files: vec!["pkg/more.py".into()],
            ..Default::default()
        },
    )
    .unwrap();
    assert!(r.e.records_request("s2", true).unwrap().is_none());
    assert!(r.e.records_request("s2", false).unwrap().is_some());
    // The episode still folds normally with the request markers in it.
    assert_eq!(r.e.episodes(5).unwrap()[0].files_edited.len(), 2);
}

fn consent(e: &Engine) {
    let mut st = e.state().unwrap();
    st.capture_consent = true;
    st.agents.insert(
        "claude".into(),
        myrn_store::state::AgentState {
            mode: "hooks".into(),
            connected_time: 0,
            written: vec![],
            created: vec![],
        },
    );
    e.save_state(&st).unwrap();
}

fn bash(session: &str, command: &str) -> myrn_model::event::Event {
    use myrn_model::event::{Event, EventKind, ToolOutcome, ToolResult};
    Event {
        session_id: session.into(),
        cwd: None,
        kind: EventKind::ToolResult(ToolResult {
            tool: "Bash".into(),
            outcome: ToolOutcome::Other,
            files: vec![],
            command: Some(command.into()),
            error: None,
        }),
    }
}

#[test]
fn shell_edits_are_captured_and_lead_to_a_record_request() {
    let mut r = repo();
    consent(&r.e);
    // A read-only command changes nothing.
    r.e.capture("claude", &bash("s3", "grep -n chunked pkg/more.py"))
        .unwrap();
    assert!(r.e.records_request("s3", false).unwrap().is_none());

    // `sed -i` changes a function body behind the agent's edit tools.
    write(
        &r.root,
        &[("pkg/more.py", "def chunked(it, n):\n    \"\"\"Break it into lists of length n.\"\"\"\n    return [list(it)]\n")],
    );
    r.e.capture(
        "claude",
        &bash("s3", "sed -i 's/\\[it\\]/[list(it)]/' pkg/more.py"),
    )
    .unwrap();
    let log = episodes::read(&episodes::path_for(&r.e.layout.episodes_dir(), "s3")).unwrap();
    let edit = log
        .iter()
        .find(|e| e.via.as_deref() == Some("shell"))
        .expect("shell edit entry");
    assert_eq!(edit.outcome.as_deref(), Some("file_edit"));
    assert_eq!(edit.files, ["pkg/more.py"]);
    assert_eq!(edit.symbols, [CHUNKED]);
    let ep = &r.e.episodes(1).unwrap()[0];
    assert!(ep.files_edited.contains("pkg/more.py"));
    let ask =
        r.e.records_request("s3", false)
            .unwrap()
            .expect("asked after a shell edit");
    assert!(
        ask.contains("ONE call") && ask.contains("pkg.more.chunked"),
        "{ask}"
    );
}

#[test]
fn one_call_records_many_items_and_trims_test_runs() {
    let mut r = repo();
    session(&r.e, "s4");
    let outcomes = r
        .e
        .record_any_json(&json!({"session": "s4", "items": [
            {"type": "fact", "body": "chunked keeps a short last chunk when sizes do not divide.",
             "about": "chunked", "evidence": [{"symbol": "chunked"}]},
            {"type": "summary", "about": "unique_everseen",
             "body": "Yields each element once, in first-seen order; needs hashable elements."},
            {"type": "fact", "body": "Something with no evidence at all here."},
            {"type": "workflow", "body": "Run the recipes tests on their own.",
             "steps": ["pytest tests/test_recipes.py"],
             "evidence": [{"test_run": "cd /home/x/repo && pytest tests/test_recipes.py 2>&1 | tail -3"}]}
        ]}))
        .unwrap();
    let accepted: Vec<bool> = outcomes.iter().map(|o| o.accepted).collect();
    assert_eq!(accepted, [true, true, false, true]);
    // Every accepted item carries the batch's session.
    let wf = &outcomes[3].ids[0];
    let text = std::fs::read_to_string(
        r.e.layout
            .notes_dir()
            .join("workflows")
            .join(format!("{wf}.md")),
    )
    .unwrap();
    assert!(text.contains("source: episode:s4"), "{text}");
    assert!(
        text.contains("test_run: pytest tests/test_recipes.py\n"),
        "{text}"
    );
    assert!(!text.contains("/home/x"), "{text}");
}
