---
id: log-short-type
type: summary
status: unverified
title: log::short_type
source: myrn summarize
about: sym:crates/myrn/src/cmd/log.rs#cmd::log::short_type
anchors:
- sym:crates/myrn/src/cmd/log.rs#cmd::log::short_type@9baf3cd0bdafd60e
map_version: '2'
---
# log::short_type

Maps a stored record type to a short label for the log table's type column. Unknown types pass through unchanged.

## Behaviour

- 'root_cause' -> 'cause'; 'decision' -> 'decide'; 'workflow' -> 'workflw' (a literal misspelling, 7 chars for fixed width)
- 'summary' -> 'summary'; 'bug' -> 'bug' (unknown types unchanged)
- 'root-cause' (hyphen) is not mapped here and passes through; run converts it to 'root_cause' before storage lookup

## Terms

abbreviation, column label, short type name, type label, display width, type shortening

## Connections

- **Location:** `crates/myrn/src/cmd/log.rs:133-141`
- **Signature:** `fn short_type(t: &str) -> &str`
- **In:** [[file-crates-myrn-src-cmd-log-rs]]
- **Called by:** [[log-run]]
