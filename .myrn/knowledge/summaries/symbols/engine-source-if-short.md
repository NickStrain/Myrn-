---
id: engine-source-if-short
type: summary
status: unverified
title: Engine::source_if_short
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::source_if_short
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::source_if_short@245c3b2a304bc581
map_version: '2'
---
# Engine::source_if_short

Reads a symbol's current source lines from disk, if the symbol is short enough to inject whole.

## Behaviour

- Returns None when the span is longer than max_lines, starts at line 0, or ends before it starts
- Returns None when the file cannot be read or the stored path is missing

## Terms

inline code, source snippet, function body, current source

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:802-810`
- **Signature:** `fn source_if_short(&self, n: &Node, max_lines: usize) -> Option<String>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-root]]
- **Called by:** [[engine-context-excluding]]
