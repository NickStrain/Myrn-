---
id: episode-clip
type: summary
status: unverified
title: episode::clip
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::clip
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::clip@064b313236cdb58b
map_version: '2'
---
# episode::clip

Truncates a string to at most max characters, counting characters rather than bytes, and appends an ellipsis when it cuts.

## Behaviour

- 'abc' with max 5 -> 'abc' unchanged, since it is within the limit
- 'abcdefgh' with max 5 -> 'abcd…', which is 5 characters including the ellipsis
- non-empty input with max 0 -> '…'

## Terms

truncate, shorten, ellipsis, length limit, cut off, char limit

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:205-212`
- **Signature:** `pub fn clip(s: &str, max: usize) -> String`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Called by:** [[api-clip-sentences]], [[episode-persist]], [[capture-clean]], [[engine-record]], [[symbolcard-render-compact]], [[render-one-line]], [[render-quoted]], [[recall-clip]], [[engine-map-body]], [[summarize-run-model]], [[summarize-parse-json]], [[log-run]], [[log-render-one]], [[trace-run]]
