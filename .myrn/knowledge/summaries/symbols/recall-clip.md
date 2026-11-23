---
id: recall-clip
type: summary
status: unverified
title: recall::clip
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::clip
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::clip@8d7205171cf9c1b5
map_version: '2'
---
# recall::clip

Trims surrounding whitespace from s and passes it to capture::episode::clip with the max length.

## Behaviour

- leading and trailing whitespace removed before clipping
- the truncation rule itself is in capture::episode::clip, which is not shown here

## Terms

truncate, trim, shorten, length limit, snippet, max chars

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:156-158`
- **Signature:** `fn clip(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[engine-recall-text]], [[recall-first-paragraph]]
