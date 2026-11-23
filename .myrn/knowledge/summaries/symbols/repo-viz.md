---
id: repo-viz
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/viz.rs#repo
anchors:
- sym:crates/myrn-engine/tests/viz.rs#repo@bd5a6c1c73c544bc
map_version: '2'
---
# repo

Builds a sample Python project at tmp/repo, git-initialises it, commits every file as 'first', and returns an Engine initialised on it with home at tmp/home.

## Behaviour

- repo(tmp) -> Engine over tmp/repo with pkg/__init__.py, pkg/recipes.py, pkg/more.py committed once
- pkg/recipes.py imports chunked from pkg/more.py, so the code graph gets a recipes -> more link
- Returns Engine::init(...).0, dropping the second tuple element; any git or write failure panics

## Terms

test repository, git fixture, temp repo, initial commit, sample project, engine setup, fixture project

## Connections

- **Location:** `crates/myrn-engine/tests/viz.rs:16-43`
- **Signature:** `fn repo(tmp: &Path) -> Engine`
- **In:** [[file-crates-myrn-engine-tests-viz-rs]]
- **Calls:** [[engine-init]], [[write-viz]]
- **Tests:** `code_graph_links_files_by_imports_and_calls`, `notes_graph_map_records_and_details`
