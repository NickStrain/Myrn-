---
id: adapter-clip
type: summary
status: unverified
title: adapter::clip
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::clip
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::clip@096677f5db883d29
map_version: '2'
---
# adapter::clip

Shortens hook context to at most MAX_CONTEXT_CHARS (9000) characters and marks the cut.

## Behaviour

- 9000 characters or fewer -> returned unchanged
- Longer -> first 8980 characters, then a newline and [truncated by myrn], so output is exactly 9000 characters
- Counts characters, not bytes, so multi-byte text is not split mid-character

## Terms

truncate, cap, context limit, shorten, token limit

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:102-109`
- **Signature:** `pub(crate) fn clip(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-format]], [[codex-format]]
