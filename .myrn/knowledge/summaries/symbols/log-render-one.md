---
id: log-render-one
type: summary
status: unverified
title: log::render_one
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::render_one
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::render_one@df74bec981e1ebe8
---
# log::render_one

Builds the full text view of one episode: header fields (session, agent, started, ended, task, outcome), the edited, symbols, read, and commits lists, then each event by kind (prompt, tool with command/files/test/error, session_start, session_end), printing unknown kinds by raw name.

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:63-125`
- **Signature:** `fn render_one(ep: &Episode, entries: &[myrn_store::episodes::Entry]) -> String`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Calls:** [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[episodes-list]], [[log-datetime]]
- **Called by:** [[log-run]]
