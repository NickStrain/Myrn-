---
id: tests-e
type: summary
status: unverified
title: tests::e
source: myrn summarize
about: sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::tests::e
anchors:
- sym:crates/myrn-engine/src/capture/episode.rs#capture::episode::tests::e@9791d66daf425360
map_version: '2'
---
# tests::e

Test helper that builds a minimal Entry with the given time, agent 'claude' and event name, leaving all other fields at their defaults.

## Behaviour

- e(2, 'prompt') -> Entry with agent 'claude', event 'prompt', and no text, files, tool or test fields
- the returned Entry has no outcome, so a tool entry built from it counts its files as read, not edited
- callers set fields on the returned Entry before passing it to fold

## Terms

test fixture, test entry, helper, log entry builder, mock event

## Connections

- **Location:** `crates/myrn-engine/src/capture/episode.rs:218-225`
- **Signature:** `fn e(t: i64, event: &str) -> Entry`
- **In:** [[file-crates-myrn-engine-src-capture-episode-rs]]
- **Tests:** `folds_a_session`
