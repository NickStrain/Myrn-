---
id: adapter-clip
type: summary
status: unverified
title: adapter::clip
source: myrn summarize
about: sym:crates/myrn-adapters/src/adapter.rs#adapter::clip
anchors:
- sym:crates/myrn-adapters/src/adapter.rs#adapter::clip@096677f5db883d29
---
# adapter::clip

Truncates text to MAX_CONTEXT_CHARS characters (counted as chars, not bytes) and appends "[truncated by myrn]"; the marker counts toward the limit, so the result never exceeds the cap.

## Connections

- **Location:** `crates/myrn-adapters/src/adapter.rs:102-109`
- **Signature:** `pub(crate) fn clip(s: &str) -> String`
- **In:** [[file-crates-myrn-adapters-src-adapter-rs]]
- **Called by:** [[claudecode-format]], [[codex-format]]
