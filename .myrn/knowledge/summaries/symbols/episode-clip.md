---
id: episode-clip
type: summary
status: unverified
title: episode::clip
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::clip
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::clip@064b313236cdb58b
---
# episode::clip

Returns s unchanged if it has at most max chars; otherwise returns the first max-1 chars followed by an ellipsis. Counts Unicode chars, not bytes, so it is safe on multibyte text.

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:205-212`
- **Signature:** `pub fn clip(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[api-clip-sentences]], [[episode-persist]], [[capture-clean]], [[engine-record]], [[symbolcard-render-compact]], [[render-one-line]], [[render-quoted]], [[recall-clip]], [[engine-map-body]], [[summarize-run-model]], [[summarize-parse-json]], [[log-run]], [[log-render-one]]
