---
id: runner-item-name
type: summary
status: unverified
title: runner::item_name
source: myrn summarize
about: sym:bench/src/runner.rs#runner::item_name
anchors:
- sym:bench/src/runner.rs#runner::item_name@46f6413da6529f62
map_version: '2'
---
# runner::item_name

Extracts the symbol's last name from a node id by dropping the part before # and any trailing #suffix, then keeping the segment after the last : or .

## Behaviour

- sym:src/a.rs#m::T::refresh -> "refresh"
- sym:a.py#pkg.Session.is_fresh#2 -> "is_fresh"
- file:src/a.rs -> None, because there is no #

## Terms

symbol name, function name, identifier, short name, leaf name, last segment

## Connections

- **Location:** `bench/src/runner.rs:688-692`
- **Signature:** `fn item_name(id: &str) -> Option<&str>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-collect-injections]]
- **Tests:** `item_paths`
