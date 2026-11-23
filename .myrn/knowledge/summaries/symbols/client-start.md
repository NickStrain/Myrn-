---
id: client-start
type: summary
status: unverified
title: client::start
source: myrn summarize
about: sym:crates/myrn-embed/src/client.rs#client::start
anchors:
- sym:crates/myrn-embed/src/client.rs#client::start@e10f72f85c50d4e0
map_version: '2'
---
# client::start

Spawns the helper binary detached with stdio nulled, then polls it for up to 3 seconds until it answers.

## Behaviour

- spawn error -> false immediately
- helper answers the probe request within 3 s -> true
- no answer within 3 s -> false, but the spawned child is not killed

## Terms

spawn helper, launch daemon, background process, wait for helper, start server, readiness probe

## Connections

- **Location:** `crates/myrn-embed/src/client.rs:62-82`
- **Signature:** `pub fn start(data_home: &Path, exe: &Path) -> bool`
- **In:** [[file-crates-myrn-embed-src-client-rs]]
- **Calls:** [[client-request]]
- **Called by:** [[client-embed]]
