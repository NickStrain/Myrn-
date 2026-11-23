---
id: file-crates-myrn-src-cmd-ui-rs
type: summary
status: unverified
title: crates/myrn/src/cmd/ui.rs
source: myrn summarize
about: file:crates/myrn/src/cmd/ui.rs
anchors:
- file:crates/myrn/src/cmd/ui.rs@638e042dcc5d1f33
---
# crates/myrn/src/cmd/ui.rs

Local, read-only web UI for the project store: binds 127.0.0.1, requires a per-run token and loopback Host header, and serves an embedded HTML page plus JSON API routes. Main parts are run (bind and accept loop), read_request and Request (HTTP parsing), serve (auth, routing, engine calls), respond (HTTP writing), Hit (search row type), and the helpers decode, token, json and open_browser.

## Connections

- **Location:** `crates/myrn/src/cmd/ui.rs`
- **In:** [[dir-crates-myrn-src-cmd]]
- **Defines:** [[ui-run]], [[ui-request]], [[ui-read-request]], [[ui-serve]], [[serve-hit]], [[ui-json]], [[ui-respond]], [[ui-decode]], [[ui-token]], [[ui-open-browser]]
- **Imports:** [[file-crates-myrn-engine-src-api-rs]], [[file-crates-myrn-src-cmd-mod-rs]]
- **Imported by:** [[file-crates-myrn-src-cmd-mod-rs]]
