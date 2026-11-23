---
id: resolver-js-import
type: summary
status: unverified
title: Resolver::js_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::js_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::js_import@3fd1b3f3a6c35140
map_version: '2'
---
# Resolver::js_import

Resolves a relative JavaScript or TypeScript import spec, splitting off a '#member' suffix, to a repository file and optional member name.

## Behaviour

- './store#lookup' from src/session.ts -> src/store/index.ts with member 'lookup'
- 'react' (not starting with '.') -> None
- './x#default' or '#*' -> member None, '*' sets glob true

## Terms

relative import, ES module, index file, member spec, default export, module specifier

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:398-422`
- **Signature:** `fn js_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[symbols-normalize]], [[id-parent-dir]]
- **Called by:** [[resolver-add-import]]
