---
id: log-render-one
type: summary
status: unverified
title: log::render_one
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::render_one
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::render_one@df74bec981e1ebe8
map_version: '2'
---
# log::render_one

Builds the full text view of one session: header, task, outcome, file and symbol lists, and a dated event timeline. Returns the report as a String.

## Behaviour

- ended line is printed only when ep.ended is Some; task line only when ep.task is Some, clipped to 100 chars
- a list section (edited, symbols, read, commits) is omitted when empty; the first item sits on the label line and the rest are indented 10 spaces
- tool event prints tool name, outcome, '$ command' (clipped to 70), comma-joined files, and [test ...]; an error prints on an indented line below, clipped to 90

## Terms

session detail, full session view, episode report, event timeline, transcript, session summary text

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:143-205`
- **Signature:** `fn render_one(ep: &Episode, entries: &[myrn_store::episodes::Entry]) -> String`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Calls:** [[episode-outcome]], [[episode-summary]], [[episode-clip]], [[episodes-list]], [[log-datetime]]
- **Called by:** [[log-run]]
