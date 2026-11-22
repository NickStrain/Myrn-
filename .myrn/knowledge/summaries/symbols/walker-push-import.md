---
id: walker-push-import
type: summary
status: unverified
title: Walker::push_import
source: myrn summarize
about: sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_import
anchors:
- sym:crates/myrn-engine/src/indexer/parse.rs#indexer::parse::Walker::push_import@89a59c604180d8d4
---
# Walker::push_import

Stamps the current Rust module path onto an Import and appends it to the output.

## Connections

- **Location:** `crates/myrn-engine/src/indexer/parse.rs:204-207`
- **Signature:** `fn push_import(&mut self, mut imp: Import)`
- **In:** [[file-crates-myrn-engine-src-indexer-parse-rs]]
- **Calls:** [[walker-rust-mod-path]]
- **Called by:** [[walker-classify-rust]], [[walker-classify-python]], [[walker-classify-js]], [[walker-js-import]], [[walker-js-require]]
