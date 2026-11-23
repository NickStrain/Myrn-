---
id: api-clip-sentences
type: summary
status: unverified
title: api::clip_sentences
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::clip_sentences
anchors:
- sym:crates/myrn-engine/src/api.rs#api::clip_sentences@cc014ac4ea54fb52
map_version: '2'
---
# api::clip_sentences

Collapses whitespace and keeps whole sentences up to about max characters, falling back to clipping the first sentence.

## Behaviour

- Keeps at least the first sentence, clipped to max if it is longer
- Stops before a sentence that would take the text past max

## Terms

summary trimming, truncate text, sentence clip, shorten summary

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:1320-1330`
- **Signature:** `fn clip_sentences(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[engine-brief]]
