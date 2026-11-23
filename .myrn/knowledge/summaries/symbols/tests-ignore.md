---
id: tests-ignore
type: summary
status: unverified
title: tests::ignore
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::tests::ignore
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::tests::ignore@796246bc75d78a52
map_version: '2'
---
# tests::ignore

Test helper that builds a Gitignore rooted at `root` containing the single rule `secrets/`.

## Behaviour

- root /r -> matcher that ignores secrets/prod.env
- a rule that fails to parse panics through unwrap

## Terms

test fixture, gitignore fixture, secrets rule, test helper, ignore rules

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:183-187`
- **Signature:** `fn ignore(root: &Path) -> Gitignore`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Tests:** `paths_are_relative_and_private_paths_are_dropped`, `commands_are_redacted_and_hide_ignored_paths`, `test_runs_get_an_outcome`
