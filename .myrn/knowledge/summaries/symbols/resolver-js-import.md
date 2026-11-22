---
id: resolver-js-import
type: summary
status: unverified
title: Resolver::js_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::js_import
anchors:
- sym:crates/myrn-engine/src/indexer/symbols.rs#indexer::symbols::Resolver::js_import@3fd1b3f3a6c35140
---
# Resolver::js_import

Resolves a relative JS or TS specifier like `./x#member` against the importing file's directory, trying the TS, JS and index-file extensions. `*` and `default` yield no member name, and non-relative specifiers return None.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/symbols.rs:398-422`
- **Signature:** `fn js_import(&self, file: &str, spec: &str) -> Option<(String, Option<String>, bool)>`
- **In:** [[file-crates-myrn-engine-src-indexer-symbols-rs]]
- **Calls:** [[symbols-normalize]], [[id-parent-dir]]
- **Called by:** [[resolver-add-import]]
