---
id: engine-source-if-short
type: summary
status: unverified
title: Engine::source_if_short
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::source_if_short
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::source_if_short@245c3b2a304bc581
---
# Engine::source_if_short

Returns a symbol's source lines from disk when its span is valid and at most max_lines long, and None otherwise, including when the file cannot be read.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:736-744`
- **Signature:** `fn source_if_short(&self, n: &Node, max_lines: usize) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-root]]
- **Called by:** [[engine-context-excluding]]
