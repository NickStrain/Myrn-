---
id: runner-item-name
type: summary
status: unverified
title: runner::item_name
source: myrn summarize
about: sym:bench/src/runner.rs#runner::item_name
anchors:
- sym:bench/src/runner.rs#runner::item_name@46f6413da6529f62
---
# runner::item_name

Returns the last segment of the symbol qualifier after '#', split on ':' or '.'. For example, sym:src/a.rs#m::T::refresh gives refresh. Returns None when the id has no '#'.

## Connections

- **Location:** `bench/src/runner.rs:533-537`
- **Signature:** `fn item_name(id: &str) -> Option<&str>`
- **In:** [[file-bench-src-runner-rs]]
- **Called by:** [[runner-collect-injections]]
- **Tests:** `item_paths`
