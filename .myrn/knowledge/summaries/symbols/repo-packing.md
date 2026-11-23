---
id: repo-packing
type: summary
status: unverified
title: repo
source: myrn summarize
about: sym:crates/myrn-engine/tests/packing.rs#repo
anchors:
- sym:crates/myrn-engine/tests/packing.rs#repo@ca818524d3ba37df
map_version: '2'
---
# repo

Creates a temp dir with a 'repo' subdirectory, writes the files, runs git init, stages everything and makes a root commit 'first' on HEAD (author 't'). Then calls Engine::init on that root with home at tmp/home and returns the TempDir and Engine.

## Behaviour

- files given -> written, staged and committed as a single parentless commit
- empty slice -> commits an empty tree and still returns a working Engine
- TempDir is returned, so the repo is deleted when it is dropped; keep it alive while using the Engine

## Terms

fixture repository, temp git repo, git init, initial commit, test repo, engine setup, temporary directory, root commit

## Connections

- **Location:** `crates/myrn-engine/tests/packing.rs:17-32`
- **Signature:** `fn repo(files: &[(&str, &str)]) -> (tempfile::TempDir, Engine)`
- **In:** [[file-crates-myrn-engine-tests-packing-rs]]
- **Calls:** [[engine-init]], [[write-packing]]
- **Tests:** `python_namespace_calls_through_wildcard_reexports_link_tests`, `typescript_namespace_calls_through_export_star_link_tests`, `packs_are_small_carry_code_and_skip_noise`
