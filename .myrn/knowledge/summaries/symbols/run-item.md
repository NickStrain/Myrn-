---
id: run-item
type: summary
status: unverified
title: run::Item
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::run::Item
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::run::Item@67f9969c95c287b1
map_version: '2'
---
# run::Item

Serializable union of a session or a record for the list view, tagged so each entry carries a kind field.

## Behaviour

- Session(&Episode) serializes with kind 'session'
- Record(&RecordRow) serializes with kind 'record'
- run sorts items by timestamp descending, then truncates to limit

## Terms

list entry, tagged union, session or record, kind tag, json kind, mixed list

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:71-74`
- **Signature:** `enum Item<'a>`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
