//! Data behind `myrn ui`: code graph, notes graph, project map, records, details.

use std::path::Path;

use myrn_engine::api::Engine;
use serde_json::json;

fn write(root: &Path, files: &[(&str, &str)]) {
    for (p, c) in files {
        let p = root.join(p);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, c).unwrap();
    }
}

fn repo(tmp: &Path) -> Engine {
    let root = tmp.join("repo");
    write(
        &root,
        &[
            (
                "pkg/__init__.py",
                "from .recipes import *\nfrom .more import *\n",
            ),
            (
                "pkg/recipes.py",
                "from .more import chunked\n\ndef unique(xs):\n    return chunked(xs, 1)\n",
            ),
            ("pkg/more.py", "def chunked(it, n):\n    return [it]\n"),
        ],
    );
    let r = git2::Repository::init(&root).unwrap();
    let mut index = r.index().unwrap();
    index
        .add_all(["*"], git2::IndexAddOption::DEFAULT, None)
        .unwrap();
    index.write().unwrap();
    let tree = r.find_tree(index.write_tree().unwrap()).unwrap();
    let sig = git2::Signature::now("t", "t@example.invalid").unwrap();
    r.commit(Some("HEAD"), &sig, &sig, "first", &tree, &[])
        .unwrap();
    Engine::init(&root, Some(&tmp.join("home"))).unwrap().0
}

#[test]
fn code_graph_links_files_by_imports_and_calls() {
    let tmp = tempfile::tempdir().unwrap();
    let e = repo(tmp.path());
    let g = e.viz_code_graph("files", None, 1).unwrap();
    let ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
    assert!(ids.contains(&"file:pkg/more.py"), "{ids:?}");
    assert!(
        g.links
            .iter()
            .any(|l| l.s == "file:pkg/recipes.py" && l.t == "file:pkg/more.py"),
        "{:?}",
        g.links
    );
    let dirs = e.viz_code_graph("dirs", None, 1).unwrap();
    assert!(dirs.nodes.iter().all(|n| n.ntype == "module"));
    let focus = e
        .viz_code_graph("focus", Some("file:pkg/more.py"), 1)
        .unwrap();
    assert!(focus.nodes.iter().any(|n| n.id == "file:pkg/more.py"));
    assert!(focus.nodes.len() > 1);
}

#[test]
fn notes_graph_map_records_and_details() {
    let tmp = tempfile::tempdir().unwrap();
    let mut e = repo(tmp.path());
    let a = e
        .record_json(
            &json!({"type": "fact", "body": "chunked wraps its input in one list.",
            "about": "chunked", "evidence": [{"symbol": "chunked"}]}),
        )
        .unwrap();
    assert!(a.accepted, "{}", a.render());
    let a = a.ids[0].clone();
    let b = e
        .record_json(
            &json!({"type": "decision", "body": format!("Keep chunked simple; see [[{a}]]."),
            "about": "chunked", "evidence": [{"symbol": "chunked"}]}),
        )
        .unwrap();
    assert!(b.accepted, "{}", b.render());
    let b = b.ids[0].clone();

    // Notes graph: both notes, linked by the wikilink; with code, the symbol too.
    let g = e.viz_notes_graph(false).unwrap();
    assert_eq!(g.nodes.len(), 2, "{:?}", g.nodes);
    assert!(g
        .nodes
        .iter()
        .all(|n| n.visibility.as_deref() == Some("private")));
    assert!(
        g.links
            .iter()
            .any(|l| (l.s == b && l.t == a) || (l.s == a && l.t == b)),
        "{:?}",
        g.links
    );
    let gc = e.viz_notes_graph(true).unwrap();
    assert!(gc.nodes.iter().any(|n| n.group == "code"), "{:?}", gc.nodes);

    // Project map: the tree reaches the symbol; nothing summarized yet.
    let m = e.viz_map().unwrap();
    let root = m.root.expect("root");
    fn find<'a>(
        e: &'a myrn_engine::viz::MapEntry,
        name: &str,
    ) -> Option<&'a myrn_engine::viz::MapEntry> {
        if e.name == name {
            return Some(e);
        }
        e.children.iter().find_map(|c| find(c, name))
    }
    let sym = find(&root, "pkg.more.chunked").or_else(|| find(&root, "chunked"));
    assert!(sym.is_some(), "chunked not in the map");
    assert_eq!(sym.unwrap().state, "missing");
    assert_eq!(m.fresh, 0);
    assert!(m.missing > 0);

    // Records and details.
    let r = e.viz_records().unwrap();
    assert_eq!(r.records.len(), 2);
    let d = e.viz_details(&a).unwrap();
    assert!(d.body.unwrap().contains("chunked wraps"));
    assert!(d.inc.iter().any(|n| n.id == b), "{:?}", d.inc);
    assert!(e.viz_sessions(10).unwrap().is_empty());
}
