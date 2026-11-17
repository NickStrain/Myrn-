//! Context packs that answer rather than point (Gate 1 findings): re-exported
//! namespace calls link tests, stubs don't duplicate implementations, packs
//! stay small and carry short code, and the brief has no "most-called" list.

use std::path::Path;

use myrn_engine::api::{Engine, Lookup};

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

const PY: &[(&str, &str)] = &[
    ("pkg/__init__.py", "from .more import *\nfrom .recipes import *\n"),
    ("pkg/more.py", "def chunked(it, n):\n    \"\"\"Break it into lists of length n.\"\"\"\n    return [it]\n"),
    (
        "pkg/recipes.py",
        "def unique_everseen(iterable, key=None):\n    \"\"\"Yield unique elements. Raises TypeError for unhashable items.\"\"\"\n    seen = set()\n    for x in iterable:\n        if x not in seen:\n            seen.add(x)\n            yield x\n",
    ),
    ("pkg/recipes.pyi", "def unique_everseen(iterable, key=...): ...\n"),
    (
        "tests/test_recipes.py",
        "import pkg as mi\n\ndef test_everseen():\n    assert list(mi.unique_everseen('aab')) == ['a', 'b']\n\ndef test_chunked():\n    mi.chunked([1], 1)\n",
    ),
];

#[test]
fn python_namespace_calls_through_wildcard_reexports_link_tests() {
    let (_t, e) = repo(PY);
    let Lookup::Found(x) = e.explain("pkg.recipes.unique_everseen").unwrap() else {
        panic!("not found or ambiguous: the .pyi stub must not make this ambiguous");
    };
    let card = x.render();
    assert!(card.contains("test_everseen"), "{card}");
    assert!(card.contains("pkg/recipes.py:1"), "{card}");
}

#[test]
fn typescript_namespace_calls_through_export_star_link_tests() {
    let (_t, e) = repo(&[
        ("src/index.ts", "export * from \"./chunk\";\n"),
        (
            "src/chunk.ts",
            "export function chunk(data: number[], size: number): number[][] {\n  return [data];\n}\n",
        ),
        (
            "test/chunk.test.ts",
            "import * as R from \"../src/index\";\n\ntest(\"chunks\", () => {\n  R.chunk([1], 1);\n});\n",
        ),
    ]);
    let Lookup::Found(x) = e.explain("chunk").unwrap() else {
        panic!("chunk not found");
    };
    let card = x.render();
    assert!(card.contains("chunk.test.ts"), "{card}");
}

#[test]
fn packs_are_small_carry_code_and_skip_noise() {
    let (_t, e) = repo(PY);
    let pack = e
        .context(
            "what does unique_everseen do with items that cannot be hashed?",
            None,
        )
        .unwrap();
    let t = &pack.text;
    assert!(t.contains("code shown is current"), "{t}");
    // The answer itself: the function's code.
    assert!(t.contains("seen.add(x)"), "{t}");
    // Tests summarised in one line, not listed; no stub card; no commits.
    assert!(
        t.contains("tests      1 in tests/test_recipes.py (1)"),
        "{t}"
    );
    assert!(!t.contains("recipes.pyi"), "{t}");
    assert!(!t.contains("commit "), "{t}");
    assert!(pack.items.len() <= 5, "{:?}", pack.items);

    // Commits only when the task is about history.
    let h = e
        .context("when was unique_everseen changed in a commit?", None)
        .unwrap();
    assert!(h.text.contains("first"), "{}", h.text);

    // Nothing relevant: nothing injected.
    assert!(e
        .context("deploy the kubernetes cluster", None)
        .unwrap()
        .text
        .is_empty());

    let brief = e.brief(None).unwrap().text;
    assert!(!brief.contains("most-called"), "{brief}");
}
