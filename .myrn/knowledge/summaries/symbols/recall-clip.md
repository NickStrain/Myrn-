---
id: recall-clip
type: summary
status: unverified
title: recall::clip
source: myrn summarize
about: sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::clip
anchors:
- sym:crates/myrn-engine/src/retrieve/recall.rs#retrieve::recall::clip@8d7205171cf9c1b5
---
# recall::clip

Trims the input and truncates it to max characters via the episode clip helper.

## Connections

- **Location:** `crates/myrn-engine/src/retrieve/recall.rs:153-155`
- **Signature:** `fn clip(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-retrieve-recall-rs]]
- **Calls:** [[episode-clip]]
- **Called by:** [[engine-recall-text]], [[recall-first-paragraph]]
