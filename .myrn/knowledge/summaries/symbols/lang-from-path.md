---
id: lang-from-path
type: summary
status: unverified
title: Lang::from_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::from_path
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::from_path@224c42522e409212
---
# Lang::from_path

Takes a file path string and returns the Lang for its extension (rs, py/pyi, js/mjs/cjs/jsx, ts/mts/cts, tsx), or None for unknown extensions, no extension, or .min.js and .d.ts files. Only the last path segment is inspected; no I/O.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:25-39`
- **Signature:** `pub fn from_path(path: &str) -> Option<Lang>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[git-index-commit]], [[resolver-add-import]], [[resolver-reachable]], [[tests-sym]], [[walk-walk]], [[seed-myrn-index-lang]]
- **Tests:** `detects_languages`
