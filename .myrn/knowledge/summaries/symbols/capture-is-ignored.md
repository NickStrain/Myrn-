---
id: capture-is-ignored
type: summary
status: unverified
title: capture::is_ignored
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/mod.rs#capture::is_ignored
anchors:
- sym:crates/myrn-engine/src/capture/mod.rs#capture::is_ignored@ebe8989b9cef594c
map_version: '2'
---
# capture::is_ignored

Returns true when the repo-relative path, or any of its parent directories, matches the gitignore rules.

## Behaviour

- "secrets/prod.env" with rule "secrets/" -> true, because the parent directory matches
- "src/a.rs" with rule "secrets/" -> false
- a negated (!) rule that re-includes the path -> false

## Terms

gitignore match, private path check, ignored file, parent directory rule, excluded path, negated rule

## Connections

- **Location:** `crates/myrn-engine/src/capture/mod.rs:137-139`
- **Signature:** `fn is_ignored(ignore: &Gitignore, rel: &str) -> bool`
- **In:** [[file-crates-myrn-engine-src-capture-mod-rs]]
- **Called by:** [[capture-to-entry]], [[capture-hide-ignored]]
