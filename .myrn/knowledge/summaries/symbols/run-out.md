---
id: run-out
type: summary
status: unverified
title: run::Out
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::run::Out
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::run::Out@f20710ba4ed4134d
map_version: '2'
---
# run::Out

Serializable payload for single-session output, holding the episode, its outcome string, and its event entries. Borrows all fields.

## Behaviour

- outcome is set from ep.outcome() at construction
- events is the entries slice returned by engine.episode_log
- used only by the session-id branch of run, never for the list view

## Terms

session json, single session output, json payload, machine output, episode outcome, event list

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:31-35`
- **Signature:** `struct Out<'a>`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
