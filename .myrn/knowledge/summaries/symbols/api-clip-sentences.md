---
id: api-clip-sentences
type: summary
status: unverified
title: api::clip_sentences
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::clip_sentences
anchors:
- sym:crates/myrn-engine/src/api.rs#api::clip_sentences@cc014ac4ea54fb52
---
# api::clip_sentences

Collapses whitespace and keeps whole sentences up to about max characters, always keeping at least the first sentence, clipped to max.

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1248-1258`
- **Signature:** `fn clip_sentences(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[engine-brief]]
