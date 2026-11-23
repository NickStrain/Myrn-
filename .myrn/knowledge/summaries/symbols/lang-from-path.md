---
id: lang-from-path
type: summary
status: unverified
title: Lang::from_path
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::from_path
anchors:
- sym:crates/myrn-engine/src/indexer/lang/mod.rs#indexer::lang::Lang::from_path@224c42522e409212
map_version: '2'
---
# Lang::from_path

Maps a file path to a Lang using the last extension of its file name. It returns None for unsupported extensions and for .min.js and .d.ts files, which are excluded before the extension check.

## Behaviour

- a/b.tsx -> Some(Tsx)
- x.min.js or y.d.ts -> None
- README.md, or Makefile with no dot -> None

## Terms

extension, file extension, suffix, filename, path to language, language detection, file type

## Connections

- **Location:** `crates/myrn-engine/src/indexer/lang/mod.rs:25-39`
- **Signature:** `pub fn from_path(path: &str) -> Option<Lang>`
- **In:** [[file-crates-myrn-engine-src-indexer-lang-mod-rs]]
- **Called by:** [[git-index-commit]], [[resolver-add-import]], [[resolver-reachable]], [[tests-sym]], [[walk-walk]], [[seed-myrn-index-lang]]
- **Tests:** `detects_languages`
