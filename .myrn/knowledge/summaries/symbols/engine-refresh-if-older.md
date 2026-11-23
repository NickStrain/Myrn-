---
id: engine-refresh-if-older
type: summary
status: unverified
title: Engine::refresh_if_older
source: myrn summarize
about: sym:crates/myrn-engine/src/api.rs#api::Engine::refresh_if_older
anchors:
- sym:crates/myrn-engine/src/api.rs#api::Engine::refresh_if_older@98f6d24490704084
map_version: '2'
---
# Engine::refresh_if_older

Refreshes only if the last refresh in this engine was at least secs seconds ago.

## Behaviour

- No refresh yet in this engine -> refreshes
- Refreshed within secs -> returns Ok(None) and does no work

## Terms

throttle, cache refresh, long-lived server, refresh interval

## Connections

- **Location:** `crates/myrn-engine/src/api.rs:319-327`
- **Signature:** `pub fn refresh_if_older(&mut self, secs: u64) -> Result<Option<IndexReport>>`
- **In:** [[file-crates-myrn-engine-src-api-rs]]
- **Calls:** [[engine-refresh]]
- **Called by:** [[tools-call]], [[ui-serve]]
