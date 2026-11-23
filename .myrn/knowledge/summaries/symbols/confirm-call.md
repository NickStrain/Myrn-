---
id: confirm-call
type: summary
status: unverified
title: confirm::call
source: myrn summarize
about: sym:crates/myrn-mcp/src/tools/confirm.rs#tools::confirm::call
anchors:
- sym:crates/myrn-mcp/src/tools/confirm.rs#tools::confirm::call@98075259f83b23d8
map_version: '2'
---
# confirm::call

Reads the 'id' argument, calls engine.confirm on it, and returns a one-line summary of the record id, the number of anchors refreshed, and the new status. The engine is mutated, so the record's anchors move to the current code.

## Behaviour

- args with id 'bug-7f3a91' and anchors_refreshed 1 -> '<id> confirmed for the current code (1 anchor refreshed); status: <status>.'
- anchors_refreshed 0 or 2 or more -> plural 'anchors' is used, for example '(0 anchors refreshed)'
- missing or non-string 'id' -> error from str_arg propagates; engine.confirm is not called

## Terms

confirm record, mark current, refresh anchors, stale record, anchor count, status message, revalidate record

## Connections

- **Location:** `crates/myrn-mcp/src/tools/confirm.rs:21-31`
- **Signature:** `pub fn call(engine: &mut Engine, args: &Value) -> Result<String>`
- **In:** [[file-crates-myrn-mcp-src-tools-confirm-rs]]
- **Calls:** [[tools-str-arg]]
- **Called by:** [[tools-call]]
